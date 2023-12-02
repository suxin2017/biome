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
        "aliceblue",
        "antiquewhite",
        "aqua",
        "aquamarine",
        "azure",
        "beige",
        "bisque",
        "black",
        "blanchedalmond",
        "blue",
        "blueviolet",
        "brown",
        "burlywood",
        "cadetblue",
        "chartreuse",
        "chocolate",
        "coral",
        "cornflowerblue",
        "cornsilk",
        "crimson",
        "cyan",
        "darkblue",
        "darkcyan",
        "darkgoldenrod",
        "darkgray",
        "darkgreen",
        "darkkhaki",
        "darkmagenta",
        "darkolivegreen",
        "darkorange",
        "darkorchid",
        "darkred",
        "darksalmon",
        "darkseagreen",
        "darkslateblue",
        "darkslategray",
        "darkturquoise",
        "darkviolet",
        "deeppink",
        "deepskyblue",
        "dimgray",
        "dodgerblue",
        "firebrick",
        "floralwhite",
        "forestgreen",
        "fuchsia",
        "gainsboro",
        "ghostwhite",
        "gold",
        "goldenrod",
        "gray",
        "green",
        "greenyellow",
        "honeydew",
        "hotpink",
        "indianred",
        "indigo",
        "ivory",
        "khaki",
        "lavender",
        "lavenderblush",
        "lawngreen",
        "lemonchiffon",
        "lightblue",
        "lightcoral",
        "lightcyan",
        "lightgoldenrodyellow",
        "lightgreen",
        "lightgrey",
        "lightpink",
        "lightsalmon",
        "lightseagreen",
        "lightskyblue",
        "lightslategray",
        "lightsteelblue",
        "lightyellow",
        "lime",
        "limegreen",
        "linen",
        "magenta",
        "maroon",
        "mediumaquamarine",
        "mediumblue",
        "mediumorchid",
        "mediumpurple",
        "mediumseagreen",
        "mediumslateblue",
        "mediumspringgreen",
        "mediumturquoise",
        "mediumvioletred",
        "midnightblue",
        "mintcream",
        "mistyrose",
        "moccasin",
        "navajowhite",
        "navy",
        "navyblue",
        "oldlace",
        "olive",
        "olivedrab",
        "orange",
        "orangered",
        "orchid",
        "palegoldenrod",
        "palegreen",
        "paleturquoise",
        "palevioletred",
        "papayawhip",
        "peachpuff",
        "peru",
        "pink",
        "plum",
        "powderblue",
        "purple",
        "red",
        "rosybrown",
        "royalblue",
        "saddlebrown",
        "salmon",
        "sandybrown",
        "seagreen",
        "seashell",
        "sienna",
        "silver",
        "skyblue",
        "slateblue",
        "slategray",
        "snow",
        "springgreen",
        "steelblue",
        "tan",
        "teal",
        "thistle",
        "tomato",
        "turquoise",
        "violet",
        "wheat",
        "white",
        "whitesmoke",
        "yellow",
        "yellowgreen",
        "media",
        "keyframes",
        "not",
        "and",
        "only",
        "or",
        "i",
        "s",
        "important",
        "highlight",
        "part",
        "dir",
        "local",
        "global",
        "-moz-any",
        "-webkit-any",
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
        "ltr",
        "rtl",
        "n",
        "even",
        "odd",
        "of",
        "from",
        "to",
        "var",
    ],
    literals: &[
        "CSS_STRING_LITERAL",
        "CSS_NUMBER_LITERAL",
        "CSS_CUSTOM_PROPERTY",
        "CSS_SPACE_LITERAL",
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
        "CSS_BLOCK",
        "CSS_DECLARATION",
        "CSS_DIMENSION",
        "CSS_IDENTIFIER",
        "CSS_KEYFRAMES_BLOCK",
        "CSS_KEYFRAMES_SELECTOR",
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
        "CSS_LIST_OF_COMPONENT_VALUES",
        "CSS_KEYFRAMES_SELECTOR_LIST",
        "CSS_PARAMETER_LIST",
        "CSS_DECLARATION_IMPORTANT",
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
        // At rule nodes
        "CSS_AT_RULE",
        "CSS_CHARSET_AT_RULE",
        "CSS_COLOR_PROFILE_AT_RULE",
        "CSS_COUNTER_STYLE_AT_RULE",
        "CSS_KEYFRAMES_AT_RULE",
        "CSS_KEYFRAMES_BODY",
        "CSS_MEDIA_AT_RULE",
        "CSS_MEDIA_QUERY",
        "CSS_MEDIA_QUERY_CONSEQUENT",
        "CSS_MEDIA_QUERY_FEATURE",
        "CSS_MEDIA_QUERY_FEATURE_BOOLEAN",
        "CSS_MEDIA_QUERY_FEATURE_COMPARE",
        "CSS_MEDIA_QUERY_FEATURE_PLAIN",
        "CSS_MEDIA_QUERY_FEATURE_RANGE",
        "CSS_MEDIA_QUERY_RANGE",
        "CSS_KEYFRAMES_ITEM_LIST",
        "CSS_MEDIA_QUERY_LIST",
        // Bogs nodes
        "CSS_BOGUS",
        "CSS_BOGUS_BODY",
        "CSS_BOGUS_RULE",
        "CSS_BOGUS_SELECTOR",
        "CSS_BOGUS_SUB_SELECTOR",
        "CSS_BOGUS_PSEUDO_CLASS",
        "CSS_BOGUS_PSEUDO_ELEMENT",
        "CSS_BOGUS_AT_RULE",
        "CSS_BOGUS_DECLARATION_ITEM",
        "CSS_BOGUS_COMPONENT_VALUE",
    ],
};
