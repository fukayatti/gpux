use gpui::{
    Bounds, Global, InteractiveElement, IntoElement, Pixels, Point, Styled, TextLayout, actions,
    div, prelude::*, px,
};

actions!(global_selection, [Copy, Cut, Paste, SelectAll]);

#[derive(Default)]
pub struct GlobalSelectionState {
    pub start: Option<Point<Pixels>>,
    pub end: Option<Point<Pixels>>,
    pub is_dragging: bool,
    pub frame_nodes: Vec<RegisteredTextNode>,
}

impl Global for GlobalSelectionState {}

#[derive(Clone)]
pub struct RegisteredTextNode {
    pub bounds: Bounds<Pixels>,
    pub text: String,
    pub layout: TextLayout,
}

pub fn reading_order_less(a: Point<Pixels>, b: Point<Pixels>) -> bool {
    if a.y < b.y - px(10.) {
        true
    } else if a.y > b.y + px(10.) {
        false
    } else {
        a.x < b.x
    }
}

pub fn get_selection_range(state: &GlobalSelectionState) -> Option<(Point<Pixels>, Point<Pixels>)> {
    if let (Some(s), Some(e)) = (state.start, state.end) {
        if reading_order_less(s, e) {
            Some((s, e))
        } else {
            Some((e, s))
        }
    } else {
        None
    }
}

pub fn selection_root(
    focus_handle: &gpui::FocusHandle,
    child: impl IntoElement,
) -> impl IntoElement {
    div()
        .id("selection-root")
        .track_focus(focus_handle)
        .size_full()
        .on_key_down(|event, _window, _cx| {
            println!("Key down in global_selection: {:?}", event.keystroke);
        })
        .on_mouse_down(gpui::MouseButton::Left, {
            let focus_handle = focus_handle.clone();
            move |event, window, cx| {
                window.focus(&focus_handle);
                cx.update_global::<GlobalSelectionState, _>(|state, _cx| {
                    state.start = Some(event.position);
                    state.end = Some(event.position);
                    state.is_dragging = true;
                });
                window.refresh();
            }
        })
        .on_mouse_move(|event, window, cx| {
            let is_dragging = cx.global::<GlobalSelectionState>().is_dragging;
            if is_dragging {
                cx.update_global::<GlobalSelectionState, _>(|state, _cx| {
                    state.end = Some(event.position);
                });
                window.refresh();
            }
        })
        .on_mouse_up(gpui::MouseButton::Left, |_, window, cx| {
            cx.update_global::<GlobalSelectionState, _>(|state, _cx| {
                state.is_dragging = false;
            });
            window.refresh();
        })
        .on_scroll_wheel(|_event, window, cx| {
            cx.update_global::<GlobalSelectionState, _>(|state, _cx| {
                // Clear selection when scrolling to prevent it from sticking to the screen
                state.start = None;
                state.end = None;
                state.is_dragging = false;
            });
            window.refresh();
        })
        .on_action(|_: &Copy, _window, cx| {
            let state = cx.global::<GlobalSelectionState>();
            println!("Copy action triggered globally!");
            if let Some((start, end)) = get_selection_range(state) {
                println!("Selection range: {:?} to {:?}", start, end);
                let mut all_selected_text = String::new();

                let mut nodes = state.frame_nodes.clone();
                nodes.sort_by(|a, b| {
                    if reading_order_less(a.bounds.origin, b.bounds.origin) {
                        std::cmp::Ordering::Less
                    } else if reading_order_less(b.bounds.origin, a.bounds.origin) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });

                for node in nodes {
                    let mut sel_start_idx = 0;
                    let mut sel_end_idx = node.text.len();

                    let start_idx = node.layout.index_for_position(start);
                    let end_idx = node.layout.index_for_position(end);

                    if let Ok(idx) = start_idx {
                        sel_start_idx = idx;
                    } else if let Err(idx) = start_idx {
                        if reading_order_less(start, node.bounds.origin) {
                            sel_start_idx = 0;
                        } else {
                            sel_start_idx = idx;
                        }
                    }

                    if let Ok(idx) = end_idx {
                        sel_end_idx = idx;
                    } else if let Err(idx) = end_idx {
                        if reading_order_less(node.bounds.origin, end) {
                            sel_end_idx = node.text.len();
                        } else {
                            sel_end_idx = idx;
                        }
                    }

                    if sel_start_idx < sel_end_idx {
                        let text_bytes = node.text.as_bytes();
                        let end_idx = sel_end_idx.min(text_bytes.len());
                        if let Ok(slice) = std::str::from_utf8(&text_bytes[sel_start_idx..end_idx])
                        {
                            all_selected_text.push_str(slice);
                            all_selected_text.push(' ');
                        }
                    }
                }

                if !all_selected_text.is_empty() {
                    let text_to_copy = all_selected_text.trim_end().to_string();
                    println!("Writing to clipboard: '{}'", text_to_copy);
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(text_to_copy));
                } else {
                    println!("No text selected to copy.");
                }
            } else {
                println!("No selection range found.");
            }
        })
        .on_action(|_: &SelectAll, window, cx| {
            cx.update_global::<GlobalSelectionState, _>(|state, _cx| {
                if let Some(first) = state.frame_nodes.first() {
                    if let Some(last) = state.frame_nodes.last() {
                        state.start = Some(first.bounds.origin);
                        state.end = Some(Point {
                            x: last.bounds.right(),
                            y: last.bounds.bottom(),
                        });
                    }
                }
            });
            window.refresh();
        })
        .child(
            gpui::canvas(
                |bounds, _, _| bounds,
                |_, _, _, cx| {
                    cx.update_global::<GlobalSelectionState, _>(|state, _| {
                        state.frame_nodes.clear();
                    });
                },
            )
            .absolute()
            .w_full()
            .h_full(),
        )
        .child(child)
}
