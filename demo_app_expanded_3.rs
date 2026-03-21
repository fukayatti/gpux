#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, Menu, MenuItem, Window,
    WindowBounds, WindowOptions, prelude::*, px, size,
};
use gpux::{
    gpui_component, icon, image, text, use_animation, use_context, use_effect,
    use_future, use_memo, use_state, view,
};
use gpux::components::{
    Backspace, Left, NextImage, Right, SelectLeft, SelectRight, SelectableText, TextInput,
};
use gpux::system::{
    Copy, Cut, GlobalSelectionState, Paste, SelectAll as GlobalSelectAll, selection_root,
};
struct MyAppState {
    theme: String,
}
impl gpui::Global for MyAppState {}
#[allow(non_snake_case)]
fn CounterPage(
    app_cx: &mut App,
    initial_value: i32,
    title: String,
    set_show_modal: std::sync::Arc<dyn Fn(bool, &mut gpui::App)>,
) -> gpui::Entity<CounterPageComponent> {
    app_cx
        .new(|_cx| CounterPageComponent {
            hooks: gpux::hooks::Hooks::new(),
            initial_value,
            title,
            set_show_modal,
        })
}
struct CounterPageComponent {
    hooks: gpux::hooks::Hooks,
    initial_value: i32,
    title: String,
    set_show_modal: std::sync::Arc<dyn Fn(bool, &mut gpui::App)>,
}
impl gpui::Render for CounterPageComponent {
    fn render(
        &mut self,
        #[allow(unused_variables)]
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.hooks.reset();
        let __entity = cx.entity().clone();
        let initial_value = self.initial_value.clone();
        let title = self.title.clone();
        let set_show_modal = self.set_show_modal.clone();
        let _ctx = cx.global::<MyAppState>();
        let (count, set_count) = {
            let (__val, __idx) = self.hooks.use_state(initial_value);
            let __entity_clone = __entity.clone();
            let __setter = move |new_val, cx: &mut gpui::App| {
                __entity_clone
                    .update(
                        cx,
                        |this, cx| {
                            this.hooks.set_state(__idx, new_val);
                            cx.notify();
                        },
                    );
            };
            (__val, __setter)
        };
        let (width, start_anim): (f32, _) = {
            let (__val, __idx) = self.hooks.use_state::<f32>(150.0f32 as f32);
            let __entity_clone = __entity.clone();
            let __animator = move |
                target_val: f32,
                duration_ms: u64,
                cx: &mut gpui::App|
            {
                let start_val = __val;
                let steps = (duration_ms / 16).max(1);
                let step_val = (target_val - start_val) / (steps as f32);
                let __entity_clone_inner = __entity_clone.clone();
                let _task: gpui::Task<()> = cx
                    .spawn(async move |mut cx| {
                        for i in 1..=steps {
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(16))
                                .await;
                            let next_val: f32 = if i == steps {
                                target_val
                            } else {
                                start_val + step_val * (i as f32)
                            };
                            let _ = cx
                                .update(|cx: &mut gpui::App| {
                                    __entity_clone_inner
                                        .update(
                                            cx,
                                            |this_ref: &mut Self, cx: &mut gpui::Context<Self>| {
                                                this_ref.hooks.set_state::<f32>(__idx, next_val);
                                                cx.notify();
                                            },
                                        )
                                });
                        }
                    });
                _task.detach();
            };
            (__val, __animator)
        };
        let text_input_view = self
            .hooks
            .check_memo(Some(()), || { cx.new(|cx| TextInput::new(cx)) });
        let bg_executor = cx.background_executor().clone();
        let manifest_dir = "/Users/fukayatti0/Projects/gpx/demo_app";
        let test_img_path = ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}/assets/test.png", manifest_dir))
        });
        let async_data = {
            let (__val_opt, __idx) = self.hooks.use_state::<Option<String>>(None);
            if self.hooks.check_effect(Some(count)) {
                self.hooks.set_state::<Option<String>>(__idx, None);
                let __entity_clone = __entity.clone();
                let _task: gpui::Task<()> = cx
                    .spawn(async move |_this, mut __cx| {
                        let __res = {
                            {
                                bg_executor
                                    .timer(std::time::Duration::from_millis(1500))
                                    .await;
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("Fetched Data for Count {0}", count),
                                    )
                                })
                            }
                        };
                        {
                            ::std::io::_print(
                                format_args!("Future completed for ID: {0}\n", __idx),
                            );
                        };
                        let _ = __cx
                            .update(|cx: &mut gpui::App| {
                                __entity_clone
                                    .update(
                                        cx,
                                        |this_ref: &mut Self, cx: &mut gpui::Context<Self>| {
                                            {
                                                ::std::io::_print(
                                                    format_args!("State updated for ID: {0}\n", __idx),
                                                );
                                            };
                                            this_ref
                                                .hooks
                                                .set_state::<Option<String>>(__idx, Some(__res));
                                            cx.notify();
                                        },
                                    )
                            });
                    });
                _task.detach();
            }
            __val_opt
        };
        if self.hooks.check_effect(Some(count)) {
            {
                {
                    ::std::io::_print(format_args!("Count changed: {0}\n", count));
                };
            }
        }
        let count_str = self
            .hooks
            .check_memo(
                Some(count),
                || {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Count: {0}", count))
                    })
                },
            );
        gpui::div()
            .id("main-scroll")
            .w_full()
            .h_full()
            .overflow_y_scroll()
            .bg(gpui::rgb(16382715u32))
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .min_h_full()
                    .items_center()
                    .p_8()
                    .gap_8()
                    .child(
                        gpui::div()
                            .w_full()
                            .max_w(gpui::px(896.0))
                            .bg(gpui::rgb(16777215u32))
                            .rounded_2xl()
                            .shadow_sm()
                            .border_1()
                            .border_color(gpui::rgb(15067115u32))
                            .p_8()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_8()
                            .when(
                                window.viewport_size().width >= gpui::px(768.0),
                                |s| s.flex_row(),
                            )
                            .child(
                                gpui::div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        gpui::div()
                                            .text_size(gpui::px(40f32))
                                            .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                            .text_color(gpui::rgb(5195493u32))
                                            .child({ text(title) }),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_size(gpui::px(16f32))
                                            .text_color(gpui::rgb(4937059u32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Experience the power of GPUI combined with React-like hooks, JSX-style macros, and automated image optimization. Building native desktop apps has never been this smooth.",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .flex_row()
                                            .gap_4()
                                            .mt_2()
                                            .child(
                                                gpui::div()
                                                    .id("increment-btn")
                                                    .bg(gpui::rgb(5195493u32))
                                                    .text_color(gpui::rgb(16777215u32))
                                                    .px_6()
                                                    .py_3()
                                                    .rounded_xl()
                                                    .font_weight(gpui::FontWeight::MEDIUM)
                                                    .cursor_pointer()
                                                    .shadow_sm()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .hover(|s| s.bg(gpui::rgb(4405450u32)))
                                                    .active(|s| s.bg(gpui::rgb(3616931u32)))
                                                    .on_click(move |
                                                        _event: &gpui::ClickEvent,
                                                        _window: &mut Window,
                                                        cx: &mut App|
                                                    set_count(count + 1, cx))
                                                    .child(
                                                        gpui::div().w_5().h_5().child({ icon(icondata::LuPlus) }),
                                                    )
                                                    .child({
                                                        text(
                                                            ::alloc::__export::must_use({
                                                                ::alloc::fmt::format(format_args!("Count: {0}", count))
                                                            }),
                                                        )
                                                    }),
                                            )
                                            .child(
                                                gpui::div()
                                                    .id("animated-box")
                                                    .bg(gpui::rgb(1096065u32))
                                                    .text_color(gpui::rgb(16777215u32))
                                                    .px_6()
                                                    .py_3()
                                                    .rounded_xl()
                                                    .font_weight(gpui::FontWeight::MEDIUM)
                                                    .cursor_pointer()
                                                    .shadow_sm()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .gap_2()
                                                    .hover(|s| s.bg(gpui::rgb(366185u32)))
                                                    .w(px(width))
                                                    .on_click(move |
                                                        _event: &gpui::ClickEvent,
                                                        _window: &mut gpui::Window,
                                                        cx: &mut gpui::App|
                                                    start_anim(300.0, 500, cx))
                                                    .child(
                                                        gpui::div().w_5().h_5().child({ icon(icondata::LuPlay) }),
                                                    )
                                                    .child(
                                                        gpui::Component::new(
                                                            gpux::components::SelectableText::new("Animate"),
                                                        ),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .id("auto-state-id")
                                    .w_64()
                                    .h_64()
                                    .rounded_2xl()
                                    .overflow_hidden()
                                    .shadow_lg()
                                    .border_4()
                                    .border_color(gpui::rgb(16777215u32))
                                    .flex_shrink_0()
                                    .child({
                                        NextImage(
                                                cx,
                                                "https://images.unsplash.com/photo-1550745165-9bc0b252726f?q=80&w=800"
                                                    .to_string(),
                                                256.0,
                                                256.0,
                                            )
                                            .into_any_element()
                                    }),
                            ),
                    )
                    .child(
                        gpui::div()
                            .w_full()
                            .max_w(gpui::px(896.0))
                            .grid()
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            .when(
                                window.viewport_size().width >= gpui::px(768.0),
                                |s| s.flex().flex_wrap(),
                            )
                            .child(
                                gpui::div()
                                    .bg(gpui::rgb(16777215u32))
                                    .p_6()
                                    .rounded_2xl()
                                    .shadow_sm()
                                    .border_1()
                                    .border_color(gpui::rgb(15067115u32))
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_size(gpui::px(18f32))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(gpui::rgb(2042167u32))
                                            .child(
                                                gpui::div()
                                                    .w_6()
                                                    .h_6()
                                                    .text_color(gpui::rgb(3900150u32))
                                                    .child({ icon(icondata::LuKeyboard) }),
                                            )
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new("Rich Input & IME"),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_color(gpui::rgb(7041664u32))
                                            .text_size(gpui::px(14f32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Full support for advanced text input and native IME integration.",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .w_full()
                                            .mt_2()
                                            .child({ text_input_view.clone() }),
                                    )
                                    .child(
                                        gpui::div()
                                            .mt_4()
                                            .pt_4()
                                            .border_t_1()
                                            .border_color(gpui::rgb(15987958u32))
                                            .child(
                                                gpui::div()
                                                    .text_size(gpui::px(14f32))
                                                    .font_weight(gpui::FontWeight::BOLD)
                                                    .text_color(gpui::rgb(2042167u32))
                                                    .mb_2()
                                                    .child(
                                                        gpui::Component::new(
                                                            gpux::components::SelectableText::new(
                                                                "File System Integrations",
                                                            ),
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                gpui::div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap_2()
                                                    .child(
                                                        gpui::div()
                                                            .flex_1()
                                                            .bg(gpui::rgb(16382715u32))
                                                            .border_2()
                                                            .border_dashed()
                                                            .border_color(gpui::rgb(13751771u32))
                                                            .rounded_xl()
                                                            .p_4()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .justify_center()
                                                            .cursor_pointer()
                                                            .text_center()
                                                            .hover(|s| {
                                                                s
                                                                    .bg(gpui::rgb(15987958u32))
                                                                    .border_color(gpui::rgb(8490232u32))
                                                            })
                                                            .on_click(move |
                                                                _event: &gpui::ClickEvent,
                                                                _window: &mut gpui::Window,
                                                                cx: &mut gpui::App|
                                                            {
                                                                if let Some(path) = gpux::system::open_file() {
                                                                    {
                                                                        ::std::io::_print(
                                                                            format_args!("Selected file: {0:?}\n", path),
                                                                        );
                                                                    };
                                                                }
                                                            })
                                                            .on_drop(move |
                                                                event: &gpui::ExternalPaths,
                                                                _window: &mut gpui::Window,
                                                                cx: &mut gpui::App|
                                                            {
                                                                {
                                                                    ::std::io::_print(
                                                                        format_args!("Dropped files: {0:?}\n", event.paths()),
                                                                    );
                                                                };
                                                            })
                                                            .child(
                                                                gpui::div()
                                                                    .w_6()
                                                                    .h_6()
                                                                    .text_color(gpui::rgb(8490232u32))
                                                                    .mb_1()
                                                                    .child({ icon(icondata::LuFolderOpen) }),
                                                            )
                                                            .child(
                                                                gpui::div()
                                                                    .text_size(gpui::px(12f32))
                                                                    .text_color(gpui::rgb(7041664u32))
                                                                    .child(
                                                                        gpui::Component::new(
                                                                            gpux::components::SelectableText::new(
                                                                                "Click to Open File or Drop Files Here",
                                                                            ),
                                                                        ),
                                                                    ),
                                                            ),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .bg(gpui::rgb(16777215u32))
                                    .p_6()
                                    .rounded_2xl()
                                    .shadow_sm()
                                    .border_1()
                                    .border_color(gpui::rgb(15067115u32))
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_size(gpui::px(18f32))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(gpui::rgb(2042167u32))
                                            .child(
                                                gpui::div()
                                                    .w_6()
                                                    .h_6()
                                                    .text_color(gpui::rgb(11032055u32))
                                                    .child({ icon(icondata::LuImage) }),
                                            )
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new("Optimized Media"),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_color(gpui::rgb(7041664u32))
                                            .text_size(gpui::px(14f32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Next.js style image components and rich SVG icon sets.",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_4()
                                            .mt_2()
                                            .child(
                                                gpui::div()
                                                    .id("auto-state-id")
                                                    .w(gpui::px(100f32))
                                                    .h(gpui::px(70f32))
                                                    .rounded_lg()
                                                    .shadow_sm()
                                                    .overflow_hidden()
                                                    .border_1()
                                                    .border_color(gpui::rgb(15987958u32))
                                                    .child({ image(test_img_path.clone()) }),
                                            )
                                            .child(
                                                gpui::div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap_3()
                                                    .child(
                                                        gpui::div()
                                                            .w_10()
                                                            .h_10()
                                                            .text_color(gpui::rgb(2042167u32))
                                                            .p_2()
                                                            .bg(gpui::rgb(15987958u32))
                                                            .rounded_full()
                                                            .child({ icon(icondata::SiGithub) }),
                                                    )
                                                    .child(
                                                        gpui::div()
                                                            .w_10()
                                                            .h_10()
                                                            .text_color(gpui::rgb(3900150u32))
                                                            .p_2()
                                                            .bg(gpui::rgb(15726335u32))
                                                            .rounded_full()
                                                            .child({ icon(icondata::SiReact) }),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .bg(gpui::rgb(16777215u32))
                                    .p_6()
                                    .rounded_2xl()
                                    .shadow_sm()
                                    .border_1()
                                    .border_color(gpui::rgb(15067115u32))
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_size(gpui::px(18f32))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(gpui::rgb(2042167u32))
                                            .child(
                                                gpui::div()
                                                    .w_6()
                                                    .h_6()
                                                    .text_color(gpui::rgb(16347926u32))
                                                    .child({ icon(icondata::LuActivity) }),
                                            )
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new("Async State"),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_color(gpui::rgb(7041664u32))
                                            .text_size(gpui::px(14f32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Seamless future resolution with use_future hook.",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .mt_2()
                                            .p_4()
                                            .bg(gpui::rgb(16382715u32))
                                            .rounded_xl()
                                            .border_1()
                                            .border_color(gpui::rgb(15987958u32))
                                            .text_size(gpui::px(14f32))
                                            .text_color(gpui::rgb(3621201u32))
                                            .child({
                                                if let Some(data) = async_data {
                                                    gpui::div().child({ text(data) }).into_any()
                                                } else {
                                                    gpui::div()
                                                        .text_color(gpui::rgb(10265519u32))
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            gpui::AnimationExt::with_animation(
                                                                gpui::div()
                                                                    .id("auto-state-id")
                                                                    .w_4()
                                                                    .h_4()
                                                                    .child({ icon(icondata::LuLoader) }),
                                                                "animate-spin",
                                                                gpui::Animation::new(std::time::Duration::from_millis(1000))
                                                                    .repeat()
                                                                    .with_easing(gpui::linear),
                                                                |this, _delta| { this },
                                                            ),
                                                        )
                                                        .child(
                                                            gpui::Component::new(
                                                                gpux::components::SelectableText::new("Loading state..."),
                                                            ),
                                                        )
                                                        .into_any()
                                                }
                                            }),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .bg(gpui::rgb(16777215u32))
                                    .p_6()
                                    .rounded_2xl()
                                    .shadow_sm()
                                    .border_1()
                                    .border_color(gpui::rgb(15067115u32))
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_size(gpui::px(18f32))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(gpui::rgb(2042167u32))
                                            .child(
                                                gpui::div()
                                                    .w_6()
                                                    .h_6()
                                                    .text_color(gpui::rgb(15485081u32))
                                                    .child({ icon(icondata::LuSparkles) }),
                                            )
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Portals & Animations",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_color(gpui::rgb(7041664u32))
                                            .text_size(gpui::px(14f32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "Render components anywhere in the tree and apply smooth CSS-like animations.",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .flex_row()
                                            .gap_4()
                                            .mt_2()
                                            .items_center()
                                            .child(
                                                gpui::AnimationExt::with_animation(
                                                    gpui::div()
                                                        .id("auto-state-id")
                                                        .text_color(gpui::rgb(15485081u32))
                                                        .font_weight(gpui::FontWeight::BOLD)
                                                        .child(
                                                            gpui::Component::new(
                                                                gpux::components::SelectableText::new("Bouncing!"),
                                                            ),
                                                        ),
                                                    "animate-bounce",
                                                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                                                        .repeat()
                                                        .with_easing(gpui::bounce(gpui::linear)),
                                                    |this, delta| this.top(gpui::px(-10.0 * delta)),
                                                ),
                                            )
                                            .child(
                                                gpui::div()
                                                    .id("open-portal-btn")
                                                    .bg(gpui::rgb(2042167u32))
                                                    .text_color(gpui::rgb(16777215u32))
                                                    .px_4()
                                                    .py_2()
                                                    .rounded_lg()
                                                    .text_size(gpui::px(14f32))
                                                    .cursor_pointer()
                                                    .ml_auto()
                                                    .hover(|s| s.bg(gpui::rgb(3621201u32)))
                                                    .on_click({
                                                        let set_show_modal = set_show_modal.clone();
                                                        move |_, _, cx| set_show_modal(true, cx)
                                                    })
                                                    .child(
                                                        gpui::Component::new(
                                                            gpux::components::SelectableText::new("Open Portal Modal"),
                                                        ),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        gpui::div()
                            .w_full()
                            .max_w(gpui::px(896.0))
                            .bg(gpui::rgb(16777215u32))
                            .p_6()
                            .rounded_2xl()
                            .shadow_sm()
                            .border_1()
                            .border_color(gpui::rgb(15067115u32))
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                gpui::div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_size(gpui::px(18f32))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(2042167u32))
                                    .child(
                                        gpui::div()
                                            .w_6()
                                            .h_6()
                                            .text_color(gpui::rgb(1357990u32))
                                            .child({ icon(icondata::LuMousePointerClick) }),
                                    )
                                    .child(
                                        gpui::Component::new(
                                            gpux::components::SelectableText::new(
                                                "Selection & Scrolling",
                                            ),
                                        ),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .p_4()
                                    .bg(gpui::rgba(4277987456u32))
                                    .rounded_xl()
                                    .border_1()
                                    .border_color(gpui::rgb(16710083u32))
                                    .text_color(gpui::rgb(3621201u32))
                                    .text_size(gpui::px(14f32))
                                    .child({
                                        gpui::Component::new(
                                            SelectableText::new(
                                                "This text is natively selectable across the GPUI view tree. Try selecting and copying it!",
                                            ),
                                        )
                                    }),
                            )
                            .child(
                                gpui::div()
                                    .id("my-scroll-area")
                                    .w_full()
                                    .h(gpui::px(120f32))
                                    .bg(gpui::rgb(16382715u32))
                                    .border_1()
                                    .border_color(gpui::rgb(15067115u32))
                                    .rounded_xl()
                                    .overflow_y_scroll()
                                    .p_2()
                                    .mt_2()
                                    .children({
                                        (1..=20)
                                            .map(|i| {
                                                gpui::div()
                                                    .px_4()
                                                    .py_2()
                                                    .border_b_1()
                                                    .border_color(gpui::rgb(15987958u32))
                                                    .border_0()
                                                    .text_color(gpui::rgb(4937059u32))
                                                    .hover(|s| s.bg(gpui::rgb(16777215u32)))
                                                    .child({
                                                        text(
                                                            ::alloc::__export::must_use({
                                                                ::alloc::fmt::format(
                                                                    format_args!("Virtual Scrollable Item #{0}", i),
                                                                )
                                                            }),
                                                        )
                                                    })
                                                    .into_any()
                                            })
                                    }),
                            ),
                    ),
            )
    }
}
#[allow(non_snake_case)]
fn SettingsPage(app_cx: &mut App) -> gpui::Entity<SettingsPageComponent> {
    app_cx
        .new(|_cx| SettingsPageComponent {
            hooks: gpux::hooks::Hooks::new(),
        })
}
struct SettingsPageComponent {
    hooks: gpux::hooks::Hooks,
}
impl gpui::Render for SettingsPageComponent {
    fn render(
        &mut self,
        #[allow(unused_variables)]
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.hooks.reset();
        let __entity = cx.entity().clone();
        gpui::div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .bg(gpui::rgb(16382715u32))
            .gap_6()
            .p_8()
            .child(
                gpui::AnimationExt::with_animation(
                    gpui::div()
                        .id("auto-state-id")
                        .w_full()
                        .max_w(gpui::px(512.0))
                        .bg(gpui::rgb(16777215u32))
                        .p_10()
                        .rounded_2xl()
                        .shadow_sm()
                        .border_1()
                        .border_color(gpui::rgb(15067115u32))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_6()
                        .child(
                            gpui::div()
                                .w_16()
                                .h_16()
                                .text_color(gpui::rgb(6514417u32))
                                .bg(gpui::rgb(15659775u32))
                                .p_4()
                                .rounded_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child({ icon(icondata::LuSettings) }),
                        )
                        .child(
                            gpui::div()
                                .text_size(gpui::px(32f32))
                                .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                .text_color(gpui::rgb(1120295u32))
                                .child(
                                    gpui::Component::new(
                                        gpux::components::SelectableText::new("Settings Page"),
                                    ),
                                ),
                        )
                        .child(
                            gpui::div()
                                .text_size(gpui::px(16f32))
                                .text_color(gpui::rgb(7041664u32))
                                .text_center()
                                .child(
                                    gpui::Component::new(
                                        gpux::components::SelectableText::new(
                                            "This is a separate screen demonstrating the built-in React-style router. Notice how seamlessly the views transition.",
                                        ),
                                    ),
                                ),
                        )
                        .child(
                            gpui::div()
                                .id("settings-back-btn")
                                .bg(gpui::rgb(1120295u32))
                                .text_color(gpui::rgb(16777215u32))
                                .px_8()
                                .py_3()
                                .rounded_xl()
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .cursor_pointer()
                                .mt_4()
                                .flex()
                                .items_center()
                                .gap_2()
                                .hover(|s| s.bg(gpui::rgb(2042167u32)))
                                .on_click(move |
                                    _event: &gpui::ClickEvent,
                                    _window: &mut gpui::Window,
                                    cx: &mut gpui::App|
                                {
                                    {
                                        ::std::io::_print(format_args!("Clicked Home!\n"));
                                    };
                                    gpux::router::Navigator::push("/", cx);
                                })
                                .child(
                                    gpui::div()
                                        .w_5()
                                        .h_5()
                                        .child({ icon(icondata::LuArrowLeft) }),
                                )
                                .child(
                                    gpui::Component::new(
                                        gpux::components::SelectableText::new("Go Back Home"),
                                    ),
                                ),
                        ),
                    "animate-fade-in",
                    gpui::Animation::new(std::time::Duration::from_millis(500))
                        .with_easing(gpui::linear),
                    |this, delta| this.opacity(delta),
                ),
            )
    }
}
#[allow(non_snake_case)]
fn AppRoot(app_cx: &mut App) -> gpui::Entity<AppRootComponent> {
    app_cx
        .new(|_cx| AppRootComponent {
            hooks: gpux::hooks::Hooks::new(),
        })
}
struct AppRootComponent {
    hooks: gpux::hooks::Hooks,
}
impl gpui::Render for AppRootComponent {
    fn render(
        &mut self,
        #[allow(unused_variables)]
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.hooks.reset();
        let __entity = cx.entity().clone();
        let (path, set_path) = {
            let (__val, __idx) = self.hooks.use_state("/".to_string());
            let __entity_clone = __entity.clone();
            let __setter = move |new_val, cx: &mut gpui::App| {
                __entity_clone
                    .update(
                        cx,
                        |this, cx| {
                            this.hooks.set_state(__idx, new_val);
                            cx.notify();
                        },
                    );
            };
            (__val, __setter)
        };
        let (show_modal, set_show_modal) = {
            let (__val, __idx) = self.hooks.use_state(false);
            let __entity_clone = __entity.clone();
            let __setter = move |new_val, cx: &mut gpui::App| {
                __entity_clone
                    .update(
                        cx,
                        |this, cx| {
                            this.hooks.set_state(__idx, new_val);
                            cx.notify();
                        },
                    );
            };
            (__val, __setter)
        };
        let set_show_modal_arc = std::sync::Arc::new(set_show_modal)
            as std::sync::Arc<dyn Fn(bool, &mut gpui::App)>;
        let counter_page = self
            .hooks
            .check_memo(
                Some(show_modal),
                || {
                    CounterPage(
                        cx,
                        100,
                        "My React-like Router!".to_string(),
                        set_show_modal_arc.clone(),
                    )
                },
            );
        let settings_page = self.hooks.check_memo(Some(()), || { SettingsPage(cx) });
        if self.hooks.check_effect(Some(())) {
            {
                let set_path = set_path.clone();
                gpux::router::Navigator::init(
                    "/".to_string(),
                    move |new_path, cx| {
                        set_path(new_path, cx);
                    },
                    cx,
                );
            }
        }
        gpui::div()
            .id("auto-state-id")
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .absolute()
            .top_0()
            .left_0()
            .overflow_hidden()
            .bg(gpui::rgb(16382715u32))
            .child(
                gpui::div()
                    .h(gpui::px(64f32))
                    .w_full()
                    .bg(gpui::rgb(16777215u32))
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_6()
                    .gap_6()
                    .shadow_sm()
                    .border_b_1()
                    .border_color(gpui::rgb(15067115u32))
                    .child(
                        gpui::div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mr_4()
                            .child(
                                gpui::div()
                                    .w_8()
                                    .h_8()
                                    .text_color(gpui::rgb(5195493u32))
                                    .child({ icon(icondata::LuLayers) }),
                            )
                            .child(
                                gpui::div()
                                    .text_color(gpui::rgb(1120295u32))
                                    .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                    .text_size(gpui::px(22f32))
                                    .child(
                                        gpui::Component::new(
                                            gpux::components::SelectableText::new("GPUX Demo"),
                                        ),
                                    ),
                            ),
                    )
                    .child(
                        gpui::div()
                            .id("nav-home-btn")
                            .text_color(gpui::rgb(4937059u32))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .cursor_pointer()
                            .hover(|s| s.text_color(gpui::rgb(5195493u32)))
                            .on_click(move |
                                _event: &gpui::ClickEvent,
                                _window: &mut gpui::Window,
                                cx: &mut gpui::App|
                            {
                                {
                                    ::std::io::_print(format_args!("Clicked Home!\n"));
                                };
                                gpux::router::Navigator::push("/", cx);
                            })
                            .child(
                                gpui::Component::new(
                                    gpux::components::SelectableText::new("Home"),
                                ),
                            ),
                    )
                    .child(
                        gpui::div()
                            .id("nav-settings-btn")
                            .text_color(gpui::rgb(4937059u32))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .cursor_pointer()
                            .hover(|s| s.text_color(gpui::rgb(5195493u32)))
                            .on_click(move |
                                _event: &gpui::ClickEvent,
                                _window: &mut gpui::Window,
                                cx: &mut gpui::App|
                            {
                                {
                                    ::std::io::_print(format_args!("Clicked Settings!\n"));
                                };
                                gpux::router::Navigator::push("/settings", cx);
                            })
                            .child(
                                gpui::Component::new(
                                    gpux::components::SelectableText::new("Settings"),
                                ),
                            ),
                    )
                    .child(gpui::div().flex_1())
                    .child(
                        gpui::div()
                            .text_color(gpui::rgb(10265519u32))
                            .cursor_pointer()
                            .w_6()
                            .h_6()
                            .hover(|s| s.text_color(gpui::rgb(2042167u32)))
                            .child({ icon(icondata::SiGithub) }),
                    ),
            )
            .child(
                gpui::div()
                    .id("auto-state-id")
                    .flex_1()
                    .w_full()
                    .relative()
                    .overflow_hidden()
                    .min_h_0()
                    .child({
                        if path == "/" {
                            counter_page.clone().into_any_element()
                        } else if path == "/settings" {
                            settings_page.clone().into_any_element()
                        } else {
                            gpui::div()
                                .p_8()
                                .child(
                                    gpui::Component::new(
                                        gpux::components::SelectableText::new("404 Not Found"),
                                    ),
                                )
                                .into_any_element()
                        }
                    }),
            )
            .child({
                if show_modal {
                    gpui::div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(gpui::rgba(153u32))
                        .child(
                            gpui::AnimationExt::with_animation(
                                gpui::div()
                                    .id("auto-state-id")
                                    .bg(gpui::rgb(16777215u32))
                                    .p_8()
                                    .rounded_2xl()
                                    .shadow_2xl()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_4()
                                    .w(gpui::px(400f32))
                                    .child(
                                        gpui::div()
                                            .w_12()
                                            .h_12()
                                            .text_color(gpui::rgb(6514417u32))
                                            .mb_2()
                                            .child({ icon(icondata::LuMonitor) }),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_size(gpui::px(24f32))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(gpui::rgb(1120295u32))
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new("Global Portal Modal"),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .text_color(gpui::rgb(7041664u32))
                                            .text_center()
                                            .text_size(gpui::px(15f32))
                                            .mb_4()
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new(
                                                        "This dialog is rendered at the root of the app, completely outside the scroll view, ensuring it perfectly covers the whole screen including the navigation bar!",
                                                    ),
                                                ),
                                            ),
                                    )
                                    .child(
                                        gpui::div()
                                            .id("close-portal-btn")
                                            .bg(gpui::rgb(5195493u32))
                                            .text_color(gpui::rgb(16777215u32))
                                            .px_6()
                                            .py_2()
                                            .rounded_xl()
                                            .cursor_pointer()
                                            .w_full()
                                            .text_center()
                                            .hover(|s| s.bg(gpui::rgb(4405450u32)))
                                            .on_click({
                                                let set_show_modal = set_show_modal_arc.clone();
                                                move |
                                                    _event: &gpui::ClickEvent,
                                                    _window: &mut gpui::Window,
                                                    cx: &mut gpui::App|
                                                set_show_modal(false, cx)
                                            })
                                            .child(
                                                gpui::Component::new(
                                                    gpux::components::SelectableText::new("Close Modal"),
                                                ),
                                            ),
                                    ),
                                "animate-slide-in",
                                gpui::Animation::new(std::time::Duration::from_millis(500))
                                    .with_easing(gpui::linear),
                                |this, delta| this.left(gpui::px(-50.0 * (1.0 - delta))),
                            ),
                        )
                        .into_any()
                } else {
                    gpui::div().into_any()
                }
            })
    }
}
struct MinimalApp {
    router_root: Entity<AppRootComponent>,
    focus_handle: FocusHandle,
}
impl MinimalApp {
    fn new(router_root: Entity<AppRootComponent>, cx: &mut Context<Self>) -> Self {
        Self {
            router_root,
            focus_handle: cx.focus_handle(),
        }
    }
}
impl Render for MinimalApp {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        selection_root(&self.focus_handle, self.router_root.clone())
    }
}
fn main() {
    env_logger::init();
    Application::new()
        .with_assets(gpux::with_assets(()))
        .run(|app_cx: &mut App| {
            app_cx
                .set_menus(
                    ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                Menu {
                                    name: "demo_app".into(),
                                    items: ::alloc::vec::Vec::new(),
                                },
                                Menu {
                                    name: "Edit".into(),
                                    items: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                        ::alloc::intrinsics::write_box_via_move(
                                            ::alloc::boxed::Box::new_uninit(),
                                            [
                                                MenuItem::action("Cut", Cut),
                                                MenuItem::action("Copy", Copy),
                                                MenuItem::action("Paste", Paste),
                                                MenuItem::action("Select All", GlobalSelectAll),
                                            ],
                                        ),
                                    ),
                                },
                            ],
                        ),
                    ),
                );
            app_cx
                .bind_keys([
                    gpui::KeyBinding::new("left", Left, None),
                    gpui::KeyBinding::new("right", Right, None),
                    gpui::KeyBinding::new("shift-left", SelectLeft, None),
                    gpui::KeyBinding::new("shift-right", SelectRight, None),
                    gpui::KeyBinding::new("backspace", Backspace, None),
                    gpui::KeyBinding::new("cmd-a", GlobalSelectAll, None),
                    gpui::KeyBinding::new("ctrl-a", GlobalSelectAll, None),
                    gpui::KeyBinding::new("cmd-c", Copy, None),
                    gpui::KeyBinding::new("ctrl-c", Copy, None),
                    gpui::KeyBinding::new("cmd-x", Cut, None),
                    gpui::KeyBinding::new("ctrl-x", Cut, None),
                    gpui::KeyBinding::new("cmd-v", Paste, None),
                    gpui::KeyBinding::new("ctrl-v", Paste, None),
                ]);
            let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), app_cx);
            app_cx
                .set_global(MyAppState {
                    theme: "Dark".to_string(),
                });
            app_cx.set_global(GlobalSelectionState::default());
            app_cx
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |_, window_cx| {
                        let router_root = AppRoot(window_cx);
                        window_cx.new(|cx| MinimalApp::new(router_root, cx))
                    },
                )
                .unwrap();
            app_cx.activate(true);
        });
}
