//! This is a language server for safety-tool or safety tags.

pub mod backend;
pub mod rust;

#[tokio::main]
async fn main() {
    use tower_lsp_server::{LspService, Server};

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(backend::Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
