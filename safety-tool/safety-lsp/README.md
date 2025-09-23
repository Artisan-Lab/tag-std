# safety-lsp

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
