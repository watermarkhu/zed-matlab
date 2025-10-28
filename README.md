# Zed MATLAB Extension

A MATLAB extension for [Zed](https://zed.dev) that provides syntax highlighting and language server support.

## Features

- **Syntax Highlighting**: Full MATLAB syntax highlighting using [tree-sitter-matlab](https://github.com/acristoffers/tree-sitter-matlab)
- **Language Server Protocol (LSP)**: Code completion, diagnostics, hover information, and more via the [MATLAB Language Server](https://github.com/mathworks/MATLAB-language-server)

## Installation

The extension can be installed from the Zed extension marketplace. The syntax highlighting will work out of the box. To enable language server features, follow the setup instructions below.

## Language Server Setup

To enable LSP features like code completion and diagnostics, you need to install the MATLAB Language Server and make it available in your PATH.

### Prerequisites

- Node.js (required to run the language server)
- MATLAB R2021b or later (required for the language server to function)

### Installation Steps

1. **Clone the MATLAB Language Server repository:**
   ```bash
   git clone https://github.com/mathworks/MATLAB-language-server ~/.local/share/MATLAB-language-server
   ```
   
   On Windows, use a suitable location like:
   ```powershell
   git clone https://github.com/mathworks/MATLAB-language-server $env:USERPROFILE\.local\share\MATLAB-language-server
   ```

2. **Build the language server:**
   ```bash
   cd ~/.local/share/MATLAB-language-server
   npm install
   npm run compile
   npm run package
   ```

3. **Create a wrapper script named `matlab_ls`:**

   **On Linux/macOS:** Create `~/.local/bin/matlab_ls` (or another directory in your PATH):
   ```bash
   #!/usr/bin/env bash
   exec node "$HOME/.local/share/MATLAB-language-server/out/index.js" "$@"
   ```
   
   Make it executable:
   ```bash
   chmod +x ~/.local/bin/matlab_ls
   ```

   **On Windows:** Create `matlab_ls.bat` in a directory in your PATH (e.g., `C:\Users\<YourUsername>\.local\bin`):
   ```batch
   @echo off
   node "%USERPROFILE%\.local\share\MATLAB-language-server\out\index.js" %*
   ```

4. **Verify the installation:**
   ```bash
   matlab_ls --version
   ```

### Configuration

You can configure the language server by adding settings to your Zed `settings.json`:

```json
{
  "lsp": {
    "matlab-language-server": {
      "settings": {
        "installPath": "/usr/local/MATLAB/R2024a",
        "indexWorkspace": true,
        "matlabConnectionTiming": "onStart",
        "telemetry": false
      }
    }
  }
}
```

#### Configuration Options

- **`installPath`** (string): Path to your MATLAB installation directory (e.g., `/usr/local/MATLAB/R2024a` on Linux/macOS, or `C:\Program Files\MATLAB\R2024a` on Windows)
- **`indexWorkspace`** (boolean): Whether to index the entire workspace for better code intelligence. Default: `false`
- **`matlabConnectionTiming`** (string): When to connect to MATLAB. Options: `"onStart"`, `"onDemand"`, `"never"`. Default: `"onDemand"`
- **`telemetry`** (boolean): Whether to send telemetry data. Default: `true`

## Troubleshooting

### Language server not working

If the language server isn't working:

1. Check that `matlab_ls` is in your PATH:
   ```bash
   which matlab_ls  # Linux/macOS
   where matlab_ls  # Windows
   ```

2. Verify Node.js is installed:
   ```bash
   node --version
   ```

3. Check Zed's log output for error messages (open with `Cmd+Shift+P` / `Ctrl+Shift+P` and search for "Open Log")

### MATLAB installation not found

Make sure the `installPath` in your settings points to your actual MATLAB installation. The language server needs to locate MATLAB to provide full functionality.

## Development

To develop this extension:

1. Clone this repository
2. Follow the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) guide from Zed docs
3. Build the extension: `cargo build --release`
4. Install locally for testing

## Credits

- MATLAB Language Server by [MathWorks](https://github.com/mathworks/MATLAB-language-server)
- Tree-sitter MATLAB grammar by [acristoffers](https://github.com/acristoffers/tree-sitter-matlab)

## License

See [LICENSE](LICENSE) file.
