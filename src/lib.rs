use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

struct MatlabExtension;

impl zed::Extension for MatlabExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Check if matlab_ls is available in PATH
        if let Some(path) = worktree.which("matlab_ls") {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        Err(format!(
            "matlab_ls not found in PATH. Please install the MATLAB language server. See https://github.com/watermarkhu/zed-matlab for installation instructions."
        )
        .into())
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("matlab-language-server", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "MATLAB": settings
        })))
    }
}

zed::register_extension!(MatlabExtension);
