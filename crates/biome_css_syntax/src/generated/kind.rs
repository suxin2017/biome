//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum CssSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    HASH,
    AMP,
    PIPE,
    PIPE2,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    COLON,
    COLON2,
    EQ,
    BANG,
    NEQ,
    MINUS,
    LTEQ,
    GTEQ,
    PLUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AT,
    DOLLAR_EQ,
    TILDE_EQ,
    CDC,
    CDO,
    MEDIA_KW,
    KEYFRAMES_KW,
    NOT_KW,
    AND_KW,
    ONLY_KW,
    OR_KW,
    I_KW,
    S_KW,
    IMPORTANT_KW,
    HIGHLIGHT_KW,
    PART_KW,
    DIR_KW,
    LOCAL_KW,
    GLOBAL_KW,
    ANY_KW,
    CURRENT_KW,
    PAST_KW,
    FUTURE_KW,
    HOST_KW,
    HOST_CONTEXT_KW,
    MATCHES_KW,
    IS_KW,
    WHERE_KW,
    HAS_KW,
    LANG_KW,
    NTH_CHILD_KW,
    NTH_LAST_CHILD_KW,
    NTH_OF_TYPE_KW,
    NTH_LAST_OF_TYPE_KW,
    NTH_COL_KW,
    NTH_LAST_COL_KW,
    CHARSET_KW,
    COLOR_PROFILE_KW,
    COUNTER_STYLE_KW,
    CONTAINER_KW,
    STYLE_KW,
    LTR_KW,
    RTL_KW,
    N_KW,
    EVEN_KW,
    ODD_KW,
    OF_KW,
    FROM_KW,
    TO_KW,
    VAR_KW,
    FONT_PALETTE_VALUES_KW,
    URL_KW,
    FONT_FACE_KW,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_CUSTOM_PROPERTY,
    CSS_SPACE_LITERAL,
    CSS_URL_VALUE_RAW_LITERAL,
    CSS_COLOR_LITERAL,
    ERROR_TOKEN,
    IDENT,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    CSS_ROOT,
    CSS_RULE_LIST,
    CSS_RULE,
    CSS_SELECTOR_LIST,
    CSS_ANY_FUNCTION,
    CSS_DECLARATION_LIST_BLOCK,
    CSS_RULE_LIST_BLOCK,
    CSS_DECLARATION,
    CSS_IDENTIFIER,
    CSS_NUMBER,
    CSS_PARAMETER,
    CSS_PERCENTAGE,
    CSS_RATIO,
    CSS_SIMPLE_FUNCTION,
    CSS_STRING,
    CSS_VAR_FUNCTION,
    CSS_VAR_FUNCTION_VALUE,
    CSS_ATTRIBUTE_LIST,
    CSS_DECLARATION_LIST,
    CSS_COMPONENT_VALUE_LIST,
    CSS_PARAMETER_LIST,
    CSS_DECLARATION_IMPORTANT,
    CSS_UNIT,
    CSS_PERCENT_DIMENSION,
    CSS_REGULAR_DIMENSION,
    CSS_NAMESPACE,
    CSS_NAMED_NAMESPACE_PREFIX,
    CSS_UNIVERSAL_NAMESPACE_PREFIX,
    CSS_ANY_SELECTOR_LIST,
    CSS_COMPLEX_SELECTOR,
    CSS_COMPOUND_SELECTOR,
    CSS_SUB_SELECTOR_LIST,
    CSS_ID_SELECTOR,
    CSS_CLASS_SELECTOR,
    CSS_TYPE_SELECTOR,
    CSS_UNIVERSAL_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS,
    CSS_PSEUDO_ELEMENT_SELECTOR,
    CSS_PSEUDO_ELEMENT_IDENTIFIER,
    CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR,
    CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER,
    CSS_PSEUDO_CLASS_IDENTIFIER,
    CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER,
    CSS_PSEUDO_CLASS_FUNCTION_SELECTOR,
    CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR,
    CSS_COMPOUND_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST,
    CSS_RELATIVE_SELECTOR_LIST,
    CSS_RELATIVE_SELECTOR,
    CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST,
    CSS_PSEUDO_VALUE_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_NTH,
    CSS_PSEUDO_CLASS_NTH_SELECTOR,
    CSS_PSEUDO_CLASS_NTH,
    CSS_PSEUDO_CLASS_NTH_NUMBER,
    CSS_PSEUDO_CLASS_NTH_IDENTIFIER,
    CSS_NTH_OFFSET,
    CSS_PSEUDO_CLASS_OF_NTH_SELECTOR,
    CSS_ATTRIBUTE_SELECTOR,
    CSS_ATTRIBUTE,
    CSS_ATTRIBUTE_NAME,
    CSS_ATTRIBUTE_MATCHER,
    CSS_ATTRIBUTE_MATCHER_VALUE,
    CSS_PARENTHESIZED_EXPRESSION,
    CSS_LIST_OF_COMPONENT_VALUES_EXPRESS,
    CSS_BINARY_EXPRESS,
    CSS_URL_VALUE_RAW,
    CSS_URL_FUNCTION,
    CSS_COLOR,
    CSS_AT_RULE,
    CSS_CHARSET_AT_RULE,
    CSS_COLOR_PROFILE_AT_RULE,
    CSS_COUNTER_STYLE_AT_RULE,
    CSS_CONTAINER_AT_RULE,
    CSS_CONTAINER_NOT_QUERY,
    CSS_CONTAINER_AND_QUERY,
    CSS_CONTAINER_OR_QUERY,
    CSS_CONTAINER_QUERY_IN_PARENS,
    CSS_CONTAINER_STYLE_QUERY_IN_PARENS,
    CSS_CONTAINER_SIZE_FEATURE_IN_PARENS,
    CSS_CONTAINER_STYLE_NOT_QUERY,
    CSS_CONTAINER_STYLE_AND_QUERY,
    CSS_CONTAINER_STYLE_OR_QUERY,
    CSS_CONTAINER_STYLE_IN_PARENS,
    CSS_FONT_FACE_AT_RULE,
    CSS_FONT_PALETTE_VALUES_AT_RULE,
    CSS_KEYFRAMES_AT_RULE,
    CSS_KEYFRAMES_BODY,
    CSS_MEDIA_AT_RULE,
    CSS_MEDIA_QUERY_LIST,
    CSS_MEDIA_QUERY,
    CSS_MEDIA_CONDITION_QUERY,
    CSS_MEDIA_TYPE_QUERY,
    CSS_MEDIA_AND_TYPE_QUERY,
    CSS_MEDIA_TYPE,
    CSS_MEDIA_NOT_CONDITION,
    CSS_MEDIA_AND_CONDITION,
    CSS_MEDIA_OR_CONDITION,
    CSS_MEDIA_CONDITION_IN_PARENS,
    CSS_MEDIA_FEATURE_IN_PARENS,
    CSS_QUERY_FEATURE_PLAIN,
    CSS_QUERY_FEATURE_BOOLEAN,
    CSS_QUERY_FEATURE_RANGE,
    CSS_QUERY_FEATURE_REVERSE_RANGE,
    CSS_QUERY_FEATURE_RANGE_INTERVAL,
    CSS_QUERY_FEATURE_RANGE_COMPARISON,
    CSS_KEYFRAMES_BLOCK,
    CSS_KEYFRAMES_ITEM_LIST,
    CSS_KEYFRAMES_ITEM,
    CSS_KEYFRAMES_IDENT_SELECTOR,
    CSS_KEYFRAMES_PERCENTAGE_SELECTOR,
    CSS_KEYFRAMES_SELECTOR_LIST,
    CSS_BOGUS,
    CSS_BOGUS_BLOCK,
    CSS_BOGUS_KEYFRAMES_ITEM,
    CSS_BOGUS_RULE,
    CSS_BOGUS_SELECTOR,
    CSS_BOGUS_SUB_SELECTOR,
    CSS_BOGUS_PSEUDO_CLASS,
    CSS_BOGUS_PSEUDO_ELEMENT,
    CSS_BOGUS_AT_RULE,
    CSS_BOGUS_DECLARATION_ITEM,
    CSS_BOGUS_COMPONENT_VALUE,
    CSS_BOGUS_PARAMETER,
    CSS_BOGUS_MEDIA_QUERY,
    #[doc(hidden)]
    __LAST,
}
use self::CssSyntaxKind::*;
impl CssSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | HASH | AMP | PIPE | PIPE2 | PLUS | STAR | SLASH
            | CARET | PERCENT | DOT | COLON | COLON2 | EQ | BANG | NEQ | MINUS | LTEQ | GTEQ
            | PLUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AT | DOLLAR_EQ
            | TILDE_EQ | CDC | CDO => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            CSS_STRING_LITERAL
            | CSS_NUMBER_LITERAL
            | CSS_CUSTOM_PROPERTY
            | CSS_SPACE_LITERAL
            | CSS_URL_VALUE_RAW_LITERAL
            | CSS_COLOR_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            CSS_RULE_LIST
            | CSS_SELECTOR_LIST
            | CSS_ATTRIBUTE_LIST
            | CSS_DECLARATION_LIST
            | CSS_COMPONENT_VALUE_LIST
            | CSS_PARAMETER_LIST
            | CSS_ANY_SELECTOR_LIST
            | CSS_SUB_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
            | CSS_COMPOUND_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
            | CSS_RELATIVE_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
            | CSS_PSEUDO_VALUE_LIST
            | CSS_MEDIA_QUERY_LIST
            | CSS_KEYFRAMES_ITEM_LIST
            | CSS_KEYFRAMES_SELECTOR_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<CssSyntaxKind> {
        let kw = match ident {
            "media" => MEDIA_KW,
            "keyframes" => KEYFRAMES_KW,
            "not" => NOT_KW,
            "and" => AND_KW,
            "only" => ONLY_KW,
            "or" => OR_KW,
            "i" => I_KW,
            "s" => S_KW,
            "important" => IMPORTANT_KW,
            "highlight" => HIGHLIGHT_KW,
            "part" => PART_KW,
            "dir" => DIR_KW,
            "local" => LOCAL_KW,
            "global" => GLOBAL_KW,
            "any" => ANY_KW,
            "current" => CURRENT_KW,
            "past" => PAST_KW,
            "future" => FUTURE_KW,
            "host" => HOST_KW,
            "host_context" => HOST_CONTEXT_KW,
            "matches" => MATCHES_KW,
            "is" => IS_KW,
            "where" => WHERE_KW,
            "has" => HAS_KW,
            "lang" => LANG_KW,
            "nth_child" => NTH_CHILD_KW,
            "nth_last_child" => NTH_LAST_CHILD_KW,
            "nth_of_type" => NTH_OF_TYPE_KW,
            "nth_last_of_type" => NTH_LAST_OF_TYPE_KW,
            "nth_col" => NTH_COL_KW,
            "nth_last_col" => NTH_LAST_COL_KW,
            "charset" => CHARSET_KW,
            "color_profile" => COLOR_PROFILE_KW,
            "counter_style" => COUNTER_STYLE_KW,
            "container" => CONTAINER_KW,
            "style" => STYLE_KW,
            "ltr" => LTR_KW,
            "rtl" => RTL_KW,
            "n" => N_KW,
            "even" => EVEN_KW,
            "odd" => ODD_KW,
            "of" => OF_KW,
            "from" => FROM_KW,
            "to" => TO_KW,
            "var" => VAR_KW,
            "font_palette_values" => FONT_PALETTE_VALUES_KW,
            "url" => URL_KW,
            "font_face" => FONT_FACE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            TILDE => "~",
            HASH => "#",
            AMP => "&",
            PIPE => "|",
            PIPE2 => "||",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            DOT => ".",
            COLON => ":",
            COLON2 => "::",
            EQ => "=",
            BANG => "!",
            NEQ => "!=",
            MINUS => "-",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            PIPEEQ => "|=",
            AMPEQ => "&=",
            CARETEQ => "^=",
            SLASHEQ => "/=",
            STAREQ => "*=",
            PERCENTEQ => "%=",
            AT => "@",
            DOLLAR_EQ => "$=",
            TILDE_EQ => "~=",
            CDC => "-->",
            CDO => "<!--",
            MEDIA_KW => "media",
            KEYFRAMES_KW => "keyframes",
            NOT_KW => "not",
            AND_KW => "and",
            ONLY_KW => "only",
            OR_KW => "or",
            I_KW => "i",
            S_KW => "s",
            IMPORTANT_KW => "important",
            HIGHLIGHT_KW => "highlight",
            PART_KW => "part",
            DIR_KW => "dir",
            LOCAL_KW => "local",
            GLOBAL_KW => "global",
            ANY_KW => "any",
            CURRENT_KW => "current",
            PAST_KW => "past",
            FUTURE_KW => "future",
            HOST_KW => "host",
            HOST_CONTEXT_KW => "host_context",
            MATCHES_KW => "matches",
            IS_KW => "is",
            WHERE_KW => "where",
            HAS_KW => "has",
            LANG_KW => "lang",
            NTH_CHILD_KW => "nth_child",
            NTH_LAST_CHILD_KW => "nth_last_child",
            NTH_OF_TYPE_KW => "nth_of_type",
            NTH_LAST_OF_TYPE_KW => "nth_last_of_type",
            NTH_COL_KW => "nth_col",
            NTH_LAST_COL_KW => "nth_last_col",
            CHARSET_KW => "charset",
            COLOR_PROFILE_KW => "color_profile",
            COUNTER_STYLE_KW => "counter_style",
            CONTAINER_KW => "container",
            STYLE_KW => "style",
            LTR_KW => "ltr",
            RTL_KW => "rtl",
            N_KW => "n",
            EVEN_KW => "even",
            ODD_KW => "odd",
            OF_KW => "of",
            FROM_KW => "from",
            TO_KW => "to",
            VAR_KW => "var",
            FONT_PALETTE_VALUES_KW => "font_palette_values",
            URL_KW => "url",
            FONT_FACE_KW => "font_face",
            CSS_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: CssSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: CssSyntaxKind :: COMMA } ; ['('] => { $ crate :: CssSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: CssSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: CssSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: CssSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: CssSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: CssSyntaxKind :: R_BRACK } ; [<] => { $ crate :: CssSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: CssSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: CssSyntaxKind :: TILDE } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; [&] => { $ crate :: CssSyntaxKind :: AMP } ; [|] => { $ crate :: CssSyntaxKind :: PIPE } ; [||] => { $ crate :: CssSyntaxKind :: PIPE2 } ; [+] => { $ crate :: CssSyntaxKind :: PLUS } ; [*] => { $ crate :: CssSyntaxKind :: STAR } ; [/] => { $ crate :: CssSyntaxKind :: SLASH } ; [^] => { $ crate :: CssSyntaxKind :: CARET } ; [%] => { $ crate :: CssSyntaxKind :: PERCENT } ; [.] => { $ crate :: CssSyntaxKind :: DOT } ; [:] => { $ crate :: CssSyntaxKind :: COLON } ; [::] => { $ crate :: CssSyntaxKind :: COLON2 } ; [=] => { $ crate :: CssSyntaxKind :: EQ } ; [!] => { $ crate :: CssSyntaxKind :: BANG } ; [!=] => { $ crate :: CssSyntaxKind :: NEQ } ; [-] => { $ crate :: CssSyntaxKind :: MINUS } ; [<=] => { $ crate :: CssSyntaxKind :: LTEQ } ; [>=] => { $ crate :: CssSyntaxKind :: GTEQ } ; [+=] => { $ crate :: CssSyntaxKind :: PLUSEQ } ; [|=] => { $ crate :: CssSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: CssSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: CssSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: CssSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: CssSyntaxKind :: STAREQ } ; [%=] => { $ crate :: CssSyntaxKind :: PERCENTEQ } ; [@] => { $ crate :: CssSyntaxKind :: AT } ; ["$="] => { $ crate :: CssSyntaxKind :: DOLLAR_EQ } ; [~=] => { $ crate :: CssSyntaxKind :: TILDE_EQ } ; [-->] => { $ crate :: CssSyntaxKind :: CDC } ; [<!--] => { $ crate :: CssSyntaxKind :: CDO } ; [media] => { $ crate :: CssSyntaxKind :: MEDIA_KW } ; [keyframes] => { $ crate :: CssSyntaxKind :: KEYFRAMES_KW } ; [not] => { $ crate :: CssSyntaxKind :: NOT_KW } ; [and] => { $ crate :: CssSyntaxKind :: AND_KW } ; [only] => { $ crate :: CssSyntaxKind :: ONLY_KW } ; [or] => { $ crate :: CssSyntaxKind :: OR_KW } ; [i] => { $ crate :: CssSyntaxKind :: I_KW } ; [s] => { $ crate :: CssSyntaxKind :: S_KW } ; [important] => { $ crate :: CssSyntaxKind :: IMPORTANT_KW } ; [highlight] => { $ crate :: CssSyntaxKind :: HIGHLIGHT_KW } ; [part] => { $ crate :: CssSyntaxKind :: PART_KW } ; [dir] => { $ crate :: CssSyntaxKind :: DIR_KW } ; [local] => { $ crate :: CssSyntaxKind :: LOCAL_KW } ; [global] => { $ crate :: CssSyntaxKind :: GLOBAL_KW } ; [any] => { $ crate :: CssSyntaxKind :: ANY_KW } ; [current] => { $ crate :: CssSyntaxKind :: CURRENT_KW } ; [past] => { $ crate :: CssSyntaxKind :: PAST_KW } ; [future] => { $ crate :: CssSyntaxKind :: FUTURE_KW } ; [host] => { $ crate :: CssSyntaxKind :: HOST_KW } ; [host_context] => { $ crate :: CssSyntaxKind :: HOST_CONTEXT_KW } ; [matches] => { $ crate :: CssSyntaxKind :: MATCHES_KW } ; [is] => { $ crate :: CssSyntaxKind :: IS_KW } ; [where] => { $ crate :: CssSyntaxKind :: WHERE_KW } ; [has] => { $ crate :: CssSyntaxKind :: HAS_KW } ; [lang] => { $ crate :: CssSyntaxKind :: LANG_KW } ; [nth_child] => { $ crate :: CssSyntaxKind :: NTH_CHILD_KW } ; [nth_last_child] => { $ crate :: CssSyntaxKind :: NTH_LAST_CHILD_KW } ; [nth_of_type] => { $ crate :: CssSyntaxKind :: NTH_OF_TYPE_KW } ; [nth_last_of_type] => { $ crate :: CssSyntaxKind :: NTH_LAST_OF_TYPE_KW } ; [nth_col] => { $ crate :: CssSyntaxKind :: NTH_COL_KW } ; [nth_last_col] => { $ crate :: CssSyntaxKind :: NTH_LAST_COL_KW } ; [charset] => { $ crate :: CssSyntaxKind :: CHARSET_KW } ; [color_profile] => { $ crate :: CssSyntaxKind :: COLOR_PROFILE_KW } ; [counter_style] => { $ crate :: CssSyntaxKind :: COUNTER_STYLE_KW } ; [container] => { $ crate :: CssSyntaxKind :: CONTAINER_KW } ; [style] => { $ crate :: CssSyntaxKind :: STYLE_KW } ; [ltr] => { $ crate :: CssSyntaxKind :: LTR_KW } ; [rtl] => { $ crate :: CssSyntaxKind :: RTL_KW } ; [n] => { $ crate :: CssSyntaxKind :: N_KW } ; [even] => { $ crate :: CssSyntaxKind :: EVEN_KW } ; [odd] => { $ crate :: CssSyntaxKind :: ODD_KW } ; [of] => { $ crate :: CssSyntaxKind :: OF_KW } ; [from] => { $ crate :: CssSyntaxKind :: FROM_KW } ; [to] => { $ crate :: CssSyntaxKind :: TO_KW } ; [var] => { $ crate :: CssSyntaxKind :: VAR_KW } ; [font_palette_values] => { $ crate :: CssSyntaxKind :: FONT_PALETTE_VALUES_KW } ; [url] => { $ crate :: CssSyntaxKind :: URL_KW } ; [font_face] => { $ crate :: CssSyntaxKind :: FONT_FACE_KW } ; [ident] => { $ crate :: CssSyntaxKind :: IDENT } ; [EOF] => { $ crate :: CssSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: CssSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; }
