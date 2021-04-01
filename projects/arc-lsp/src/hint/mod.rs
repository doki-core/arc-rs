use serde_json::Value;
use tower_lsp::lsp_types::*;

mod document_symbol;
pub use document_symbol::document_symbol_provider;

pub fn code_action_provider(p: CodeActionParams) -> Option<CodeActionResponse> {
    let _ = p;
    let mut actions = vec![];
    actions.extend(extract_actions());
    return Some(actions);
}

pub fn hover_provider(p: HoverParams) -> Option<Hover> {
    let _ = p;
    // Some(Hover {
    //     contents: HoverContents::Markup(MarkupContent {
    //         kind: MarkupKind::Markdown,
    //         value: "![](https://projecteuler.net/images/icons/info.png)".to_string(),
    //     }),
    //     range: None,
    // })
    return None
}

pub fn code_lens_provider(p: CodeLensParams) -> Option<Vec<CodeLens>> {
    let _ = p;
    let len = CodeLens {
        range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 1, character: 1 } },
        command: None,
        data: Some(Value::String("lens".to_string())),
    };
    Some(vec![len])
}

fn extract_actions() -> Vec<CodeActionOrCommand> {
    vec![
        cmd("🗝 Extract key path as cite", "arc.extract.key", vec![Value::Bool(true)]),
        cmd("📮 Extract value as json", "arc.extract.value", vec![Value::Bool(true)]),
    ]
}

fn cmd(show: &str, run: &str, arg: Vec<Value>) -> CodeActionOrCommand {
    CodeActionOrCommand::Command(Command { title: String::from(show), command: String::from(run), arguments: Some(arg) })
}

#[allow(dead_code)]
fn action() -> CodeAction {
    CodeAction {
        title: "Save image to local".to_string(),
        kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
        diagnostics: None,
        edit: None,
        command: None,
        is_preferred: Some(true),
    }
}
