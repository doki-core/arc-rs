// This file was generated by Peginator v0.6.0 built at 1670312979
// CRC-32/ISO-HDLC of the grammar file: 81bc5b2c
// Any changes to it will be lost on regeneration

#[derive(Debug, Clone)]
pub struct VomlParser {
    pub statements: Statement,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Statement {
    KeyValueNode(KeyValueNode),
    ScopeNode(ScopeNode),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ValueNode {
    NumberNode(NumberNode),
    SpecialNode(SpecialNode),
    StringNode(StringNode),
    TableNode(TableNode),
}
#[derive(Debug, Clone)]
pub struct ScopeNode {
    pub namespace: NamespaceNode,
}
#[derive(Debug, Clone)]
pub struct TableNode {
    pub hint: Option<IdentifierNode>,
    pub items: Vec<TableItem>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TableItem {
    KeyValueNode(KeyValueNode),
    Split(Split),
    ValueNode(ValueNode),
}
#[derive(Debug, Clone)]
pub struct KeyValueNode {
    pub key: KeyNode,
    pub value: ValueNode,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum KeyNode {
    IdentifierNode(IdentifierNode),
    StringNode(StringNode),
}
#[derive(Debug, Clone)]
pub struct StringNode {
    pub hint: Option<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct NumberNode {
    pub num: Num,
    pub hint: Option<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct Num {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct NamespaceNode {
    pub path: Vec<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct SpecialNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
pub type XID_START = char;
pub type XID_CONTINUE = char;
pub type Dot = char;
pub type Split = char;
impl peginator_generated::PegParserAdvanced<()> for VomlParser {
    fn parse_advanced<TT: peginator_generated::ParseTracer>(
        s: &str,
        settings: &peginator_generated::ParseSettings,
        user_defined: (),
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(peginator_generated::parse_VomlParser(
            peginator_generated::ParseState::new(s, settings),
            &mut peginator_generated::ParseGlobal::<TT, peginator_generated::ParseCache, ()>::new(
                Default::default(),
                user_defined,
            ),
        )?
        .result)
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut, dead_code)]
mod peginator_generated {
    use super::*;
    use peginator::*;
    pub use peginator::{
        IndentedTracer, ParseError, ParseGlobal, ParseSettings, ParseState, ParseTracer, PegParser, PegParserAdvanced,
        PegPosition,
    };
    #[derive(Default)]
    pub struct ParseCache<'a> {
        _please_dont_complain: std::marker::PhantomData<&'a ()>,
    }
    mod VomlParser_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: statements, state } =
                parse_Whitespace(state, &mut *global).and_then(|ParseOk { state, .. }| parse_Statement(state, &mut *global))?;
            let ParseOk { state, .. } =
                parse_Whitespace(state, &mut *global).and_then(|ParseOk { state, .. }| parse_end_of_input(state))?;
            Ok(ParseOk { result: statements, state })
        }
        pub type Parsed = Statement;
    }
    #[inline]
    pub(super) fn parse_VomlParser<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, VomlParser> {
        global.tracer.print_trace_start(&state, "VomlParser");
        let result = {
            let result = VomlParser_impl::parse(state, global)?.map(|r| super::VomlParser { statements: r });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod Statement_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_ScopeNode(state, global))
                        .map_inner(Parsed__override::ScopeNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_KeyValueNode(state, global))
                        .map_inner(Parsed__override::KeyValueNode)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::Statement as Parsed__override;
    }
    #[inline]
    pub(super) fn parse_Statement<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, Statement> {
        global.tracer.print_trace_start(&state, "Statement");
        let result = {
            let result = Statement_impl::parse(state, global)?;
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod ValueNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_TableNode(state, global))
                        .map_inner(Parsed__override::TableNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_SpecialNode(state, global))
                        .map_inner(Parsed__override::SpecialNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_NumberNode(state, global))
                        .map_inner(Parsed__override::NumberNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_StringNode(state, global))
                        .map_inner(Parsed__override::StringNode)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::ValueNode as Parsed__override;
    }
    #[inline]
    pub(super) fn parse_ValueNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, ValueNode> {
        global.tracer.print_trace_start(&state, "ValueNode");
        let result = {
            let result = ValueNode_impl::parse(state, global)?;
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod ScopeNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '^'))
                .discard_result()?;
            let ParseOk { result: namespace, state } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_NamespaceNode(state, &mut *global))?;
            Ok(ParseOk { result: namespace, state })
        }
        pub type Parsed = NamespaceNode;
    }
    #[inline]
    pub(super) fn parse_ScopeNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, ScopeNode> {
        global.tracer.print_trace_start(&state, "ScopeNode");
        let result = {
            let result = ScopeNode_impl::parse(state, global)?.map(|r| super::ScopeNode { namespace: r });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod TableNode_impl {
        use super::*;
        mod part_2 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a, TT: ParseTracer>(
                state: ParseState<'a>,
                global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut items: Vec<TableItem> = Vec::new();
                loop {
                    match parse_Whitespace(state.clone(), &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_TableItem(state, &mut *global))
                        .map_inner(|result| vec![result])
                    {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            items.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: items, state })
            }
            pub type Parsed = Vec<TableItem>;
        }
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: hint, state } = parse_Whitespace(state.clone(), &mut *global)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, &mut *global))
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '('))
                .discard_result()?;
            let ParseOk { result: mut items, state } = part_2::parse(state, global)?;
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, ')'))
                .discard_result()?;
            Ok(ParseOk { result: Parsed { hint, items }, state })
        }
        pub struct Parsed {
            pub hint: Option<IdentifierNode>,
            pub items: Vec<TableItem>,
        }
    }
    #[inline]
    pub(super) fn parse_TableNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, TableNode> {
        global.tracer.print_trace_start(&state, "TableNode");
        let result = {
            let result = TableNode_impl::parse(state, global)?.map(|r| super::TableNode { hint: r.hint, items: r.items });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod TableItem_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_KeyValueNode(state, global))
                        .map_inner(Parsed__override::KeyValueNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_ValueNode(state, global))
                        .map_inner(Parsed__override::ValueNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_Split(state, global))
                        .map_inner(Parsed__override::Split)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::TableItem as Parsed__override;
    }
    #[inline]
    pub(super) fn parse_TableItem<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, TableItem> {
        global.tracer.print_trace_start(&state, "TableItem");
        let result = {
            let result = TableItem_impl::parse(state, global)?;
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod KeyValueNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: key, state } =
                parse_Whitespace(state, &mut *global).and_then(|ParseOk { state, .. }| parse_KeyNode(state, &mut *global))?;
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, ':'))
                .discard_result()?;
            let ParseOk { result: value, state } =
                parse_Whitespace(state, &mut *global).and_then(|ParseOk { state, .. }| parse_ValueNode(state, &mut *global))?;
            Ok(ParseOk { result: Parsed { key, value }, state })
        }
        pub struct Parsed {
            pub key: KeyNode,
            pub value: ValueNode,
        }
    }
    #[inline]
    pub(super) fn parse_KeyValueNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, KeyValueNode> {
        global.tracer.print_trace_start(&state, "KeyValueNode");
        let result = {
            let result = KeyValueNode_impl::parse(state, global)?.map(|r| super::KeyValueNode { key: r.key, value: r.value });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    mod KeyNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, global))
                        .map_inner(Parsed__override::IdentifierNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_StringNode(state, global))
                        .map_inner(Parsed__override::StringNode)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::KeyNode as Parsed__override;
    }
    #[inline]
    pub(super) fn parse_KeyNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, KeyNode> {
        global.tracer.print_trace_start(&state, "KeyNode");
        let result = {
            let result = KeyNode_impl::parse(state, global)?;
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for KeyNode {
        fn position(&self) -> &std::ops::Range<usize> {
            match self {
                Self::IdentifierNode(x) => x.position(),
                Self::StringNode(x) => x.position(),
            }
        }
    }
    mod StringNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: hint, state } = parse_Whitespace(state.clone(), &mut *global)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, &mut *global))
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '"'))
                .discard_result()?;
            let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '"'))
                .discard_result()?;
            Ok(ParseOk { result: hint, state })
        }
        pub type Parsed = Option<IdentifierNode>;
    }
    #[inline]
    pub(super) fn parse_StringNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, StringNode> {
        global.tracer.print_trace_start(&state, "StringNode");
        let result = {
            let result = StringNode_impl::parse(state.clone(), global)?
                .map_with_state(|r, new_state| super::StringNode { hint: r, position: state.range_until(new_state) });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for StringNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod NumberNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: num, state } = parse_Num(state, &mut *global)?;
            let ParseOk { result: hint, state } = parse_IdentifierNode(state.clone(), &mut *global)
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            Ok(ParseOk { result: Parsed { num, hint }, state })
        }
        pub struct Parsed {
            pub num: Num,
            pub hint: Option<IdentifierNode>,
        }
    }
    #[inline]
    pub(super) fn parse_NumberNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, NumberNode> {
        global.tracer.print_trace_start(&state, "NumberNode");
        let result = {
            let result = NumberNode_impl::parse(state.clone(), global)?.map_with_state(|r, new_state| super::NumberNode {
                num: r.num,
                hint: r.hint,
                position: state.range_until(new_state),
            });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for NumberNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod Num_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a, TT: ParseTracer>(
                state: ParseState<'a>,
                global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                loop {
                    match parse_character_range(state.clone(), '0', '9').discard_result() {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                if iterations == 0 {
                    return Err(state.report_farthest_error());
                }
                Ok(ParseOk { result: (), state })
            }
            pub type Parsed = ();
        }
        mod part_2 {
            use super::*;
            mod optional {
                use super::*;
                mod part_1 {
                    use super::*;
                    mod closure {
                        use super::*;
                    }
                    #[inline(always)]
                    pub fn parse<'a, TT: ParseTracer>(
                        state: ParseState<'a>,
                        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
                    ) -> ParseResult<'a, Parsed> {
                        let mut iterations: usize = 0;
                        let mut state = state;
                        loop {
                            match parse_character_range(state.clone(), '0', '9').discard_result() {
                                Ok(ParseOk { result: __result, state: new_state, .. }) => {
                                    state = new_state;
                                }
                                Err(err) => {
                                    state = state.record_error(err);
                                    break;
                                }
                            }
                            iterations += 1;
                        }
                        if iterations == 0 {
                            return Err(state.report_farthest_error());
                        }
                        Ok(ParseOk { result: (), state })
                    }
                    pub type Parsed = ();
                }
                #[inline(always)]
                pub fn parse<'a, TT: ParseTracer>(
                    state: ParseState<'a>,
                    global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
                ) -> ParseResult<'a, Parsed> {
                    let ParseOk { state, .. } = parse_Dot(state, &mut *global).discard_result()?;
                    let ParseOk { state, .. } = part_1::parse(state, global)?;
                    Ok(ParseOk { result: (), state })
                }
                pub type Parsed = ();
            }
            #[inline(always)]
            pub fn parse<'a, TT: ParseTracer>(
                state: ParseState<'a>,
                global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
            ) -> ParseResult<'a, Parsed> {
                optional::parse(state.clone(), global).or_else(|err| Ok(ParseOk { result: (), state: state.record_error(err) }))
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = ChoiceHelper::new(state.clone())
                .choice(|state| parse_character_literal(state, '+').discard_result())
                .choice(|state| parse_character_literal(state, '-').discard_result())
                .end()
                .or_else(|err| Ok(ParseOk { result: (), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = part_1::parse(state, global)?;
            let ParseOk { state, .. } = part_2::parse(state, global)?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
    }
    #[inline]
    pub(super) fn parse_Num<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, Num> {
        global.tracer.print_trace_start(&state, "Num");
        let result = {
            let result = Num_impl::parse(state.clone(), global)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                Num { string, position: state.range_until(new_state) }
            });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for Num {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod NamespaceNode_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
                #[inline(always)]
                pub fn parse<'a, TT: ParseTracer>(
                    state: ParseState<'a>,
                    global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
                ) -> ParseResult<'a, Parsed> {
                    let ParseOk { state, .. } = parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_Dot(state, &mut *global))
                        .discard_result()?;
                    let ParseOk { result: mut path, state } = parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, &mut *global))
                        .map_inner(|result| vec![result])?;
                    Ok(ParseOk { result: path, state })
                }
                pub type Parsed = Vec<IdentifierNode>;
            }
            #[inline(always)]
            pub fn parse<'a, TT: ParseTracer>(
                state: ParseState<'a>,
                global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut path: Vec<IdentifierNode> = Vec::new();
                loop {
                    match closure::parse(state.clone(), global) {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            path.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: path, state })
            }
            pub type Parsed = Vec<IdentifierNode>;
        }
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: mut path, state } = parse_Whitespace(state, &mut *global)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, &mut *global))
                .map_inner(|result| vec![result])?;
            let ParseOk { result: extend_path_with, state } = part_1::parse(state, global)?;
            path.extend(extend_path_with);
            Ok(ParseOk { result: path, state })
        }
        pub type Parsed = Vec<IdentifierNode>;
    }
    #[inline]
    pub(super) fn parse_NamespaceNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, NamespaceNode> {
        global.tracer.print_trace_start(&state, "NamespaceNode");
        let result = {
            let result = NamespaceNode_impl::parse(state.clone(), global)?
                .map_with_state(|r, new_state| super::NamespaceNode { path: r, position: state.range_until(new_state) });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for NamespaceNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod IdentifierNode_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a, TT: ParseTracer>(
                state: ParseState<'a>,
                global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                loop {
                    match parse_XID_CONTINUE(state.clone(), &mut *global).discard_result() {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: (), state })
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = ChoiceHelper::new(state)
                .choice(|state| parse_XID_START(state, &mut *global).discard_result())
                .choice(|state| parse_character_literal(state, '_').discard_result())
                .end()?;
            let ParseOk { state, .. } = part_1::parse(state, global)?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
    }
    #[inline]
    pub(super) fn parse_IdentifierNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, IdentifierNode> {
        global.tracer.print_trace_start(&state, "IdentifierNode");
        let result = {
            let result = IdentifierNode_impl::parse(state.clone(), global)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                IdentifierNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for IdentifierNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod SpecialNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "null"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "default"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "true"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, &mut *global)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "false"))
                        .discard_result()
                })
                .end()
        }
        pub type Parsed = ();
    }
    #[inline]
    pub(super) fn parse_SpecialNode<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, SpecialNode> {
        global.tracer.print_trace_start(&state, "SpecialNode");
        let result = {
            let result = SpecialNode_impl::parse(state.clone(), global)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                SpecialNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        };
        global.tracer.print_trace_result(&result);
        result
    }
    impl PegPosition for SpecialNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    #[inline]
    pub(super) fn parse_XID_START<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, XID_START> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_start(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
        }
        if let Ok(result) = parse_char(state.clone(), global) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }))
    }
    #[inline]
    pub(super) fn parse_XID_CONTINUE<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, XID_CONTINUE> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_continue(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
        }
        if let Ok(result) = parse_char(state.clone(), global) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }))
    }
    #[inline]
    pub(super) fn parse_Dot<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, Dot> {
        if let Ok(result) = parse_character_literal(state.clone(), '.') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), '。') {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "Dot" }))
    }
    #[inline]
    pub(super) fn parse_Split<'a, TT: ParseTracer>(
        state: ParseState<'a>,
        global: &mut ParseGlobal<TT, ParseCache<'a>, ()>,
    ) -> ParseResult<'a, Split> {
        if let Ok(result) = parse_character_literal(state.clone(), ';') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), ',') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), '，') {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "Split" }))
    }
}
