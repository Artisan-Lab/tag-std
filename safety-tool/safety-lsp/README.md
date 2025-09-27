# safety-lsp

![](https://github.com/user-attachments/assets/5c530183-ee86-4c48-aba9-b725c1c257b5)

![](https://github.com/user-attachments/assets/593b7cd3-3584-41c4-8980-abd3de180f3b)

## Configuration for VSCode

1. For `.vscode/settings.json`, `SP_FILE` starts from workspace root if it's a relative path:

```json
{
  "safety-tool.env": {
    "SP_FILE": "./ostd/safety-tags.toml"
  }
}
```

`SP_FILE` can also be a absolute path.

`SP_DIR` is also supported, but only one of them should be specified.

2. For remote usage, `safety-lsp.env.SAFETY_LSP` should be set as vscode will fail to find
  `safety-lsp` due to absence of bash env loading.

```json
{
  "safety-tool.env": {
    "SAFETY_LSP": "/path/to/safety-lsp"
  }
}
```

3. `Ctrl+Space` to open hover doc panel of each completion candidate if the doc is not shown.

## Configuration for Neovim

```lua
vim.lsp.config["safety-lsp"] = {
  -- Command and arguments to start the server.
  cmd = { "/home/gh-zjp-CN/tag-std/safety-tool/safety-lsp/target/debug/safety-lsp" },
  -- Environment variables passed to the LSP process on spawn
  cmd_env = { SP_DISABLE_CHECK = 1 },

  -- Filetypes to automatically attach to.
  filetypes = { "rust" },

  -- Sets the "workspace" to the directory where any of these files is found.
  -- Files that share a root directory will reuse the LSP server connection.
  -- Nested lists indicate equal priority, see |vim.lsp.Config|.
  root_markers = { { "Cargo.toml" }, ".git" },

  -- Specific settings to send to the server. The schema is server-defined.
  settings = {},
}
-- Make LSP server config into effects.
vim.lsp.enable("safety-lsp")
```
