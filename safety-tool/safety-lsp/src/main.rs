use ropey::Rope;
use std::sync::Mutex;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Tree};

struct Backend {
    client: Client,
    rust: Mutex<Rust>,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: Some(true),
                        will_save_wait_until: Some(false),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "[initialized] server initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        let len = text.len();
        let (tree, attrs) = {
            let mut lock = self.rust.lock().unwrap();
            let tree = lock.update_node_tree(text);
            let attrs = lock.find_attrs();
            (tree, attrs)
        };
        self.client
            .log_message(MessageType::INFO, format!("[did_open] document byte len={len}\t{tree}"))
            .await;
        self.client.log_message(MessageType::INFO, format!("[did_open] attrs = {attrs:?}")).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let text = params.content_changes.iter().map(|c| &*c.text).collect::<Vec<_>>().join("");
        let len = text.len();
        let (tree, attrs) = {
            let mut lock = self.rust.lock().unwrap();
            let tree = lock.update_node_tree(text);
            let attrs = lock.find_attrs();
            (tree, attrs)
        };
        self.client
            .log_message(MessageType::INFO, format!("[did_change] document byte len={len}\t{tree}"))
            .await;
        self.client.log_message(MessageType::INFO, format!("[did_open] attrs = {attrs:?}")).await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client.log_message(MessageType::INFO, "[completion] trigger completion").await;
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let pos = params.text_document_position_params.position;
        let attr = self.rust.lock().unwrap().get_attr(pos);
        let pos_end = {
            let mut pos = pos;
            pos.character += 1;
            pos
        };
        let range = Range { start: pos, end: pos_end };
        let text = format!(
            "# You're hovering!

```rust
attr = {attr:#?}
```

```rust
range = {range:?}
```

```rust
params = {params:#?}
```"
        );
        Ok(Some(Hover {
            // render markdown string
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: text,
            }),
            // possibly used to highlight text document in this range (no effect for neovim)
            range: Some(range),
        }))
    }
}

impl Backend {
    fn new(client: Client) -> Self {
        Backend { client, rust: Mutex::new(Rust::new()) }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

type ByteRange = std::ops::Range<usize>;

#[derive(Debug)]
struct Attr {
    byte_range: ByteRange,
    start_pos: Position,
    end_pos: Position,
}

struct Rust {
    parser: Parser,
    /// Byte range for attributes for the text.
    attrs: Vec<Attr>,
    /// Source code as a rust file.
    text: String,
    /// Text rope.
    rope: Rope,
    tree: Option<Tree>,
}

impl Rust {
    fn new() -> Self {
        Rust {
            parser: init_tree_sitter(),
            attrs: Vec::new(),
            text: String::new(),
            rope: Rope::new(),
            tree: None,
        }
    }

    fn update_node_tree(&mut self, text: String) -> String {
        self.tree = self.parser.parse(&text, None);
        self.rope = Rope::from_str(&text);
        self.text = text;
        let tree = self.tree.as_ref().unwrap();
        format!("text={:?}\ntree={tree:?}\nroot_node={}", self.text, tree.root_node())
    }

    fn find_attrs(&mut self) -> Vec<String> {
        self.attrs.clear();
        if let Some(tree) = &self.tree {
            let mut cursor = tree.walk();
            if cursor.node().grammar_name() == "source_file" {
                cursor.goto_descendant(1);
                let mut v = Vec::new();
                let mut push = |node: tree_sitter::Node| {
                    if node.grammar_name() == "attribute_item" {
                        let range = node.byte_range();
                        let attr = Attr {
                            byte_range: range.clone(),
                            start_pos: byte_to_pos(range.start, &self.rope),
                            end_pos: byte_to_pos(range.end, &self.rope),
                        };
                        let src = &self.text[range];
                        v.push(format!("src={src:?}\tattr={attr:?}"));
                        self.attrs.push(attr);
                    }
                };
                push(cursor.node());
                while cursor.goto_next_sibling() {
                    push(cursor.node());
                }
                return v;
            }
        }
        Vec::new()
    }

    /// Returns the attr if the cursor is in its pos scope.
    fn get_attr(&self, pos: Position) -> Option<String> {
        for attr in &self.attrs {
            if pos >= attr.start_pos && pos <= attr.end_pos {
                return Some(self.text[attr.byte_range.clone()].to_owned());
            }
        }
        None
    }
}

fn init_tree_sitter() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");
    parser
}

fn byte_to_pos(byte: usize, rope: &Rope) -> Position {
    let line = rope.byte_to_line(byte);
    let character = byte - rope.line_to_byte(line);
    Position { line: line as u32, character: character as u32 }
}
