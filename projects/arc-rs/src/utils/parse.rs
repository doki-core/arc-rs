use crate::{
    ast::{KeyNode, KeyPath, KeyType},
    Arc,
};
use arc_pest::{ArcParser, Rule};
use linked_hash_map::LinkedHashMap;
use pest::{iterators::Pair, Parser};
use std::{
    borrow::BorrowMut,
    collections::{hash_map::RandomState, VecDeque},
    fs::read_to_string,
};


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
    ctx.analyze();
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
    orders: Vec<Arc>,
    last_node: Option<Arc>,
    allow_override: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context { root: Arc::new_dict(), orders: vec![], last_node: None, allow_override: true }
    }
}

impl Context {
    pub fn parse(&mut self, text: &str) {
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
    pub fn analyze(&mut self) {
        for i in self.orders.clone() {
            match i {
                Arc::Record(k, v) => {
                    let mut node = self.get_key_path_node(&k);
                    println!("{:?}", node);
                }
                Arc::Key(_, _) => {}
                _ => unreachable!(),
            }
        }
    }

    fn get_key_path_node(&mut self, p: &KeyPath) -> &Arc {
        let mut node = match &mut self.last_node {
            None => &mut self.root,
            Some(s) => s,
        };

        for n in &p.0 {
            match n {
                KeyNode::Key(k) => match node {
                    Arc::Dict(dict) => node = dict.entry(k.clone()).or_insert(Arc::new_dict()),
                    _ => panic!("not a dict!"),
                },
                // Can't create list with key path!!!
                KeyNode::Index(i) => unreachable!(),
            };
        }
        return node;
    }

    fn parse_root_dict(&mut self, pairs: Pair<Rule>) {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::dict_pair => {
                    let (k, v) = self.parse_dict_pair(pair);
                    self.orders.push(Arc::Record(k, Box::new(v)))
                }
                _ => debug_cases!(pair),
            };
        }
    }
    fn parse_dict(&self, pairs: Pair<Rule>) -> Arc {
        let mut hash = LinkedHashMap::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::dict_pair => {
                    let (k, v) = self.parse_dict_pair(pair);
                    hash.insert(k, v)
                }
                _ => debug_cases!(pair),
            };
        }
        return Arc::FreeDict(hash);
    }
    fn parse_dict_pair(&self, pairs: Pair<Rule>) -> (KeyPath, Arc) {
        let mut k = KeyPath::default();
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
    fn parse_name_space(&self, pairs: Pair<Rule>) -> KeyPath {
        let mut vec = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Key => {
                    let s = match self.parse_value(pair) {
                        Arc::Number(i) => KeyNode::Index(0),
                        Arc::String(s) => KeyNode::Key(Box::from(s)),
                        _ => unreachable!(),
                    };
                    vec.push(s)
                }
                _ => debug_cases!(pair),
            };
        }
        return KeyPath(vec);
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
    // fn parse_key(&self, pairs: Pair<Rule>) -> Arc {
    // for pair in pairs.into_inner() {
    // return match pair.as_rule() {
    // Rule::String => self.parse_value(pair),
    // Rule::dict_literal => continue,
    // Rule::list_literal => continue,
    // _ => debug_cases!(pair),
    // };
    // }
    // Arc::Null
    // }
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
