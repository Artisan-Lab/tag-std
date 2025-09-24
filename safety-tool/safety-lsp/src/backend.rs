use crate::rust::Rust;
use std::sync::Mutex;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
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
        let message = "[initialized] safety-tool server initialized!";
        self.client.log_message(MessageType::INFO, message).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        self.update_document(text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let text = params.content_changes.iter().map(|c| &*c.text).collect::<Vec<_>>().join("");
        self.update_document(text);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let pos = params.text_document_position.position;
        self.with_rust(|r| {
            if r.get_attr_range(pos).is_none() {
                // The cursor is not in an attribute, thus no completion.
                return Ok(None);
            }

            let response = r.for_each_tag(|tag| CompletionItem {
                label: tag.name.to_owned(),
                label_details: Some(CompletionItemLabelDetails {
                    detail: None,
                    // inline desc left-aligned
                    description: Some("(safety tag)".to_owned()),
                }),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(tag.hover_detail()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: tag.hover_documentation(),
                })),
                text_edit: Some(
                    TextEdit::new(
                        Range {
                            start: Position {
                                line: pos.line,
                                character: pos.character.saturating_sub(1),
                            },
                            end: pos,
                        },
                        tag.hover_detail(),
                    )
                    .into(),
                ),
                ..Default::default()
            });

            Ok(Some(CompletionResponse::Array(response)))
        })
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let pos = params.text_document_position_params.position;

        let attr = self.with_rust(|r| r.get_attr_str(pos));
        let safety_attr = safety_parser::safety::parse_attr_and_get_properties(
            attr.as_deref().unwrap_or_default(),
        );

        let mut safety_doc =
            safety_attr.iter().map(|attr| attr.gen_hover_doc()).collect::<Vec<_>>().join("\n");
        let tag_count = safety_attr.iter().map(|attr| attr.tags.len()).sum::<usize>();
        match tag_count {
            0 => (),
            1 => safety_doc.insert_str(0, "# Safety Requirement\n\n"),
            _ => safety_doc.insert_str(0, "# Safety Requirements\n\n"),
        }

        let pos_end = {
            let mut pos = pos;
            pos.character += 1;
            pos
        };
        let range = Range { start: pos, end: pos_end };
        Ok(Some(Hover {
            // render markdown string
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: safety_doc,
            }),
            // possibly used to highlight text document in this range (no effect for neovim)
            range: Some(range),
        }))
    }
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Backend { client, rust: Mutex::new(Rust::new()) }
    }

    fn with_rust<T>(&self, f: impl FnOnce(&mut Rust) -> T) -> T {
        f(&mut *self.rust.lock().unwrap())
    }

    fn update_document(&self, text: String) {
        self.with_rust(|r| {
            _ = r.update_node_tree(text);
            _ = r.find_attrs();
        });
    }
}
