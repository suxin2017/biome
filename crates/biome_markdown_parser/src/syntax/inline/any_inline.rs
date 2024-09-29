

pub(crate) fn (p: &mut MarkdownParser) -> bool {
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
