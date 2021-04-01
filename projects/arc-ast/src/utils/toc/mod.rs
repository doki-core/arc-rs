use crate::{Range, AST, ParserConfig};
use crate::ast::ASTKind;
use lsp_types::{DocumentSymbol, SymbolKind};

#[derive(Debug)]
pub struct TOC {
    level: usize,
    name: String,
    kind: SymbolKind,
    range: Range,
    children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self {
            level: 0,
            name: String::from("ROOT"),
            kind: SymbolKind::File,
            range: Default::default(),
            children: vec![],
        }
    }
}


impl From<TOC> for DocumentSymbol {
    fn from(toc: TOC) -> Self {
        let details = format!("H{}", toc.children.len());
        let children = match toc.children.len() {
            0 => None,
            _ => Some(toc.children.into_iter().map(From::from).collect()),
        };
        #[allow(deprecated)]
        DocumentSymbol {
            name: toc.name,
            detail: Some(details),
            kind: toc.kind,
            deprecated: None,
            range: toc.range,
            selection_range: toc.range,
            children,
        }
    }
}


impl AST {
    /// Table of contents
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut root = TOC::default();
        match &self.kind {
            ASTKind::Program(terms) | ASTKind::Sequence(terms) => {
                for term in terms {
                    match &term.kind {
                        ASTKind::DictScope(level, children) => {
                            if 1 + level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(1 + level - 1);
                            let new = TOC {
                                level: 1 + level,
                                kind: SymbolKind::Class,
                                name: children.to_string(),
                                range: term.range,
                                children: vec![],
                            };
                            parent.children.push(new);
                        }
                        ASTKind::ListScope(level, children) => {
                            if 1 + level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(1 + level - 1);
                            let new = TOC {
                                level: 1 + level,
                                kind: SymbolKind::Variable,
                                name: children.to_string(),
                                range: term.range,
                                children: vec![],
                            };
                            parent.children.push(new);
                        }
                        _ => (),
                    }
                }
            }
            _ => {}
        };
        return root;
    }
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}


// pub fn join_ast_list(list: &AST) -> String {
//     let mut out = String::new();
//     match &list.kind {
//         ASTKind::Namespace(name) => {
//             for i in name {
//                 out.push_str(&format!("{:?}", i.kind))
//             }
//         }
//         _ => ()
//     }
//     return out;
// }

#[test]
fn test() {
    let cfg = ParserConfig::default();
   let toc =  cfg.parse(r#"
{x}




[c]
"#).unwrap().toc(9);
    println!("{:#?}", toc)
}