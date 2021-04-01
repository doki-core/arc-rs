
use crate::{Range, AST};
use crate::ast::ASTKind;

#[derive(Debug)]
pub struct TOC {
    level: usize,
    pub detail: String,
    pub range: Range,
    pub children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), children: vec![] }
    }
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

impl AST {
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut root = TOC::default();
        match &self.kind {
            ASTKind::Program(terms) | ASTKind::Sequence(terms) => {
                for term in terms {
                    match &term.kind {
                        ASTKind::DictScope(level, children) => {
                            if toc_ignore {
                                toc_ignore = false;
                                continue;
                            }
                            if *level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(level - 1);
                            let new = TOC {
                                level: *level,
                                detail: join_ast_list(children),
                                range: *r,
                                children: vec![],
                            };
                            parent.children.push(new);
                        }
                        ASTKind::ListScope(level, children) => {
                            if toc_ignore {
                                toc_ignore = false;
                                continue;
                            }
                            if *level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(level - 1);
                            let new = TOC {
                                level: *level,
                                detail: join_ast_list(children),
                                range: *r,
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

pub fn join_ast_list(list: &[AST]) -> String {
    let mut out = String::new();
    for i in list {
        out.push_str(&i.to_string())
    }
    return out;
}
