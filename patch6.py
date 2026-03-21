with open("core/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace('impl gpui::Render for #struct_name {\n            fn render(&mut self, #[allow(unused_variables)] window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {\n                use gpui::{InteractiveElement, ParentElement, Styled};', 'impl gpui::Render for #struct_name {\n            fn render(&mut self, #[allow(unused_variables)] window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {\n                use gpui::{InteractiveElement, ParentElement, Styled, Global};')

with open("core/src/lib.rs", "w") as f:
    f.write(content)
