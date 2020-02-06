use std::fs::{read_to_string, File};
use std::io::Write;
use arc_parser::{ArcParser, Rule};
use pest::Parser;
use pest::iterators::Pair;
use textwrap::indent;

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
    pub arc_indent: usize,
    pub arc_symbol_set: String,
    pub arc_dict_separator: String,
    pub arc_list_separator: String,
    pub arc_list_max_length: usize,
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
        //        println!("{:#?}", codes);
        //        unreachable!();
        return code;
    }
    fn format_json_dict(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::dict_pair => {
                    codes.push(self.format_dict_pair(pair))
                }
                _ => debug_cases!(pair),
            };
        }
        return codes.join("\n");
    }
    fn format_dict_literal(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::dict_pair => {
                    codes.push(self.format_dict_pair(pair))
                }
                _ => debug_cases!(pair),
            };
        }
        match codes.len() {
            0 => String::from("{}"),
            1 => format!("{{{}}}", codes[0]),
            _ => {
                let i = &" ".repeat(self.arc_indent);
                let s = match self.arc_dict_separator.as_str() {
                    "," => ",",
                    ";" => ";",
                    _ => ""
                };
                let mut code = String::new();
                for c in &codes {
                    code.push_str(i);
                    code.push_str(c);
                    code.push_str(s);
                    code.push('\n')
                }
                format!("{{\n{}}}", code)
            }
        }
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
        match self.arc_symbol_set.as_str() {
            "=" => format!("{} = {}", key, value),
            _ => format!("{}: {}", key, value),
        }
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
                    let s = pair.as_str();
                    //FIXME
                    return if s.contains('.') {
                        pair.as_str().to_string()
                    } else {
                        s.trim_matches('"').to_string()
                    };
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
