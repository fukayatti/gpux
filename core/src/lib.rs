mod colors;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use rstml::node::{Node, NodeAttribute, NodeElement};
use std::collections::HashMap;
use syn::{parse_macro_input, ItemFn};

#[derive(Hash, Eq, PartialEq, Debug)]
enum Modifier {
    Base,
    Hover,
    Focus,
    Active,
    GroupHover(String),
    GroupActive(String),
    Animate,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

#[proc_macro_attribute]
pub fn gpui_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;

    // sig.inputs contains the function arguments (like cx: &mut Window)
    let inputs = &sig.inputs;

    let mut props_fields = Vec::new();
    let mut props_init = Vec::new();
    let mut props_extract = Vec::new();

    let mut first_arg_ident = format_ident!("cx");

    for (i, arg) in inputs.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let ident = &pat_ident.ident;
                if i == 0 {
                    first_arg_ident = ident.clone();
                } else {
                    let ty = &pat_type.ty;
                    props_fields.push(quote! { #ident: #ty });
                    props_init.push(quote! { #ident });
                    props_extract.push(quote! { let #ident = self.#ident.clone(); });
                }
            }
        }
    }

    let struct_name = format_ident!("{}Component", fn_name);
    let stmts = &input_fn.block.stmts;

    // We inject a hidden Hooks module and struct to support the state arena
    let expanded = quote! {
        #[allow(non_snake_case)]
        #vis fn #fn_name(#inputs) -> gpui::Entity<#struct_name> {
            #first_arg_ident.new(|_cx| #struct_name {
                hooks: gpux::hooks::Hooks::new(),
                #(#props_init),*
            })
        }

        #vis struct #struct_name {
                hooks: gpux::hooks::Hooks,
                #(#props_fields),*
            }

            impl gpui::Render for #struct_name {
            fn render(&mut self, #[allow(unused_variables)] window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
                self.hooks.reset();
                let __entity = cx.entity().clone();
                #(#props_extract)*

                #(#stmts)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_context(input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as syn::Type);
    let expanded = quote! {
        cx.global::<#ty>()
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_effect(input: TokenStream) -> TokenStream {
    use syn::parse::{Parse, ParseStream};
    struct EffectInput {
        deps: syn::Expr,
        _comma: syn::Token![,],
        body: syn::Block,
    }
    impl Parse for EffectInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                deps: input.parse()?,
                _comma: input.parse()?,
                body: input.parse()?,
            })
        }
    }

    let EffectInput { deps, body, .. } = parse_macro_input!(input as EffectInput);

    let expanded = quote! {
        if self.hooks.check_effect(Some(#deps)) {
            #body
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_memo(input: TokenStream) -> TokenStream {
    use syn::parse::{Parse, ParseStream};
    struct MemoInput {
        deps: syn::Expr,
        _comma: syn::Token![,],
        calc: syn::Expr,
    }
    impl Parse for MemoInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                deps: input.parse()?,
                _comma: input.parse()?,
                calc: input.parse()?,
            })
        }
    }
    let MemoInput { deps, calc, .. } = parse_macro_input!(input as MemoInput);
    let expanded = quote! {
        self.hooks.check_memo(Some(#deps), || { #calc })
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_ref(input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as syn::Type);
    let expanded = quote! {
        {
            let (__val, __idx) = self.hooks.use_state(None::<#ty>);
            let __entity_clone = __entity.clone();
            let __setter = move |new_val, cx: &mut gpui::App| {
                __entity_clone.update(cx, |this, _cx| {
                    this.hooks.set_state(__idx, Some(new_val));
                });
            };
            (__val, __setter)
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_state(input: TokenStream) -> TokenStream {
    let init_expr = parse_macro_input!(input as syn::Expr);

    let expanded = quote! {
        {
            let (__val, __idx) = self.hooks.use_state(#init_expr);
            let __entity_clone = __entity.clone();

            let __setter = move |new_val, cx: &mut gpui::App| {
                __entity_clone.update(cx, |this, cx| {
                    this.hooks.set_state(__idx, new_val);
                    cx.notify();
                });
            };

            (__val, __setter)
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_animation(input: TokenStream) -> TokenStream {
    let init_expr = syn::parse_macro_input!(input as syn::Expr);

    let expanded = quote! {
        {
            let (__val, __idx) = self.hooks.use_state::<f32>(#init_expr as f32);
            let __entity_clone = __entity.clone();

            let __animator = move |target_val: f32, duration_ms: u64, cx: &mut gpui::App| {
                let start_val = __val;
                let steps = (duration_ms / 16).max(1);
                let step_val = (target_val - start_val) / (steps as f32);
                let __entity_clone_inner = __entity_clone.clone();

                let _task: gpui::Task<()> = cx.spawn(async move |mut cx| {
                    for i in 1..=steps {
                        cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
                        let next_val: f32 = if i == steps { target_val } else { start_val + step_val * (i as f32) };
                        let _ = cx.update(|cx: &mut gpui::App| {
                            __entity_clone_inner.update(cx, |this_ref: &mut Self, cx: &mut gpui::Context<Self>| {
                                this_ref.hooks.set_state::<f32>(__idx, next_val);
                                cx.notify();
                            })
                        });
                    }
                });
                _task.detach();
            };

            (__val, __animator)
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn use_future(input: TokenStream) -> TokenStream {
    use syn::parse::{Parse, ParseStream};
    struct FutureInput {
        ty: syn::Type,
        _comma1: syn::Token![,],
        deps: syn::Expr,
        _comma2: syn::Token![,],
        body: syn::Expr,
    }
    impl Parse for FutureInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ty: input.parse()?,
                _comma1: input.parse()?,
                deps: input.parse()?,
                _comma2: input.parse()?,
                body: input.parse()?,
            })
        }
    }
    let FutureInput { ty, deps, body, .. } = syn::parse_macro_input!(input as FutureInput);

    let expanded = quote! {
        {
            let (__val_opt, __idx) = self.hooks.use_state::<Option<#ty>>(None);
            if self.hooks.check_effect(Some(#deps)) {
                self.hooks.set_state::<Option<#ty>>(__idx, None);
                let __entity_clone = __entity.clone();
                let _task: gpui::Task<()> = cx.spawn(async move |_this, mut __cx| {
                    let __res = { #body };
                    let _ = __cx.update(|cx: &mut gpui::App| {
                        __entity_clone.update(cx, |this_ref: &mut Self, cx: &mut gpui::Context<Self>| {
                            this_ref.hooks.set_state::<Option<#ty>>(__idx, Some(__res));
                            cx.notify();
                        })
                    });
                });
                _task.detach();
            }
            __val_opt
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let nodes = match rstml::parse(input) {
        Ok(nodes) => nodes,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut output = proc_macro2::TokenStream::new();

    for node in nodes {
        let tokens = process_node(&node);
        output.extend(tokens);
    }

    let expanded = quote! {
        #output
    };

    TokenStream::from(expanded)
}

fn process_node(node: &Node) -> proc_macro2::TokenStream {
    match node {
        Node::Element(elem) => process_element(elem),
        Node::Text(text) => {
            // Remove the surrounding quotes from JSX text literals and extra whitespace
            let s = text.value_string();
            let trimmed = s.trim().trim_matches('"');

            // Ignore empty text nodes (e.g. formatting whitespace between tags)
            // as they will be turned into blank flex items and push real text off-screen.
            if trimmed.is_empty() {
                quote! {}
            } else {
                quote! { #trimmed }
            }
        }
        Node::Block(block) => {
            quote! { #block }
        }
        _ => quote! {},
    }
}

fn process_element(elem: &NodeElement) -> proc_macro2::TokenStream {
    let tag_name = elem.name().to_string();

    if tag_name == "Portal" {
        let mut child_tokens = proc_macro2::TokenStream::new();
        for child in &elem.children {
            child_tokens.extend(process_node(child));
        }
        return quote! {
            gpui::deferred(
                gpui::div().absolute().top(gpui::px(0.)).left(gpui::px(0.)).w_full().h_full().child(#child_tokens)
            ).priority(999)
        };
    }

    let tag_ident = format_ident!("{}", tag_name);

    let mut chain = quote! { gpui::#tag_ident() };

    let mut has_id = false;
    for attr in elem.attributes() {
        if let NodeAttribute::Attribute(a) = attr {
            if a.key.to_string() == "id" {
                has_id = true;
                if let Some(val) = a.value() {
                    chain.extend(quote! { .id(#val) });
                }
            }
        }
    }

    let mut animations = Vec::new();

    for attr in elem.attributes() {
        if let NodeAttribute::Attribute(a) = attr {
            let key = a.key.to_string();
            if key == "class" {
                if let Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) = a.value()
                {
                    let class_string = lit_str.value();

                    if (class_string.contains("active:") || class_string.contains("animate-"))
                        && !has_id
                    {
                        chain.extend(quote! { .id("auto-id") });
                        has_id = true;
                    }

                    let mut grouped_classes: HashMap<Modifier, Vec<proc_macro2::TokenStream>> =
                        HashMap::new();

                    for class in class_string.split_whitespace() {
                        let (modifier, tokens) = process_tailwind_class(class);
                        grouped_classes.entry(modifier).or_default().push(tokens);
                    }

                    if let Some(base_tokens) = grouped_classes.get(&Modifier::Base) {
                        for token in base_tokens {
                            chain.extend(token.clone());
                        }
                    }
                    if let Some(hover_tokens) = grouped_classes.get(&Modifier::Hover) {
                        chain.extend(quote! { .hover(|s| s #(#hover_tokens)* ) });
                    }
                    if let Some(focus_tokens) = grouped_classes.get(&Modifier::Focus) {
                        chain.extend(quote! { .focus(|s| s #(#focus_tokens)* ) });
                    }
                    if let Some(active_tokens) = grouped_classes.get(&Modifier::Active) {
                        chain.extend(quote! { .active(|s| s #(#active_tokens)* ) });
                    }

                    for (modifier, tokens) in grouped_classes.iter() {
                        match modifier {
                            Modifier::GroupHover(name) => {
                                chain.extend(quote! { .group_hover(#name, |s| s #(#tokens)* ) });
                            }
                            Modifier::GroupActive(name) => {
                                chain.extend(quote! { .group_active(#name, |s| s #(#tokens)* ) });
                            }
                            Modifier::Animate => {
                                for token in tokens {
                                    animations.push(token.clone());
                                }
                            }
                            Modifier::Sm => {
                                chain.extend(quote! { .when(window.viewport_size().width >= gpui::px(640.0), |s| s #(#tokens)* ) });
                            }
                            Modifier::Md => {
                                chain.extend(quote! { .when(window.viewport_size().width >= gpui::px(768.0), |s| s #(#tokens)* ) });
                            }
                            Modifier::Lg => {
                                chain.extend(quote! { .when(window.viewport_size().width >= gpui::px(1024.0), |s| s #(#tokens)* ) });
                            }
                            Modifier::Xl => {
                                chain.extend(quote! { .when(window.viewport_size().width >= gpui::px(1280.0), |s| s #(#tokens)* ) });
                            }
                            Modifier::Xxl => {
                                chain.extend(quote! { .when(window.viewport_size().width >= gpui::px(1536.0), |s| s #(#tokens)* ) });
                            }
                            _ => {}
                        }
                    }
                }
            } else if key != "id" {
                let attr_key = format_ident!("{}", key);
                if let Some(val) = a.value() {
                    chain.extend(quote! {
                        .#attr_key(#val)
                    });
                }
            }
        }
    }

    for child in &elem.children {
        let child_tokens = process_node(child);
        if !child_tokens.is_empty() {
            chain.extend(quote! {
                .child(#child_tokens)
            });
        }
    }

    for anim in animations {
        chain = quote! { gpui::AnimationExt::with_animation(#chain, #anim) };
    }

    chain
}

fn process_tailwind_class(class: &str) -> (Modifier, proc_macro2::TokenStream) {
    if let Some((modifier_str, rest)) = class.split_once(':') {
        let (_, inner) = process_tailwind_class(rest);

        if modifier_str.starts_with("group-hover") {
            let group_name = if let Some(slash_idx) = modifier_str.find('/') {
                modifier_str[slash_idx + 1..].to_string()
            } else {
                "group".to_string()
            };
            return (Modifier::GroupHover(group_name), inner);
        }

        if modifier_str.starts_with("group-active") {
            let group_name = if let Some(slash_idx) = modifier_str.find('/') {
                modifier_str[slash_idx + 1..].to_string()
            } else {
                "group".to_string()
            };
            return (Modifier::GroupActive(group_name), inner);
        }

        let modifier = match modifier_str {
            "hover" => Modifier::Hover,
            "focus" => Modifier::Focus,
            "active" => Modifier::Active,
            "sm" => Modifier::Sm,
            "md" => Modifier::Md,
            "lg" => Modifier::Lg,
            "xl" => Modifier::Xl,
            "2xl" => Modifier::Xxl,
            _ => Modifier::Base, // Fallback for unhandled modifiers
        };

        return (modifier, inner);
    }

    if class == "group" {
        return (Modifier::Base, quote! { .group("group") });
    } else if let Some(stripped) = class.strip_prefix("group/") {
        return (Modifier::Base, quote! { .group(#stripped) });
    }

    match class {
        "font-bold" => {
            return (
                Modifier::Base,
                quote! { .font_weight(gpui::FontWeight::BOLD) },
            );
        }
        "font-semibold" => {
            return (
                Modifier::Base,
                quote! { .font_weight(gpui::FontWeight::SEMIBOLD) },
            );
        }
        "font-normal" => {
            return (
                Modifier::Base,
                quote! { .font_weight(gpui::FontWeight::NORMAL) },
            );
        }
        "font-light" => {
            return (
                Modifier::Base,
                quote! { .font_weight(gpui::FontWeight::LIGHT) },
            );
        }
        "cursor-pointer" => return (Modifier::Base, quote! { .cursor_pointer() }),
        "animate-pulse" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-pulse",
                    gpui::Animation::new(std::time::Duration::from_millis(2000))
                        .repeat()
                        .with_easing(gpui::pulsating_between(0.5, 1.0)),
                    |this, delta| this.opacity(delta)
                },
            );
        }
        "animate-bounce" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-bounce",
                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                        .repeat()
                        .with_easing(gpui::bounce(gpui::linear)),
                    |this, delta| this.top(gpui::px(-10.0 * delta))
                },
            );
        }
        "animate-ping" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-ping",
                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                        .repeat()
                        .with_easing(gpui::linear),
                    |this, delta| {
                        // Note: actual scale/transform depends on GPUI capabilities
                        this.opacity(1.0 - delta)
                    }
                },
            );
        }
        "animate-spin" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-spin",
                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                        .repeat()
                        .with_easing(gpui::linear),
                    |this, _delta| {
                        // Rotation transform needs GPUI rotation API support
                        this
                    }
                },
            );
        }
        "animate-scale" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-scale",
                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                        .repeat()
                        .with_easing(gpui::linear),
                    |this, _delta| {
                        // Scale transform needs GPUI scale API support
                        this
                    }
                },
            );
        }
        "animate-fade-in" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-fade-in",
                    gpui::Animation::new(std::time::Duration::from_millis(500))
                        .with_easing(gpui::linear),
                    |this, delta| this.opacity(delta)
                },
            );
        }
        "animate-slide-in" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-slide-in",
                    gpui::Animation::new(std::time::Duration::from_millis(500))
                        .with_easing(gpui::linear),
                    |this, delta| this.left(gpui::px(-50.0 * (1.0 - delta)))
                },
            );
        }
        "animate-slide-up" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-slide-up",
                    gpui::Animation::new(std::time::Duration::from_millis(500))
                        .with_easing(gpui::linear),
                    |this, delta| this.top(gpui::px(50.0 * (1.0 - delta)))
                },
            );
        }
        "animate-wiggle" => {
            return (
                Modifier::Animate,
                quote! {
                    "animate-wiggle",
                    gpui::Animation::new(std::time::Duration::from_millis(1000))
                        .repeat()
                        .with_easing(gpui::linear),
                    |this, delta| this.left(gpui::px((delta * std::f32::consts::PI * 4.0).sin() * 10.0))
                },
            );
        }
        "transition" | "overflow-y-scroll" | "overflow-scroll" | "overflow-x-scroll" => {
            return (Modifier::Base, quote! {});
        }
        _ => {
            if class.starts_with("duration-")
                || class.starts_with("ease-")
                || class.starts_with("delay-")
                || class.starts_with("transition-")
                || class.starts_with("animate-")
            {
                return (Modifier::Base, quote! {});
            }
        }
    }

    if let Some(stripped) = class.strip_prefix("opacity-") {
        if let Ok(opacity_val) = stripped.parse::<f32>() {
            let opacity_float = opacity_val / 100.0;
            return (Modifier::Base, quote! { .opacity(#opacity_float) });
        } else if let Some(start) = class.find("-[") {
            if class.ends_with(']') {
                let value_str = &class[start + 2..class.len() - 1];
                if let Ok(opacity_val) = value_str.parse::<f32>() {
                    return (Modifier::Base, quote! { .opacity(#opacity_val) });
                }
            }
        }
    }

    if let Some(start) = class.find("-[") {
        if class.ends_with(']') {
            let mut prefix = &class[..start + 1];
            let value_str = &class[start + 2..class.len() - 1];
            let is_negative = class.starts_with('-');

            if is_negative {
                prefix = &class[1..start + 1];
            }

            if value_str.ends_with("px") {
                if let Ok(mut num) = value_str[..value_str.len() - 2].parse::<f32>() {
                    if is_negative {
                        num = -num;
                    }
                    match prefix {
                        "w-" => return (Modifier::Base, quote! { .w(gpui::px(#num)) }),
                        "h-" => return (Modifier::Base, quote! { .h(gpui::px(#num)) }),
                        "p-" => return (Modifier::Base, quote! { .p(gpui::px(#num)) }),
                        "px-" => return (Modifier::Base, quote! { .px(gpui::px(#num)) }),
                        "py-" => return (Modifier::Base, quote! { .py(gpui::px(#num)) }),
                        "m-" => return (Modifier::Base, quote! { .m(gpui::px(#num)) }),
                        "mt-" => return (Modifier::Base, quote! { .mt(gpui::px(#num)) }),
                        "mb-" => return (Modifier::Base, quote! { .mb(gpui::px(#num)) }),
                        "ml-" => return (Modifier::Base, quote! { .ml(gpui::px(#num)) }),
                        "mr-" => return (Modifier::Base, quote! { .mr(gpui::px(#num)) }),
                        "mx-" => return (Modifier::Base, quote! { .mx(gpui::px(#num)) }),
                        "my-" => return (Modifier::Base, quote! { .my(gpui::px(#num)) }),
                        "gap-" => return (Modifier::Base, quote! { .gap(gpui::px(#num)) }),
                        "text-" => return (Modifier::Base, quote! { .text_size(gpui::px(#num)) }),
                        "rounded-" => return (Modifier::Base, quote! { .rounded(gpui::px(#num)) }),
                        "max-w-" => return (Modifier::Base, quote! { .max_w(gpui::px(#num)) }),
                        "max-h-" => return (Modifier::Base, quote! { .max_h(gpui::px(#num)) }),
                        "min-w-" => return (Modifier::Base, quote! { .min_w(gpui::px(#num)) }),
                        "min-h-" => return (Modifier::Base, quote! { .min_h(gpui::px(#num)) }),
                        "top-" => return (Modifier::Base, quote! { .top(gpui::px(#num)) }),
                        "bottom-" => return (Modifier::Base, quote! { .bottom(gpui::px(#num)) }),
                        "left-" => return (Modifier::Base, quote! { .left(gpui::px(#num)) }),
                        "right-" => return (Modifier::Base, quote! { .right(gpui::px(#num)) }),
                        "inset-" => return (Modifier::Base, quote! { .inset(gpui::px(#num)) }),
                        _ => {}
                    }
                }
            } else if value_str.starts_with('#') {
                if let Ok(hex) = u32::from_str_radix(&value_str[1..], 16) {
                    match prefix {
                        "bg-" => return (Modifier::Base, quote! { .bg(gpui::rgb(#hex)) }),
                        "text-" => {
                            return (Modifier::Base, quote! { .text_color(gpui::rgb(#hex)) });
                        }
                        "border-" => {
                            return (Modifier::Base, quote! { .border_color(gpui::rgb(#hex)) });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let is_color_class =
        class.starts_with("bg-") || class.starts_with("text-") || class.starts_with("border-");

    let (base_class, opacity) = if is_color_class {
        if let Some((base, op)) = class.split_once('/') {
            let op_val = op.parse::<f32>().ok().map(|v| v / 100.0);
            (base, op_val)
        } else {
            (class, None)
        }
    } else {
        (class, None)
    };

    let check_color = |prefix: &str| -> Option<u32> {
        if base_class.starts_with(prefix) {
            colors::get_tailwind_color(&base_class[prefix.len()..])
        } else {
            None
        }
    };

    let mut color_hex = None;
    let mut color_type = "";

    if let Some(hex) = check_color("bg-") {
        color_hex = Some(hex);
        color_type = "bg";
    } else if let Some(hex) = check_color("text-") {
        color_hex = Some(hex);
        color_type = "text";
    } else if let Some(hex) = check_color("border-") {
        color_hex = Some(hex);
        color_type = "border";
    }

    if base_class.ends_with("transparent") {
        match color_type {
            "bg" => return (Modifier::Base, quote! { .bg(gpui::rgba(0x00000000)) }),
            "text" => {
                return (
                    Modifier::Base,
                    quote! { .text_color(gpui::rgba(0x00000000)) },
                );
            }
            "border" => {
                return (
                    Modifier::Base,
                    quote! { .border_color(gpui::rgba(0x00000000)) },
                );
            }
            _ => {}
        }
    } else if let Some(hex) = color_hex {
        if let Some(alpha) = opacity {
            let alpha_u8 = (alpha * 255.0).round() as u32;
            let rgba_val = (hex << 8) | alpha_u8;
            match color_type {
                "bg" => return (Modifier::Base, quote! { .bg(gpui::rgba(#rgba_val)) }),
                "text" => {
                    return (
                        Modifier::Base,
                        quote! { .text_color(gpui::rgba(#rgba_val)) },
                    );
                }
                "border" => {
                    return (
                        Modifier::Base,
                        quote! { .border_color(gpui::rgba(#rgba_val)) },
                    );
                }
                _ => {}
            }
        } else {
            match color_type {
                "bg" => return (Modifier::Base, quote! { .bg(gpui::rgb(#hex)) }),
                "text" => return (Modifier::Base, quote! { .text_color(gpui::rgb(#hex)) }),
                "border" => return (Modifier::Base, quote! { .border_color(gpui::rgb(#hex)) }),
                _ => {}
            }
        }
    }

    let method_name = class.replace('-', "_").replace('/', "_");

    let sanitized_name: String = method_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    if sanitized_name.is_empty() || sanitized_name.chars().next().unwrap().is_numeric() {
        return (Modifier::Base, quote! {});
    }

    if sanitized_name == "active" || sanitized_name == "hover" || sanitized_name == "focus" {
        return (Modifier::Base, quote! {});
    }

    let method_ident = format_ident!("{}", sanitized_name);

    (
        Modifier::Base,
        quote! {
            .#method_ident()
        },
    )
}
