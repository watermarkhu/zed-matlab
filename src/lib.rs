use zed_extension_api::{self as zed, Result};

struct MatlabExtension {
    did_find_server: bool,
}

impl MatlabExtension {
    const LANGUAGE_SERVER_ID: &'static str = "matlab-language-server";
    const SERVER_PATH: &'static str = "node_modules/.bin/matlab-language-server";
    const PACKAGE_NAME: &'static str = "@mathworks/matlab-language-server";
}

impl zed::Extension for MatlabExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != Self::LANGUAGE_SERVER_ID {
            return Err(format!("unknown language server: {language_server_id}").into());
        }

        let server_path = worktree.which(Self::SERVER_PATH);

        if self.did_find_server {
            if let Some(path) = server_path {
                return Ok(zed::Command {
                    command: path,
                    args: vec!["--stdio".to_string()],
                    env: Default::default(),
                });
            }
        }

        if let Some(path) = server_path {
            self.did_find_server = true;
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(Self::PACKAGE_NAME)?;

        if !zed::npm_package_installed_version(Self::PACKAGE_NAME)?
            .as_ref()
            .map_or(false, |installed_version| installed_version == &version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(Self::PACKAGE_NAME, &version)?;
        }

        self.did_find_server = true;

        Ok(zed::Command {
            command: worktree
                .which(Self::SERVER_PATH)
                .ok_or_else(|| format!("language server binary not found"))?,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MatlabExtension);
