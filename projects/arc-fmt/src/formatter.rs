use std::fs::{read_to_string, File};
use std::io::Write;
use arc_parser::{ArcParser, Rule};
use pest::Parser;
use pest::iterators::Pair;
macro_rules! debug_cases {
    ($i:ident) => {
        {
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
        }
    };
}

#[derive(Debug)]
pub struct Settings {
    pub symbol_set: String,
    pub pest_indent: usize,
    pub pest_sequence_first: bool,
}

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r);
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        return Ok(());
    }
    pub fn format(&self, text: &str) -> String {
        let pairs = ArcParser::parse(Rule::program, text)
            .unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => continue,
                Rule::dict_literal => {
                    return self.format_json_dict(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return code;
    }
    fn format_json_dict(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::dict_pair => {
                    let s = self.format_dict_pair(pair);
                    code.push_str(&s)
                }
                _ => debug_cases!(pair),
            };
        }
        return code;
    }
    fn format_dict_literal(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        let o = self.format_json_dict(pairs);
        code.push_str(&o);
        return code;
    }
    fn format_list_literal(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::list_empty => return String::from("[]"),
                Rule::Value => {
                    self.format_value(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return code;
    }
    fn format_dict_pair(&self, pairs: Pair<Rule>) -> String {
        let mut key = String::new();
        let mut value = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::Set => continue,
                Rule::NameSpace => {
                    key = self.format_name_space(pair);
                }
                Rule::Value => {
                    value = self.format_value(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return match self.symbol_set.as_str() {
            "=" => format!("{} = {}", key, value),
            _ => format!("{}: {}", key, value),
        };
    }
    fn format_name_space(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Key => {
                    codes.push(self.format_key(pair));
                }
                _ => debug_cases!(pair),
            };
        }
        return codes.join(".");
    }
    fn format_key(&self, pairs: Pair<Rule>) -> String {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringNormal => {
                    return pair.as_str().to_string();
                }
                _ => debug_cases!(pair),
            };
        }
        return String::new();
    }
    fn format_value(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::String => {
                    return self.format_string(pair);
                }
                Rule::dict_literal => {
                    return self.format_dict_literal(pair);
                }
                Rule::list_literal => {
                    return self.format_list_literal(pair);
                }

                _ => debug_cases!(pair),
            };
        }
        return code;
    }
    fn format_string(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        let mut text = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringNormal => {
                    text = pair.as_str().to_string();
                    return text;
                }
                _ => debug_cases!(pair),
            };
        }
        return code;
    }
}
