use crate::kind_src::KindsSrc;

// 对Token进行分组，生成is_xx 方法
pub const MARKDOWN_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("/", "SLASH"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("`", "BACKTICK"),
        ("~", "TILDE"),
    ],
    keywords: &["false"],
    literals: &[
        "MARKDOWN_HARD_LINE_LITERAL",
        "MARKDOWN_SOFT_BREAK_LITERAL",
        "MARKDOWN_TEXTUAL_LITERAL",
        "MARKDOWN_STRING_LITERAL",
        "MARKDOWN_INDENT_CHUNK_LITERAL",
        "MARKDOWN_BREAK_BLOCK_LITERAL",
    ],
    tokens: &["NEWLINE", "WHITESPACE"],
    nodes: &[
        // Bogus nodes
        "BOGUS",
        "MARKDOWN_BOGUS",
        // node
        "MARKDOWN_DOCUMENT",
        "MARKDOWN_HEADER",
        "MARKDOWN_H1",
        "MARKDOWN_H2",
        "MARKDOWN_H3",
        "MARKDOWN_H4",
        "MARKDOWN_H5",
        "MARKDOWN_H6",
        "MARKDOWN_SETEXT_H1",
        "MARKDOWN_SETEXT_H2",
        "MARKDOWN_INDENT_CODE_BLOCK",
        "MARKDOWN_FENCED_CODE_BLOCK",
        "MARKDOWN_HTML_BLOCK",
        "MARKDOWN_LINK_BLOCK",
        "MARKDOWN_QUOTE",
        "MARKDOWN_ORDER_LIST_ITEM",
        "MARKDOWN_BULLET_LIST_ITEM",
        "MARKDOWN_BULLET_LIST",
        "MARKDOWN_ORDER_LIST",
        "MARKDOWN_PARAGRAPH",
        "MARKDOWN_PARAGRAPH_ITEM_LIST",
        "MARKDOWN_INLINE_CODE",
        "MARKDOWN_INLINE_EMPHASIS",
        "MARKDOWN_INLINE_LINK",
        "MARKDOWN_INLINE_IMAGE",
        "MARKDOWN_HARD_LINE",
        "MARKDOWN_SOFT_BREAK",
        "MARKDOWN_TEXTUAL",
        "MARKDOWN_STRING",
        "MARKDOWN_INDENT",
        "MARKDOWN_BREAK_BLOCK",
    ],
};
