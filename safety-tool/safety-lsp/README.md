# safety-lsp

## Configuration for Neovim

FIXME: replace lsp name and filetypes for real .rs files when implementation is done.

```lua
vim.lsp.config["demo"] = {
  -- Command and arguments to start the server.
  cmd = { "/home/gh-zjp-CN/tag-std/safety-tool/safety-lsp/target/debug/safety-lsp" },
  -- Filetypes to automatically attach to.

  filetypes = { "demo" },
  -- Sets the "workspace" to the directory where any of these files is found.

  -- Files that share a root directory will reuse the LSP server connection.
  -- Nested lists indicate equal priority, see |vim.lsp.Config|.
  root_markers = { { ".demo" }, ".git" },

  -- Specific settings to send to the server. The schema is server-defined.
  settings = {},
}

-- Auto set up nrs filetype because neovim doesn't do it for us.
vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, {
  pattern = { "*.demo" },
  command = "set filetype=demo",
})

-- Make LSP server config into effects.
vim.lsp.enable("demo")
```
