//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct MarkdownSyntaxFactory;
impl SyntaxFactory for MarkdownSyntaxFactory {
    type Kind = MarkdownSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            MARKDOWN_BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            MAKRDOWN_SETEXT_H1 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MAKRDOWN_SETEXT_H1.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MAKRDOWN_SETEXT_H1, children)
            }
            MAKRDOWN_SETEXT_H2 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MAKRDOWN_SETEXT_H2.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MAKRDOWN_SETEXT_H2, children)
            }
            MARKDOWN_BREAK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_BREAK_BLOCK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_BREAK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_BREAK_BLOCK, children)
            }
            MARKDOWN_BULLET_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_BULLET_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_BULLET_LIST_ITEM, children)
            }
            MARKDOWN_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![UNICODE_BOM] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyMarkdownBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_DOCUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_DOCUMENT, children)
            }
            MARKDOWN_FENCED_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_FENCED_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_FENCED_CODE_BLOCK, children)
            }
            MARKDOWN_H1 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H1.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H1, children)
            }
            MARKDOWN_H2 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H2.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H2, children)
            }
            MARKDOWN_H3 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H3.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H3, children)
            }
            MARKDOWN_H4 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H4.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H4, children)
            }
            MARKDOWN_H5 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H5.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H5, children)
            }
            MARKDOWN_H6 => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_H6.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_H6, children)
            }
            MARKDOWN_HTML_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HTML_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HTML_BLOCK, children)
            }
            MARKDOWN_HARD_LINE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_HARD_LINE_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HARD_LINE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HARD_LINE, children)
            }
            MARKDOWN_INDENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_INDENT_CHUNK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INDENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INDENT, children)
            }
            MARKDOWN_INDENT_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INDENT_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INDENT_CODE_BLOCK, children)
            }
            MARKDOWN_INLINE_CODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_CODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_CODE, children)
            }
            MARKDOWN_INLINE_EMPHASIS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_EMPHASIS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_EMPHASIS, children)
            }
            MARKDOWN_INLINE_IMAGE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_IMAGE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_IMAGE, children)
            }
            MARKDOWN_INLINE_LINK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_LINK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_LINK, children)
            }
            MARKDOWN_LINK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownString::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_LINK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_LINK_BLOCK, children)
            }
            MARKDOWN_ORDER_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_ORDER_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_ORDER_LIST_ITEM, children)
            }
            MARKDOWN_PARAGRAPH => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraphItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_PARAGRAPH.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_PARAGRAPH, children)
            }
            MARKDOWN_QUOTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyMarkdownBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_QUOTE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_QUOTE, children)
            }
            MARKDOWN_SOFT_BREAK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_SOFT_BREAK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_SOFT_BREAK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_SOFT_BREAK, children)
            }
            MARKDOWN_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_STRING_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_STRING, children)
            }
            MARKDOWN_TEXTUAL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_TEXTUAL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_TEXTUAL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_TEXTUAL, children)
            }
            MARKDOWN_BULLET_LIST => {
                Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast)
            }
            MARKDOWN_ORDER_LIST => {
                Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast)
            }
            MARKDOWN_PARAGRAPH_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, AnyMarkdownInline::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
