mod config;
use arc_ast::{AST, TextRange};
use arc_pest::Span;
pub use crate::parser::config::ParserConfig;
use crate::{ Result};
use arc_pest::{Pair, Pairs, Parser, Rule, ArcParser};

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
        Ok(self.parse_program(ArcParser::parse(Rule::program, &input)?))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::statement => {
                    codes.push(self.parse_statement(pair));
                }
                Rule::data=>{
                    codes.push(self.parse_data(pair));
                },
                _ => debug_cases!(pair),
            };
        }
        AST::program(codes)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes: Vec<AST> = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                // Rule::WHITESPACE => continue,
                // Rule::expression => self.parse_expression(pair),
                // Rule::if_statement => self.parse_if_else(pair),
                // Rule::for_statement => self.parse_for_in(pair),
                // Rule::assign_statement => self.parse_assign(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        unimplemented!()
        // AST::statement(codes, r)
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
            Rule::dict_literal=>self.dict_literal(pair),
            // Rule::list => self.parse_list(pair),
            // Rule::String => self.parse_string(pair),
            // Rule::Number => self.parse_number(pair),
            // Rule::Symbol => self.parse_namespace(pair),
            // Rule::SpecialValue => self.parse_special(pair),

            _ => debug_cases!(pair),
        }
    }
    fn dict_literal(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::dict_pair=>self.dict_pair(pair),
                // Rule::WHITESPACE => continue,
                // Rule::expression => self.parse_expression(pair),
                // Rule::if_statement => self.parse_if_else(pair),
                // Rule::for_statement => self.parse_for_in(pair),
                // Rule::assign_statement => self.parse_assign(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        unimplemented!()
    }
    fn dict_pair(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes: Vec<AST> = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::namespace=>self.namespace(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        unimplemented!()
    }
    //
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
    fn namespace(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut value: Vec<AST> = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            };
        }
        unimplemented!()
    }
    // fn parse_symbol(&self, pairs: Pair<Rule>) -> AST {
    //     let r = self.get_position(pairs.as_span());
    //     let value = vec![self.parse_string(pairs)];
    //     AST::symbol(value, r)
    // }
    //
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
    // fn parse_number(&self, pairs: Pair<Rule>) -> AST {
    //     let r = self.get_position(pairs.as_span());
    //     let pair = pairs.into_inner().nth(0).unwrap();
    //     match pair.as_rule() {
    //         Rule::Integer => AST::integer(pair.as_str(), 10, r),
    //         Rule::Decimal => AST::decimal(pair.as_str(), 10, r),
    //         Rule::DecimalBad => {
    //             let s = pair.as_str().to_string();
    //             match s.starts_with(".") {
    //                 true => AST::decimal(&format!("0{}", s), 10, r),
    //                 false => AST::decimal(&format!("{}0", s), 10, r),
    //             }
    //         }
    //         Rule::Byte => {
    //             let s = pair.as_str().to_string();
    //             match &s[0..1] {
    //                 "0b" => AST::integer(pair.as_str(), 2, r),
    //                 "0o" => AST::integer(pair.as_str(), 8, r),
    //                 "0x" => AST::integer(pair.as_str(), 16, r),
    //                 _ => AST::decimal(pair.as_str(), 16, r),
    //             }
    //         }
    //         _ => unreachable!(),
    //     }
    // }
    // fn parse_special(&self, pairs: Pair<Rule>) -> AST {
    //     let r = self.get_position(pairs.as_span());
    //     match pairs.as_str() {
    //         "true" => AST::boolean(true, r),
    //         "false" => AST::boolean(false, r),
    //         _ => AST::null(r),
    //     }
    // }
}
