# Safety-tool LSP Client

## What's this?

This is [safety-tool]'s LSP client for vscode to support features like auto-completion and hover-doc.

Its LSP [server] should be separately installed on your own to be available.

[safety-tool]: https://github.com/Artisan-Lab/tag-std
[server]: https://github.com/Artisan-Lab/tag-std/tree/main/safety-tool/safety-lsp

![](https://github.com/user-attachments/assets/5c530183-ee86-4c48-aba9-b725c1c257b5)

![](https://github.com/user-attachments/assets/593b7cd3-3584-41c4-8980-abd3de180f3b)

## Development

### Debug

- Run `npm install` in this folder. This installs all necessary npm modules in both the client and server folder
- Open VS Code on this folder.
- Press Ctrl+Shift+B to start compiling the client and server in [watch mode](https://code.visualstudio.com/docs/editor/tasks#:~:text=The%20first%20entry%20executes,the%20HelloWorld.js%20file.).
- Switch to the Run and Debug View in the Sidebar (Ctrl+Shift+D).
- Select `Launch Client` from the drop down (if it is not already).
- Press ▷ to run the launch config (F5).

### Publish

Run `vsce publish` to publish this plugin to vscode marketplace.

But need these preparations once:

```bash
# Install vsce
npm install -g @vscode/vsce

# Login
# 1. Create Publisher in https://marketplace.visualstudio.com/manage
# 2. Get Personal Access Tokens (PAT) from https://dev.azure.com by enabling Marketplace (Manage) permission
vsce login <your-publisher-name>
# Paste PAT for the publisher

# Generate local .vsix
# vsce package
```
