mod config;
pub use self::config::ParserConfig;
use crate::{
    ast::{ASTKind, ExtendStatement},
    value::Text,
    Result, RuntimeError, TextRange, AST,
};
use arc_pest::{ArcParser, Pair, Pairs, Parser, Rule, Span};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserConfig {
    pub fn parse(&self, input: &str) -> Result<AST> {
        let input = input.replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        match ArcParser::parse(Rule::program, &input) {
            Ok(o) => Ok(self.parse_program(o)),
            Err(e) => Err(RuntimeError::OtherError(box e)),
        }
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        let mut additional = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::statement => {
                    codes.push(self.parse_extend(pair));
                }
                Rule::data => return self.parse_data(pair),
                Rule::dict_pair => codes.push(self.parse_dict_pair(pair)),
                Rule::dict_head => codes.push(self.parse_dict_head(pair)),
                Rule::COMMENT => additional = Some(pair.as_str().to_string()),
                Rule::extend_statement => codes.push(self.parse_extend(pair)),
                _ => debug_cases!(pair),
            };
        }
        AST { kind: ASTKind::Program(codes), range: None, additional }
    }
    fn parse_extend(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut path = String::new();
        let mut format = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => format = pair.as_str().to_string(),
                Rule::StringNormal => path = self.parse_string_inner(pair).as_str().to_string(),
                _ => debug_cases!(pair),
            };
        }
        let ext = ExtendStatement::new(format, path, self.file_path.to_owned());
        AST { kind: ASTKind::ExtendStatement(ext), range: r.boxed(), additional: None }
    }
    // fn parse_block(&self, pairs: Pair<Rule>) -> AST {
    //     let pair = pairs.into_inner().nth(0).unwrap();
    //     match pair.as_rule() {
    //         Rule::statement => self.parse_statement(pair),
    //         _ => unreachable!(),
    //     }
    // }
}

impl ParserConfig {
    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::list_literal => self.parse_list_literal(pair),
            Rule::dict_literal => self.parse_dict_literal(pair),
            Rule::String => self.parse_string(pair),
            Rule::Special => self.parse_special(pair),
            Rule::Number => self.parse_number(pair),
            Rule::Cite => self.parse_cite(pair),
            // Rule::Symbol => self.parse_namespace(pair),
            // Rule::SpecialValue => self.parse_special(pair),
            _ => debug_cases!(pair),
        }
    }
    fn parse_list_literal(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SEPARATOR => continue,
                Rule::InlineString => codes.push(self.parse_string_bare(pair)),
                Rule::data => codes.push(self.parse_data(pair)),
                _ => debug_cases!(pair),
            };
        }
        let mut out = AST::list(codes);
        out.set_range(r);
        return out;
    }
    fn parse_dict_head(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut depth = 0;
        let mut path = AST::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dot => depth += 1,
                Rule::namespace => path = self.parse_namespace(pair),
                _ => debug_cases!(pair),
            };
        }
        AST { kind: ASTKind::DictScope(depth, Box::new(path)), range: r.boxed(), additional: None }
    }
    fn parse_dict_literal(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SEPARATOR => continue,
                Rule::dict_pair => codes.push(self.parse_dict_pair(pair)),
                _ => debug_cases!(pair),
            };
        }
        let mut out = AST::dict(codes);
        out.set_range(r);
        return out;
    }
    fn parse_dict_pair(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut key = AST::default();
        let mut value = AST::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Set => continue,
                Rule::namespace => key = self.parse_namespace(pair),
                Rule::data => value = self.parse_data(pair),
                Rule::RestLineText => value = self.parse_string_bare(pair),
                _ => debug_cases!(pair),
            };
        }
        let mut out = AST::pair(key, value);
        out.set_range(r);
        return out;
    }
    // fn parse_list(&self, pairs: Pair<Rule>) -> AST {
    //     let r = self.get_position(pairs.as_span());
    //     let mut terms = vec![];
    //     for pair in pairs.into_inner() {
    //         match pair.as_rule() {
    //             Rule::WHITESPACE => continue,
    //             Rule::Comma => continue,
    //             Rule::expr => terms.push(self.parse_expr(pair)),
    //             _ => debug_cases!(pair),
    //         };
    //     }
    //     AST::list(terms, r)
    // }
    // fn parse_pair(&self, pairs: Pair<Rule>) -> (AST, AST) {
    //     let (mut key, mut value) = Default::default();
    //     for pair in pairs.into_inner() {
    //         match pair.as_rule() {
    //             Rule::Set => continue,
    //             Rule::BadSymbol => key = self.parse_string(pair),
    //             Rule::term => value = self.parse_term(pair),
    //             _ => debug_cases!(pair),
    //         };
    //     }
    //     (key, value)
    // }
    fn parse_namespace(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut symbols: Vec<AST> = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dot => continue,
                Rule::StringNormal => {
                    let key = AST::string(self.parse_string_inner(pair));
                    symbols.push(key)
                }
                Rule::SYMBOL => {
                    let key = AST::string(Text::from(pair.as_str()));
                    symbols.push(key)
                }
                Rule::SignedNumber => {
                    let index = AST::integer(pair.as_str());
                    symbols.push(index)
                }
                _ => debug_cases!(pair),
            };
        }
        let mut out = AST::namespace(symbols);
        out.set_range(r);
        return out;
    }
    // fn parse_string(&self, pairs: Pair<Rule>) -> AST {
    //     let r = self.get_position(pairs.as_span());
    //     let mut is_pure_string = true;
    //     let mut block = vec![];
    //     let mut _marks = 0;
    //     let mut buffer = String::new();
    //     for pair in pairs.into_inner() {
    //         match pair.as_rule() {
    //             Rule::StringEmpty => return AST::string(String::new(), r),
    //             Rule::S1 | Rule::S2 | Rule::S3 | Rule::S4 => _marks += 1,
    //             Rule::NS1 | Rule::NS2 | Rule::NS3 | Rule::NS4 => {
    //                 let text = pair.as_str();
    //                 match text {
    //                     "{{" => buffer.push('{'),
    //                     "}}" => buffer.push('}'),
    //                     "\\n" => buffer.push('\n'),
    //                     _ => match text.starts_with('\\') {
    //                         true => buffer.push_str(&text[1..text.len()]),
    //                         false => buffer.push_str(text),
    //                     },
    //                 }
    //             }
    //             Rule::expr => {
    //                 is_pure_string = false;
    //                 if !buffer.is_empty() {
    //                     block.push(AST::string(buffer, Default::default()));
    //                     buffer = String::new()
    //                 }
    //                 block.push(self.parse_expr(pair))
    //             }
    //             _ => debug_cases!(pair),
    //         };
    //     }
    //     match is_pure_string {
    //         true => AST::string(buffer, r),
    //         false => {
    //             if !buffer.is_empty() {
    //                 block.push(AST::string(buffer, Default::default()));
    //             }
    //             AST::string_expression(block, Default::default(), r)
    //         }
    //     }
    // }
    //
    fn parse_string(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut text = Text::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringNormal => text = self.parse_string_inner(pair),
                _ => debug_cases!(pair),
            };
        }
        let mut out = AST::string(text);
        out.set_range(r);
        return out;
    }
    fn parse_string_bare(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let text = Text::string_bare(pairs.as_str());
        let mut out = AST::string(text);
        out.set_range(r);
        return out;
    }
    fn parse_string_inner(&self, pairs: Pair<Rule>) -> Text {
        let mut is_literal = false;
        let mut text = String::with_capacity(pairs.as_str().len());
        let mut delimiter = 0;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::S1 => {
                    delimiter += 1;
                    is_literal = true
                }
                Rule::S2 => delimiter += 1,
                Rule::NS1 | Rule::NS2 => text.push_str(pair.as_str()),
                _ => debug_cases!(pair),
            };
        }
        match is_literal {
            true => Text::string_literal(text, "", delimiter / 2),
            false => Text::string_escaped(text, "", delimiter / 2),
        }
    }
    fn parse_cite(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let item = pairs.into_inner().next().unwrap();
        AST { kind: ASTKind::Cite(Box::new(self.parse_namespace(item))), range: r.boxed(), additional: None }
    }
    fn parse_number(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut items = pairs.into_inner();
        let mut out = AST::number(items.next().unwrap().as_str());
        out.set_range(r);
        return out;
    }
    fn parse_special(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut out = match pairs.as_str() {
            "true" => AST::boolean(true),
            "false" => AST::boolean(false),
            _ => AST::null(),
        };
        out.set_range(r);
        return out;
    }
}
