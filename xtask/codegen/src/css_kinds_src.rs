use crate::kinds_src::KindsSrc;

pub const CSS_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("#", "HASH"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("||", "PIPE2"),
        ("+", "PLUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        (".", "DOT"),
        (":", "COLON"),
        ("::", "COLON2"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("-", "MINUS"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("@", "AT"),
        ("$=", "DOLLAR_EQ"),
        ("~=", "TILDE_EQ"),
        ("-->", "CDC"),
        ("<!--", "CDO"),
    ],
    keywords: &[
        "media",
        "keyframes",
        "not",
        "and",
        "only",
        "or",
        "i",
        "important",
        "highlight",
        "part",
        "dir",
        "local",
        "global",
        "any",
        "current",
        "past",
        "future",
        "host",
        "host-context",
        "matches",
        "is",
        "where",
        "has",
        "lang",
        "nth-child",
        "nth-last-child",
        "nth-of-type",
        "nth-last-of-type",
        "nth-col",
        "nth-last-col",
        "charset",
        "color-profile",
        "counter-style",
        "container",
        "style",
        "ltr",
        "rtl",
        "n",
        "even",
        "odd",
        "of",
        "from",
        "to",
        "var",
        "font-palette-values",
        // length units
        "em",
        "rem",
        "ex",
        "rex",
        "cap",
        "rcap",
        "ch",
        "rch",
        "ic",
        "ric",
        "lh",
        "rlh",
        // Viewport-percentage Lengths
        "vw",
        "svw",
        "lvw",
        "dvw",
        "vh",
        "svh",
        "lvh",
        "dvh",
        "vi",
        "svi",
        "lvi",
        "dvi",
        "vb",
        "svb",
        "lvb",
        "dvb",
        "vmin",
        "svmin",
        "lvmin",
        "dvmin",
        "vmax",
        "svmax",
        "lvmax",
        "dvmax",
        // Absolute lengths
        "cm",
        "mm",
        "q",
        "in",
        "pc",
        "pt",
        "px",
        "mozmm",
        // mini app
        "rpx",
        // container lengths
        "cqw",
        "cqh",
        "cqi",
        "cqb",
        "cqmin",
        "cqmax",
        // angle units
        "deg",
        "grad",
        "rad",
        "turn",
        // time units
        "s",
        "ms",
        // frequency units
        "hz",
        "khz",
        // resolution units
        "dpi",
        "dpcm",
        "dppx",
        "x",
        // flex units
        "fr",
        "url",
        "font-face",
        // Don't add to the end of this list, add to the end of the list in
        // because we have a range check in is_contextual_keyword function.
    ],
    literals: &[
        "CSS_STRING_LITERAL",
        "CSS_NUMBER_LITERAL",
        "CSS_CUSTOM_PROPERTY",
        "CSS_SPACE_LITERAL",
        "CSS_URL_VALUE_RAW_LITERAL",
        "CSS_COLOR_LITERAL",
    ],
    tokens: &[
        "ERROR_TOKEN",
        "IDENT",
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
    ],
    nodes: &[
        "CSS_ROOT",
        "CSS_RULE_LIST",
        "CSS_RULE",
        "CSS_SELECTOR_LIST",
        "CSS_ANY_FUNCTION",
        "CSS_DECLARATION_LIST_BLOCK",
        "CSS_RULE_LIST_BLOCK",
        "CSS_DECLARATION",
        "CSS_IDENTIFIER",
        "CSS_NUMBER",
        "CSS_PARAMETER",
        "CSS_PERCENTAGE",
        "CSS_RATIO",
        "CSS_SIMPLE_FUNCTION",
        "CSS_STRING",
        "CSS_VAR_FUNCTION",
        "CSS_VAR_FUNCTION_VALUE",
        "CSS_ATTRIBUTE_LIST",
        "CSS_DECLARATION_LIST",
        "CSS_COMPONENT_VALUE_LIST",
        "CSS_PARAMETER_LIST",
        "CSS_DECLARATION_IMPORTANT",
        "CSS_UNIT",
        "CSS_PERCENT_DIMENSION",
        "CSS_REGULAR_DIMENSION",
        // Selectors nodes
        "CSS_NAMESPACE",
        "CSS_NAMED_NAMESPACE_PREFIX",
        "CSS_UNIVERSAL_NAMESPACE_PREFIX",
        "CSS_ANY_SELECTOR_LIST",
        "CSS_COMPLEX_SELECTOR",
        "CSS_COMPOUND_SELECTOR",
        "CSS_SUB_SELECTOR_LIST",
        "CSS_ID_SELECTOR",
        "CSS_CLASS_SELECTOR",
        "CSS_TYPE_SELECTOR",
        "CSS_UNIVERSAL_SELECTOR",
        "CSS_PSEUDO_CLASS_SELECTOR",
        "CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS",
        "CSS_PSEUDO_ELEMENT_SELECTOR",
        "CSS_PSEUDO_ELEMENT_IDENTIFIER",
        "CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR",
        "CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER",
        "CSS_PSEUDO_CLASS_IDENTIFIER",
        "CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER",
        "CSS_PSEUDO_CLASS_FUNCTION_SELECTOR",
        "CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR",
        "CSS_COMPOUND_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST",
        "CSS_RELATIVE_SELECTOR_LIST",
        "CSS_RELATIVE_SELECTOR",
        "CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST",
        "CSS_PSEUDO_VALUE_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_NTH",
        "CSS_PSEUDO_CLASS_NTH_SELECTOR",
        "CSS_PSEUDO_CLASS_NTH",
        "CSS_PSEUDO_CLASS_NTH_NUMBER",
        "CSS_PSEUDO_CLASS_NTH_IDENTIFIER",
        "CSS_NTH_OFFSET",
        "CSS_PSEUDO_CLASS_OF_NTH_SELECTOR",
        "CSS_ATTRIBUTE_SELECTOR",
        "CSS_ATTRIBUTE",
        "CSS_ATTRIBUTE_NAME",
        "CSS_ATTRIBUTE_MATCHER",
        "CSS_ATTRIBUTE_MATCHER_VALUE",
        "CSS_PARENTHESIZED_EXPRESSION",
        "CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION",
        "CSS_BINARY_EXPRESSION",
        "CSS_URL_VALUE_RAW",
        "CSS_URL_FUNCTION",
        "CSS_COLOR",
        // At rule nodes
        "CSS_AT_RULE",
        "CSS_CHARSET_AT_RULE",
        "CSS_COLOR_PROFILE_AT_RULE",
        "CSS_COUNTER_STYLE_AT_RULE",
        "CSS_CONTAINER_AT_RULE",
        "CSS_CONTAINER_NOT_QUERY",
        "CSS_CONTAINER_AND_QUERY",
        "CSS_CONTAINER_OR_QUERY",
        "CSS_CONTAINER_QUERY_IN_PARENS",
        "CSS_CONTAINER_STYLE_QUERY_IN_PARENS",
        "CSS_CONTAINER_SIZE_FEATURE_IN_PARENS",
        "CSS_CONTAINER_STYLE_NOT_QUERY",
        "CSS_CONTAINER_STYLE_AND_QUERY",
        "CSS_CONTAINER_STYLE_OR_QUERY",
        "CSS_CONTAINER_STYLE_IN_PARENS",
        "CSS_FONT_FACE_AT_RULE",
        "CSS_FONT_PALETTE_VALUES_AT_RULE",
        "CSS_KEYFRAMES_AT_RULE",
        "CSS_KEYFRAMES_BODY",
        "CSS_MEDIA_AT_RULE",
        "CSS_MEDIA_QUERY_LIST",
        "CSS_MEDIA_QUERY",
        "CSS_MEDIA_CONDITION_QUERY",
        "CSS_MEDIA_TYPE_QUERY",
        "CSS_MEDIA_AND_TYPE_QUERY",
        "CSS_MEDIA_TYPE",
        "CSS_MEDIA_NOT_CONDITION",
        "CSS_MEDIA_AND_CONDITION",
        "CSS_MEDIA_OR_CONDITION",
        "CSS_MEDIA_CONDITION_IN_PARENS",
        "CSS_MEDIA_FEATURE_IN_PARENS",
        "CSS_QUERY_FEATURE_PLAIN",
        "CSS_QUERY_FEATURE_BOOLEAN",
        "CSS_QUERY_FEATURE_RANGE",
        "CSS_QUERY_FEATURE_REVERSE_RANGE",
        "CSS_QUERY_FEATURE_RANGE_INTERVAL",
        "CSS_QUERY_FEATURE_RANGE_COMPARISON",
        "CSS_KEYFRAMES_BLOCK",
        "CSS_KEYFRAMES_ITEM_LIST",
        "CSS_KEYFRAMES_ITEM",
        "CSS_KEYFRAMES_IDENT_SELECTOR",
        "CSS_KEYFRAMES_PERCENTAGE_SELECTOR",
        "CSS_KEYFRAMES_SELECTOR_LIST",
        // Bogus nodes
        "CSS_BOGUS",
        "CSS_BOGUS_BLOCK",
        "CSS_BOGUS_KEYFRAMES_ITEM",
        "CSS_BOGUS_RULE",
        "CSS_BOGUS_SELECTOR",
        "CSS_BOGUS_SUB_SELECTOR",
        "CSS_BOGUS_PSEUDO_CLASS",
        "CSS_BOGUS_PSEUDO_ELEMENT",
        "CSS_BOGUS_AT_RULE",
        "CSS_BOGUS_DECLARATION_ITEM",
        "CSS_BOGUS_COMPONENT_VALUE",
        "CSS_BOGUS_PARAMETER",
        "CSS_BOGUS_MEDIA_QUERY",
    ],
};
