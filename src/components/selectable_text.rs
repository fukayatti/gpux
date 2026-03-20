use gpui::{
    App, HighlightStyle, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::*, rgba,
};

use crate::system::{
    GlobalSelectionState, RegisteredTextNode, get_selection_range, reading_order_less,
};

pub struct SelectableText {
    pub text: String,
}

impl SelectableText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

pub fn text(text: impl Into<String>) -> gpui::Component<SelectableText> {
    gpui::Component::new(SelectableText::new(text))
}

impl RenderOnce for SelectableText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = cx.try_global::<GlobalSelectionState>();

        let mut sel_start_idx = 0;
        let mut sel_end_idx = 0;
        let mut has_selection = false;

        // Try to recover the exact multi-line bounds and layout computed during the previous frame's render
        if let Some(global_state) = state {
            if let Some(node) = global_state
                .frame_nodes
                .iter()
                .find(|n| n.text == self.text)
            {
                if let Some((start, end)) = get_selection_range(global_state) {
                    let start_res = node.layout.index_for_position(start);
                    if let Ok(idx) = start_res {
                        sel_start_idx = idx;
                        has_selection = true;
                    } else if let Err(idx) = start_res {
                        if reading_order_less(start, node.bounds.origin) {
                            sel_start_idx = 0;
                        } else {
                            sel_start_idx = idx;
                        }
                    }

                    let end_res = node.layout.index_for_position(end);
                    if let Ok(idx) = end_res {
                        sel_end_idx = idx;
                        has_selection = true;
                    } else if let Err(idx) = end_res {
                        if reading_order_less(node.bounds.origin, end) {
                            sel_end_idx = node.text.len();
                        } else {
                            sel_end_idx = idx;
                        }
                    }

                    // Ensure we capture if the selection fully engulfs this node
                    if reading_order_less(start, node.bounds.origin)
                        && reading_order_less(node.bounds.origin, end)
                    {
                        has_selection = true;
                    }
                }
            }
        }

        let sel_start = sel_start_idx.min(sel_end_idx);
        let sel_end = sel_start_idx.max(sel_end_idx);

        let mut styled_text = gpui::StyledText::new(self.text.clone());

        // Ensure index limits are safe within byte boundaries to prevent panic
        if has_selection && sel_start < sel_end && sel_end <= self.text.len() {
            let mut highlight = HighlightStyle::default();
            highlight.background_color = Some(rgba(0xADD8E6FF).into());
            styled_text = styled_text.with_highlights(vec![(sel_start..sel_end, highlight)]);
        }

        let layout_clone = styled_text.layout().clone();
        let text_clone = self.text.clone();

        div()
            .relative()
            .child(
                gpui::canvas(
                    |bounds, _, _| bounds,
                    move |bounds, _, _, cx| {
                        if cx.has_global::<GlobalSelectionState>() {
                            cx.update_global::<GlobalSelectionState, _>(|state, _| {
                                state.frame_nodes.push(RegisteredTextNode {
                                    bounds,
                                    text: text_clone.clone(),
                                    layout: layout_clone.clone(),
                                });
                            });
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
            .child(styled_text)
    }
}
