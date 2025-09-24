# Safety-tool LSP Client

## Debug

- Run `npm install` in this folder. This installs all necessary npm modules in both the client and server folder
- Open VS Code on this folder.
- Press Ctrl+Shift+B to start compiling the client and server in [watch mode](https://code.visualstudio.com/docs/editor/tasks#:~:text=The%20first%20entry%20executes,the%20HelloWorld.js%20file.).
- Switch to the Run and Debug View in the Sidebar (Ctrl+Shift+D).
- Select `Launch Client` from the drop down (if it is not already).
- Press ▷ to run the launch config (F5).

## Publish

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
