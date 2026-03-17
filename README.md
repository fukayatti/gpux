# gpux (GPUI eXtended) - React-like DX for GPUI

A highly experimental, procedural macro-driven library that brings a **React-like developer experience (JSX + Hooks) and Tailwind CSS utility classes** to the [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) framework.

Write concise, declarative UI code using Tailwind classes inside an `rstml`-powered `view!` macro, with built-in React-like hooks (`use_state`, `use_effect`, `use_memo`, etc.) — all compiled directly into zero-overhead GPUI method chains.

---

## ⚠️ Disclaimer
**This is an experimental proof-of-concept / alpha software.**
It was built to explore the limits of GPUI and procedural macros.
Currently tested with `gpui = "0.2.2"`. Breaking changes in GPUI may require updates to this macro.

## Features

- **Tailwind CSS Parsing**: Converts Tailwind utility classes (`flex`, `w-full`, `bg-blue-500`t) directly into GPUI `.flex()`, `.w_full()`, etc. at compile time.
- **Arbitrary Values**: Supports Tailwind's bracket syntax like `w-[150px]`, `p-[16px]`, `bg-[#123456]`.
- **Modifiers & Pseudo-classes**: Full support for `hover:`, `active:`, `focus:`, and `group-hover:`.
- **Responsive Design**: Supports `sm:`, `md:`, `lg:`, `xl:`, and `2xl:` breakpoints mapped to GPUI's window viewport size.
- **Animations**: Recreates Tailwind animations like `animate-pulse`, `animate-bounce`, `animate-ping`, `animate-fade-in` via GPUI's animation engine.
- **React-like Hooks Engine**:
  - `use_state!`
  - `use_effect!`
  - `use_memo!`
  - `use_context!`
  - `use_future!` (Suspense-like async data fetching)
  - `use_animation!` (Manual custom animations)
- **Portals**: Use the `<Portal>` tag to render overlays, modals, and tooltips at the root level outside the standard DOM hierarchy.

## Project Structure

This workspace is split into three parts:
- `core/` (`gpux_macros`): The procedural macro crate that parses JSX (via `rstml`) and generates the GPUI Rust code.
- `src/` (`gpux`): The runtime wrapper crate that exports the macros and provides the `Hooks` arena struct used by the components.
- `demo_app/`: A sample GPUI application showcasing the capabilities of this library.

## Quick Start

### 1. Define a Component

Decorate a function with `#[gpui_component]` and use the `view!` macro.

```rust
use gpui::{App, IntoElement, px};
use gpux::{gpui_component, use_state, view};

#[gpui_component]
fn Counter(cx: &mut App, initial_value: i32) -> impl IntoElement {
    // React-like state!
    let (count, set_count) = use_state!(initial_value);

    view! {
        <div class="flex flex-col items-center justify-center w-full h-full bg-slate-100 gap-4">
            <div class="text-[32px] font-bold text-gray-800">
                { format!("Count: {}", count) }
            </div>
            <button
                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white px-[16px] py-[8px] rounded-md cursor-pointer"
                on_click={move |_event, _window, cx| set_count(count + 1, cx)}
            >
                "Increment"
            </button>
        </div>
    }
}
```

### 2. Animations & Responsive Modifiers

```rust
view! {
    // The background color will change depending on the window width
    // It will also continuously bounce!
    <div class="w-[200px] h-[200px] bg-red-500 sm:bg-green-500 md:bg-blue-500 animate-bounce rounded-full">
        "I am responsive and animated!"
    </div>
}
```

### 3. Modals and Portals

You can break out of the standard flow for modals using `<Portal>`.

```rust
view! {
    <Portal>
        <div class="absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black/50">
            <div class="bg-white p-[24px] rounded-lg shadow-lg">
                "I render on top of everything!"
            </div>
        </div>
    </Portal>
}
```

## Running the Demo App

The workspace includes a `demo_app` that demonstrates:
- A counter with Svelte/React-like state
- Tailwind color parsing and arbitrary spacing
- Suspending rendering for an async future
- Modals via Portals
- Real-time GPUI native animations (`animate-pulse`, `animate-bounce`, etc.)

```bash
cargo run -p demo_app
```

## What is Supported vs What is Missing

**Supported:**
Almost all layout (`flex`, `absolute`), flexbox/grid (`items-center`, `justify-between`), sizing (`w-full`, `h-full`), and spacing (`p-4`, `gap-2`) classes work out of the box because GPUI's style methods inherently mirror Tailwind's class names (converted to snake_case).

**Currently Missing / Limited by GPUI:**
- **`transition` & `duration`**: Unlike CSS, GPUI currently lacks a built-in "tweening" engine that automatically interpolates values between two states (e.g., smoothly fading a background color on hover). Styles apply instantly.
- **Complex Transforms**: Arbitrary CSS `transform` APIs (like exact degrees of `rotate` or scaling percentages) aren't natively supported in standard GPUI views without manual matrix manipulation.
- **Filters**: Advanced `backdrop-blur` or complex `box-shadow` configurations might not map perfectly.

## License
MIT