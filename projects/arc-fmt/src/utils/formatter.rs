use crate::{
    utils::{indent, split_once},
    Settings,
};
use arc_parser::{ArcParser, Rule};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
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
        let pairs = ArcParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => continue,
                Rule::dict_scope => code.push_str(&self.format_dict_scope(pair)),
                Rule::dict_literal => {
                    return self.format_json_dict(pair);
                }
                Rule::COMMENT => code.push_str(&format!("{}\n", pair.as_str())),
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
                Rule::dict_pair => codes.push(self.format_dict_pair(pair)),
                _ => debug_cases!(pair),
            };
        }
        return codes.join("\n");
    }
    fn format_dict_literal(&self, pairs: Pair<Rule>) -> String {
        let mut max = 0;
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::dict_empty => return String::from("{}"),
                Rule::dict_pair => {
                    let s = self.format_dict_pair(pair);
                    if s.lines().count() > max {
                        max = s.lines().count()
                    }
                    codes.push(s)
                }
                _ => debug_cases!(pair),
            };
        }
        let i = &" ".repeat(self.arc_indent);

        if codes.len() == 1 {
            if max <= 1 {
                format!("{{{}}}", codes[0])
            } else {
                println!("{:#?}", codes);
                unreachable!();
            }
        } else {
            let s = match self.arc_dict_separator.as_str() {
                "," => ",",
                ";" => ";",
                _ => "",
            };
            let mut code = String::new();
            for c in &codes {
                code.push_str(c);
                code.push_str(s);
                code.push('\n')
            }
            format!("{{\n{}}}", indent(&code, i))
        }
    }
    fn format_list_literal(&self, pairs: Pair<Rule>) -> String {
        let mut lens = 0;
        let mut max = 0;
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SEPARATOR => continue,
                Rule::list_empty => return String::from("[]"),
                Rule::Value => {
                    let s = self.format_value(pair);
                    lens += s.chars().count();
                    if s.lines().count() > max {
                        max = s.lines().count()
                    }
                    codes.push(s)
                }
                _ => debug_cases!(pair),
            };
        }
        let i = &" ".repeat(self.arc_indent);
        if codes.len() == 1 {
            if max <= 1 { format!("[{}]", codes[0]) } else { format!("[\n{}]", indent(&codes[0], i)) }
        } else if lens <= self.arc_list_max_length && max <= 1 {
            format!("[{}]", codes.join(", "))
        } else {
            let s = match self.arc_list_separator.as_str() {
                "," => ",",
                ";" => ";",
                _ => "",
            };
            let mut code = String::new();
            for c in &codes {
                code.push_str(c);
                code.push_str(s);
                code.push('\n')
            }
            format!("[\n{}]", indent(&code, i))
        }
    }
    fn format_dict_scope(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::dict_head => codes.push(format!("{{{}}}", self.format_scope_head(pair))),
                Rule::dict_pair => {
                    let s = self.format_dict_pair(pair);
                    codes.push(s)
                }
                // FIXME
                Rule::COMMENT => continue,
                _ => debug_cases!(pair),
            };
        }
        return codes.join("\n");
    }
    fn format_scope_head(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::NameSpace => {
                    let s = self.format_name_space(pair);
                    codes.push(s)
                }
                _ => debug_cases!(pair),
            };
        }
        return codes.join(".");
    }

    fn format_name_space(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Key => {
                    codes.push(self.format_key(pair));
                }
                Rule::Dot => continue,
                _ => debug_cases!(pair),
            };
        }
        return codes.join(".");
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

    fn format_key(&self, pairs: Pair<Rule>) -> String {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringNormal => {
                    let s = pair.as_str();
                    // FIXME
                    return if s.contains('.') { pair.as_str().to_string() } else { s.trim_matches('"').to_string() };
                }
                Rule::SYMBOL => {
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
                Rule::Number => {
                    return self.format_number(pair);
                }
                Rule::Boolean => {
                    return pair.as_str().to_string();
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
                    return pair.as_str().to_string();
                }
                _ => debug_cases!(pair),
            };
        }
        return code;
    }
    fn format_number(&self, pairs: Pair<Rule>) -> String {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Complex => return self.format_complex_number(pair),
                Rule::Exponent => return self.format_exponent_number(pair),
                Rule::SignedNumber => return self.format_signed_number(pair),
                Rule::Integer => return self.format_integer(pair.as_str()),
                Rule::Decimal => return self.format_decimal(pair.as_str()),
                _ => debug_cases!(pair),
            };
        }
        return String::new();
    }
    fn format_complex_number(&self, pairs: Pair<Rule>) -> String {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            };
        }
        return String::new();
    }
    fn format_exponent_number(&self, pairs: Pair<Rule>) -> String {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            };
        }
        return String::new();
    }
    fn format_signed_number(&self, pairs: Pair<Rule>) -> String {
        let mut sign = "";
        let mut num = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Sign => {
                    if pair.as_str() == "-" {
                        sign = "-"
                    }
                }
                Rule::Integer => num = self.format_integer(pair.as_str()),
                Rule::Decimal => num = self.format_decimal(pair.as_str()),
                Rule::DecimalBad => num = self.format_decimal_bad(pair),
                _ => debug_cases!(pair),
            }
        }
        return format!("{}{}", sign, num);
    }

    fn format_decimal(&self, s: &str) -> String {
        return s.replace("_", "");
    }
    fn format_decimal_bad(&self, pairs: Pair<Rule>) -> String {
        let has_head = pairs.as_str().starts_with('.');
        let mut digits = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Integer => digits.push_str(&self.format_integer(pair.as_str())),
                Rule::Dot => continue,
                _ => debug_cases!(pair),
            };
        }
        if has_head { format!("0.{}", digits) } else { format!("{}.0", digits) }
    }
    fn format_integer(&self, i: &str) -> String {
        let mut code = String::new();
        let p = self.arc_number_separate;
        if p == 0 {
            return i.replace("_", "");
        }
        let mut count = 0;
        for c in i.chars() {
            if c != '_' {
                code.push(c);
                count += 1
            }
        }
        return code;
    }
}
