pub mod block {
    pub mod atx_header_block;
    pub mod thematic_break_block;
}
pub mod inline {
    pub mod textual;
}

use biome_markdown_syntax::{kind::MarkdownSyntaxKind::*, MarkdownSyntaxKind, T};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, prelude::ParsedSyntax::{self, *}, Parser
};
use block::thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};

use crate::MarkdownParser;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();
    let _ = parse_block_list(p);
    m.complete(p, MD_DOCUMENT);
}

struct MdBlockList;

struct HashListParseRecovery;

impl ParseRecovery for HashListParseRecovery {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;
    const RECOVERED_KIND: Self::Kind = MD_BOGUS;

    fn is_at_recovered(&self, _p: &mut Self::Parser<'_>) -> bool {
        true
    }
}

impl ParseNodeList for MdBlockList {
    type Kind= MarkdownSyntaxKind;

    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BLOCK_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_block(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![EOF])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        todo!()
    }
}

pub(crate) fn parse_block_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    while !p.at(T![EOF]) {
        parse_any_block(p);
    }
    Present(m.complete(p, MD_BLOCK_LIST))
}

pub(crate) fn parse_any_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if at_indent_code_block(p) {
        return parse_indent_code_block(p);
    } else if at_thematic_break_block(p) {
        let break_block = try_parse(p, |p| {
            let break_block = parse_thematic_break_block(p);
            if break_block.is_absent() {
                return Err(());
            }
            Ok(break_block)
        });
        match break_block {
            Ok(break_block) => return break_block,
            Err(_) => return parse_paragraph(p),
        }
    } else {
        return parse_paragraph(p);
    }
}

pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    p.before_whitespace_count() > 4
}

pub(crate) fn parse_indent_code_block(_p: &mut MarkdownParser) -> ParsedSyntax {
    todo!()
}

pub(crate) fn parse_paragraph(_p: &mut MarkdownParser) -> ParsedSyntax {
    todo!()
}

/// Attempt to parse some input with the given parsing function. If parsing
/// succeeds, `Ok` is returned with the result of the parse and the state is
/// preserved. If parsing fails, this function rewinds the parser back to
/// where it was before attempting the parse and the `Err` value is returned.
#[must_use = "The result of try_parse contains information about whether the parse succeeded and should not be ignored"]
pub(crate) fn try_parse<T, E>(
    p: &mut MarkdownParser,
    func: impl FnOnce(&mut MarkdownParser) -> Result<T, E>,
) -> Result<T, E> {
    let checkpoint = p.checkpoint();

    let res = func(p);

    if res.is_err() {
        p.rewind(checkpoint);
    }

    res
}
