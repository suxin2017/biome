use crate::parser::MarkdownParser;
use biome_markdown_syntax::{
    MarkdownSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    diagnostic::expected_node,
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecovery, RecoveryError},
    prelude::{
        ParseDiagnostic,
        ParsedSyntax::{self, *},
    },
    Parser,
};
use biome_rowan::{TextRange, TextSize};

// 需要解决几个问题
// # 前面有空格的问题
// 转译问题
// 连续#的问题

pub(crate) fn at_atx_header_block(p: &mut MarkdownParser) -> bool {
    p.at(T![#])
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_atx_header_block(p) {
        return Absent;
    }
    let m = p.start();

    let mut before_hash_list = MdHashList::new(p);    

    before_hash_list.parse_list(p);

    parse_any_inline(p);
    
    let mut after_hash_list = MdHashList::new(p);    
    after_hash_list.parse_list(p);

    Present(m.complete(p, MD_HEADER))
}

struct MdHashList {
    whitespace_count: usize,
}

impl MdHashList {
    pub fn new(p: &MarkdownParser<'_>) -> Self {
        Self {
            whitespace_count: p.before_whitespace_count(),
        }
    }

    pub fn at_continuous_hash(&self, p: &MarkdownParser<'_>) -> bool {
        self.whitespace_count == p.before_whitespace_count()
    }
}

impl ParseNodeList for MdHashList {
    type Kind = MarkdownSyntaxKind;

    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_HASH_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let m = p.start();
        p.expect(T![#]);
        Present(m.complete(p, MD_HASH))
    }

    // different token or space or tab
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        self.at_continuous_hash(p) && p.at(HASH)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        match parsed_element {
            Absent => Err(RecoveryError::AlreadyRecovered),
            Present(m) => Ok(m),
        }
    }
}