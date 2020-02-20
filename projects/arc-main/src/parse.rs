use arc_ast::Arc;
use arc_parser::{ArcParser, Rule};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::{HashMap, VecDeque};
use std::fs::{read_to_string, File};
use std::io::Write;

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

pub fn parse(text: &str) -> Arc {
    let mut ctx = Context::default();
    ctx.parse(text);
    ctx.root
}

pub fn parse_file(path_from: &str) -> Result<Arc, std::io::Error> {
    let r = read_to_string(path_from)?;
    let s = parse(&r);
    return Ok(s);
}

#[derive(Debug)]
pub struct Context {
    pub root: Arc,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            root: Arc::new_dict(),
        }
    }
}

impl Context {
    pub fn parse(&self, text: &str) {
        let pairs = ArcParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::dict_literal => self.parse_root_dict(pair),
                _ => debug_cases!(pair),
            };
        }
        //        println!("{:#?}", codes);
        //        unreachable!();
    }

    fn parse_root_dict(&self, pairs: Pair<Rule>) {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::dict_pair => {
                    let (k, v) = self.parse_dict_pair(pair);
                    println!("{:?}: {}", k, v)
                }
                _ => debug_cases!(pair),
            };
        }
    }
    fn parse_dict(&self, pairs: Pair<Rule>) -> Arc {
        let hash = Arc::new_dict();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::dict_pair => {
                    let (k, v) = self.parse_dict_pair(pair);
                    println!("{:?}: {}", k, v)
                }
                _ => debug_cases!(pair),
            };
        }
        return hash;
    }
    fn parse_dict_pair(&self, pairs: Pair<Rule>) -> (Vec<Arc>, Arc) {
        let mut k = vec![];
        let mut v = Arc::Null;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Set => continue,
                Rule::NameSpace => k = self.parse_name_space(pair),
                Rule::Value => v = self.parse_value(pair),
                _ => debug_cases!(pair),
            };
        }
        return (k, v);
    }
    fn parse_name_space(&self, pairs: Pair<Rule>) -> Vec<Arc> {
        let mut vec = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Key => vec.push(self.parse_value(pair)),
                _ => debug_cases!(pair),
            };
        }
        return vec;
    }
    fn parse_list(&self, pairs: Pair<Rule>) -> Arc {
        let mut vec = VecDeque::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::list_empty => continue,
                Rule::Value => vec.push_back(self.parse_value(pair)),
                _ => debug_cases!(pair),
            };
        }
        return Arc::List(vec);
    }
    /*
    fn parse_key(&self, pairs: Pair<Rule>) -> Arc {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                Rule::String => self.parse_value(pair),
                Rule::dict_literal => continue,
                Rule::list_literal => continue,
                _ => debug_cases!(pair),
            };
        }
        Arc::Null
    }
    */
    fn parse_value(&self, pairs: Pair<Rule>) -> Arc {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                Rule::String => self.parse_value(pair),
                Rule::StringNormal => {
                    let s = pair.as_str();
                    /*
                    match s.chars().next().unwrap() {
                        '"' =>
                    }
                    */
                    
                    Arc::from(&s[1..s.len() - 1])
                }
                Rule::dict_literal => self.parse_dict(pair),
                Rule::list_literal => self.parse_list(pair),
                _ => debug_cases!(pair),
            };
        }
        Arc::Null
    }
}
