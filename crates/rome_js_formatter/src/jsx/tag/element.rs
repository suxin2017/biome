use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsxElement;
use rome_rowan::AstNode;

impl FormatNode for JsxElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
