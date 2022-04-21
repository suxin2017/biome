use crate::utils::sort_modifiers_by_precedence;
use crate::{join_elements, space_token, Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsPropertyModifierList;

impl Format for JsPropertyModifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(sort_modifiers_by_precedence(self))?,
        ))
    }
}
