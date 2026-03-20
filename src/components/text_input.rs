use gpui::{
    App, Bounds, Context, ElementInputHandler, EntityInputHandler, FocusHandle, Focusable,
    InteractiveElement, IntoElement, KeyContext, ParentElement, Pixels, Point, Render, Styled,
    UTF16Selection, Window, actions, div, prelude::*,
};
use std::ops::Range;

// Define custom actions for keyboard navigation
actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll
    ]
);

pub struct TextInput {
    focus_handle: FocusHandle,
    pub text: String,
    cursor_offset: usize,               // Current cursor position in characters
    selection_anchor: Option<usize>,    // Anchor for text selection
    marked_range: Option<Range<usize>>, // IME composition range
    is_dragging: bool,
    bounds: std::rc::Rc<std::cell::Cell<Option<Bounds<Pixels>>>>,
}

impl TextInput {
    pub fn new(cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            text: String::new(),
            cursor_offset: 0,
            selection_anchor: None,
            marked_range: None,
            is_dragging: false,
            bounds: std::rc::Rc::new(std::cell::Cell::new(None)),
        }
    }

    fn char_index_for_x(&self, x: gpui::Pixels, window: &mut Window) -> usize {
        if self.text.is_empty() {
            return 0;
        }
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let run = text_style.to_run(self.text.len());
        let text_system = window.text_system();
        let layout = text_system.layout_line(&self.text, font_size, &[run], None);
        let byte_index = layout.index_for_x(x).unwrap_or(self.text.len());
        self.text[..byte_index.min(self.text.len())].chars().count()
    }

    fn handle_mouse_down(
        &mut self,
        event: &gpui::MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.bounds.get() {
            let x = event.position.x - bounds.left() - gpui::px(8.);
            let index = self.char_index_for_x(x.max(gpui::px(0.)), window);
            self.cursor_offset = index;
            self.selection_anchor = None;
            self.is_dragging = true;
            cx.notify();
        }
    }

    fn handle_mouse_move(
        &mut self,
        event: &gpui::MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            if let Some(bounds) = self.bounds.get() {
                let x = event.position.x - bounds.left() - gpui::px(8.);
                let index = self.char_index_for_x(x.max(gpui::px(0.)), window);
                if self.selection_anchor.is_none() && index != self.cursor_offset {
                    self.selection_anchor = Some(self.cursor_offset);
                }
                self.cursor_offset = index;
                cx.notify();
            }
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &gpui::MouseUpEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.is_dragging = false;
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.marked_range.is_some() {
            return;
        }
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_offset {
                self.cursor_offset = self.cursor_offset.min(anchor);
                self.selection_anchor = None;
                cx.notify();
                return;
            }
        }
        self.selection_anchor = None;
        if self.cursor_offset > 0 {
            self.cursor_offset -= 1;
            cx.notify();
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.marked_range.is_some() {
            return;
        }
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_offset {
                self.cursor_offset = self.cursor_offset.max(anchor);
                self.selection_anchor = None;
                cx.notify();
                return;
            }
        }
        self.selection_anchor = None;
        let chars_count = self.text.chars().count();
        if self.cursor_offset < chars_count {
            self.cursor_offset += 1;
            cx.notify();
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        if self.marked_range.is_some() {
            return;
        }
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_offset);
        }
        if self.cursor_offset > 0 {
            self.cursor_offset -= 1;
            cx.notify();
        }
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        if self.marked_range.is_some() {
            return;
        }
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_offset);
        }
        let chars_count = self.text.chars().count();
        if self.cursor_offset < chars_count {
            self.cursor_offset += 1;
            cx.notify();
        }
    }

    fn select_all(&mut self, _: &crate::system::SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        if self.text.is_empty() {
            return;
        }
        self.selection_anchor = Some(0);
        self.cursor_offset = self.text.chars().count();
        cx.notify();
        cx.stop_propagation();
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.marked_range.is_some() {
            return;
        }
        let mut chars: Vec<char> = self.text.chars().collect();
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_offset {
                let start = self.cursor_offset.min(anchor);
                let end = self.cursor_offset.max(anchor);
                chars.drain(start..end);
                self.text = chars.into_iter().collect();
                self.cursor_offset = start;
                self.selection_anchor = None;
                cx.notify();
                return;
            }
        }
        self.selection_anchor = None;
        if self.cursor_offset > 0 {
            chars.remove(self.cursor_offset - 1);
            self.text = chars.into_iter().collect();
            self.cursor_offset -= 1;
            cx.notify();
        }
    }

    fn copy(&mut self, _: &gpux::system::Copy, _: &mut Window, cx: &mut Context<Self>) {
        println!("Copy triggered in AdvancedTextInput!");
        if let Some(anchor) = self.selection_anchor {
            let start = self.cursor_offset.min(anchor);
            let end = self.cursor_offset.max(anchor);
            let chars: Vec<char> = self.text.chars().collect();
            let selected_text: String = chars[start..end].iter().collect();
            if !selected_text.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(selected_text));
            }
        }
        cx.stop_propagation();
    }

    fn cut(&mut self, _: &gpux::system::Cut, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(anchor) = self.selection_anchor {
            let start = self.cursor_offset.min(anchor);
            let end = self.cursor_offset.max(anchor);
            let mut chars: Vec<char> = self.text.chars().collect();
            let selected_text: String = chars[start..end].iter().collect();
            if !selected_text.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(selected_text));
                chars.drain(start..end);
                self.text = chars.into_iter().collect();
                self.cursor_offset = start;
                self.selection_anchor = None;
                cx.notify();
            }
        }
        cx.stop_propagation();
    }

    fn paste(&mut self, _: &gpux::system::Paste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(clipboard) = cx.read_from_clipboard() {
            if let Some(text_to_paste) = clipboard.text() {
                if !text_to_paste.is_empty() {
                    let mut chars: Vec<char> = self.text.chars().collect();
                    let start = if let Some(anchor) = self.selection_anchor {
                        let s = self.cursor_offset.min(anchor);
                        let e = self.cursor_offset.max(anchor);
                        chars.drain(s..e);
                        s
                    } else {
                        self.cursor_offset
                    };

                    let paste_chars: Vec<char> = text_to_paste.chars().collect();
                    chars.splice(start..start, paste_chars.iter().copied());
                    self.text = chars.into_iter().collect();
                    self.cursor_offset = start + paste_chars.len();
                    self.selection_anchor = None;
                    cx.notify();
                }
            }
        }
        cx.stop_propagation();
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// By implementing EntityInputHandler, GPUI natively passes IME events, key strokes, and text replacements to us.
impl EntityInputHandler for TextInput {
    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if let Some(anchor) = self.selection_anchor {
            let reversed = self.cursor_offset < anchor;
            let start = self.cursor_offset.min(anchor);
            let end = self.cursor_offset.max(anchor);
            Some(UTF16Selection {
                range: start..end,
                reversed,
            })
        } else {
            Some(UTF16Selection {
                range: self.cursor_offset..self.cursor_offset,
                reversed: false,
            })
        }
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range.clone()
    }

    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let chars: Vec<char> = self.text.chars().collect();
        if range_utf16.end <= chars.len() {
            Some(chars[range_utf16].iter().collect())
        } else {
            None
        }
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!(
            "replace_text_in_range: {:?} '{}', current marked: {:?}",
            replacement_range, text, self.marked_range
        );
        let mut chars: Vec<char> = self.text.chars().collect();
        let range = replacement_range
            .or(self.marked_range.clone())
            .unwrap_or_else(|| {
                if let Some(anchor) = self.selection_anchor {
                    if anchor != self.cursor_offset {
                        return self.cursor_offset.min(anchor)..self.cursor_offset.max(anchor);
                    }
                }
                self.cursor_offset..self.cursor_offset
            });

        let start = range.start.min(chars.len());
        let end = range.end.min(chars.len()).max(start);

        // Replace the text
        chars.splice(start..end, text.chars());
        self.text = chars.into_iter().collect();

        // Update the cursor position to be at the end of the newly inserted text
        self.cursor_offset = start + text.chars().count();
        self.selection_anchor = None;
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!(
            "replace_and_mark_text_in_range: {:?} '{}' sel: {:?}, current marked: {:?}",
            replacement_range, new_text, new_selected_range, self.marked_range
        );
        let mut chars: Vec<char> = self.text.chars().collect();

        // When replacing and marking, if replacement_range is explicitly provided, we should use it.
        // Otherwise, if there is a marked_range, we use that.
        // Otherwise, we use the cursor offset.
        let range = replacement_range
            .or(self.marked_range.clone())
            .unwrap_or_else(|| {
                if let Some(anchor) = self.selection_anchor {
                    if anchor != self.cursor_offset {
                        return self.cursor_offset.min(anchor)..self.cursor_offset.max(anchor);
                    }
                }
                self.cursor_offset..self.cursor_offset
            });

        let start = range.start.min(chars.len());
        let end = range.end.min(chars.len()).max(start);

        chars.splice(start..end, new_text.chars());
        self.text = chars.into_iter().collect();

        let marked_start = start;
        let marked_end = marked_start + new_text.chars().count();

        // This denotes the text that is currently being composed by the IME
        self.marked_range = Some(marked_start..marked_end);
        self.selection_anchor = None;

        if let Some(sel) = new_selected_range {
            self.cursor_offset = marked_start + sel.start;
        } else {
            self.cursor_offset = marked_end;
        }
        cx.notify();
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.marked_range = None;
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        // Here we ideally compute the visual bounds of the character range so the OS can position the IME popup correctly.
        // For simplicity, we just return the element's overall bounds.
        Some(element_bounds)
    }

    fn character_index_for_point(
        &mut self,
        point: Point<gpui::Pixels>,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        if let Some(bounds) = self.bounds.get() {
            let x = point.x - bounds.left() - gpui::px(8.);
            Some(self.char_index_for_x(x.max(gpui::px(0.)), window))
        } else {
            None
        }
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);

        let is_focused_val = is_focused;
        let focus_handle = self.focus_handle.clone();
        let entity = cx.entity().clone();

        let chars: Vec<char> = self.text.chars().collect();
        let sel_start = self
            .selection_anchor
            .unwrap_or(self.cursor_offset)
            .min(self.cursor_offset);
        let sel_end = self
            .selection_anchor
            .unwrap_or(self.cursor_offset)
            .max(self.cursor_offset);

        let before_sel: String = chars[..sel_start].iter().collect();
        let selected_text: String = chars[sel_start..sel_end].iter().collect();
        let after_sel: String = chars[sel_end..].iter().collect();

        // Helper to visualize marked text (IME composition)
        let marked_text_display = if let Some(marked) = &self.marked_range {
            let start = marked.start.min(chars.len());
            let end = marked.end.min(chars.len());
            chars[start..end].iter().collect::<String>()
        } else {
            String::new()
        };

        let bounds_cell = self.bounds.clone();

        div()
            .id("advanced-input")
            .track_focus(&self.focus_handle)
            .key_context("AdvancedInput")
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, event, window, cx| {
                    window.focus(&this.focus_handle);
                    this.handle_mouse_down(event, window, cx);
                    cx.stop_propagation();
                }),
            )
            .on_mouse_move(cx.listener(|this, event, window, cx| {
                this.handle_mouse_move(event, window, cx);
            }))
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|this, event, window, cx| {
                    this.handle_mouse_up(event, window, cx);
                }),
            )
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::paste))
            .flex()
            .items_center()
            .w_full()
            .h(gpui::px(40.))
            .px(gpui::px(8.))
            .border_1()
            .rounded_md()
            .border_color({
                let c: gpui::Hsla = if is_focused {
                    gpui::blue()
                } else {
                    gpui::rgba(0x888888FF).into()
                };
                c
            })
            .bg(gpui::white())
            .child(
                gpui::canvas(
                    |bounds, _, _| bounds,
                    move |bounds, _, window, cx| {
                        bounds_cell.set(Some(bounds));
                        if is_focused_val {
                            window.handle_input(
                                &focus_handle,
                                ElementInputHandler::new(bounds, entity.clone()),
                                cx,
                            );

                            // Bind actions specifically when we are focused
                            let mut key_context = KeyContext::default();
                            key_context.add("AdvancedInput");
                            window.set_key_context(key_context);
                        }
                    },
                )
                .absolute()
                .w_full()
                .h_full(),
            )
            .child(div().flex().items_center().text_color(gpui::black()).child(
                if self.text.is_empty() && !is_focused {
                    "Type something perfectly...".to_string()
                } else {
                    before_sel
                },
            ))
            // Cursor before selection (if reversed selection, and selection exists)
            .when(
                is_focused && !selected_text.is_empty() && self.cursor_offset == sel_start,
                |this| {
                    this.child(
                        div().w(gpui::px(0.)).h(gpui::px(20.)).child(
                            div()
                                .absolute()
                                .w(gpui::px(2.))
                                .h(gpui::px(20.))
                                .ml(gpui::px(-1.))
                                .bg(gpui::blue()),
                        ),
                    )
                },
            )
            // Selected text background
            .when(!selected_text.is_empty(), |this| {
                this.child(
                    div()
                        .bg(gpui::rgba(0xADD8E6FF))
                        .text_color(gpui::black())
                        .child(selected_text.clone()),
                )
            })
            // Cursor after selection (if normal selection or no selection)
            .when(
                is_focused && (selected_text.is_empty() || self.cursor_offset == sel_end),
                |this| {
                    this.child(
                        div().w(gpui::px(0.)).h(gpui::px(20.)).child(
                            div()
                                .absolute()
                                .w(gpui::px(2.))
                                .h(gpui::px(20.))
                                .ml(gpui::px(-1.))
                                .bg(gpui::blue()),
                        ),
                    )
                },
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_color(gpui::black())
                    .child(after_sel),
            )
            // Show IME composition underneath if active
            .when(self.marked_range.is_some(), |this| {
                this.child(
                    div()
                        .absolute()
                        .top(gpui::px(35.))
                        .text_color(gpui::blue())
                        .text_size(gpui::px(12.))
                        .child(format!("IME: {}", marked_text_display)),
                )
            })
    }
}
