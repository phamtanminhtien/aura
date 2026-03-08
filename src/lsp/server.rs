use crate::compiler::frontend::lexer::Lexer;
use crate::compiler::frontend::parser::Parser;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Aura Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
            language_id: "aura".to_string(), // Default value
        })
        .await;
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    async fn goto_definition(
        &self,
        _: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let source = &params.text;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex_all();

        let mut parser = Parser::new(tokens);
        let _program = parser.parse_program();

        let mut diagnostics = Vec::new();

        // Collect Lexer errors
        for diag in &lexer.diagnostics.diagnostics {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(diag.line as u32 - 1, diag.column as u32 - 1),
                    end: Position::new(diag.line as u32 - 1, diag.column as u32),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: diag.message.clone(),
                ..Default::default()
            });
        }

        // Collect Parser errors
        for diag in &parser.diagnostics.diagnostics {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(diag.line as u32 - 1, diag.column as u32 - 1),
                    end: Position::new(diag.line as u32 - 1, diag.column as u32),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: diag.message.clone(),
                ..Default::default()
            });
        }

        // Semantic Analysis (Placeholder, actual implementation might need more context)
        // let mut analyzer = SemanticAnalyzer::new();
        // if let Err(e) = analyzer.analyze(program) { ... }

        self.client
            .publish_diagnostics(params.uri, diagnostics, Some(params.version))
            .await;
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
