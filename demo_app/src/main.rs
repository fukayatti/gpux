use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, Menu, MenuItem, Window, WindowBounds,
    WindowOptions, prelude::*, px, size,
};
use gpux::{
    gpui_component, icon, image, text, use_animation, use_context, use_effect, use_future,
    use_memo, use_state, view,
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

#[gpui_component]
fn CounterPage(
    app_cx: &mut App,
    initial_value: i32,
    title: String,
    set_show_modal: std::sync::Arc<dyn Fn(bool, &mut gpui::App)>,
) -> impl IntoElement {
    let _ctx = use_context!(MyAppState);
    // println!("Theme is: {}", ctx.theme);

    // 状態を宣言（Svelte/Reactライク）
    let (count, set_count) = use_state!(initial_value);
    let (width, start_anim): (f32, _) = use_animation!(150.0f32);
    let (dropped_image, set_dropped_image) = use_state!(None::<String>);

    let text_input_view = use_memo!((), cx.new(|cx| TextInput::new(cx)));

    let bg_executor = cx.background_executor().clone();

    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let test_img_path = format!("{}/assets/test.png", manifest_dir);

    let async_data = use_future!(String, count, {
        bg_executor
            .timer(std::time::Duration::from_millis(1500))
            .await;
        format!("Fetched Data for Count {}", count)
    });

    use_effect!(count, {
        println!("Count changed: {}", count);
    });

    let count_str = use_memo!(count, format!("Count: {}", count));

    view! {
        <div id="main-scroll" class="w-full h-full overflow-y-scroll bg-gray-50">
          <div class="flex flex-col min-h-full items-center p-8 gap-8">
            // Hero Section
            <div class="w-full max-w-4xl bg-white rounded-2xl shadow-sm border border-gray-200 p-8 flex flex-col md:flex-row items-center gap-8">
                <div class="flex-1 flex flex-col gap-4">
                    <div class="text-[40px] font-extrabold text-indigo-600">
                        { text(title) }
                    </div>
                    <div class="text-[16px] text-gray-600 leading-relaxed">
                        "Experience the power of GPUI combined with React-like hooks, JSX-style macros, and automated image optimization. Building native desktop apps has never been this smooth."
                    </div>
                    <div class="flex flex-row gap-4 mt-2">
                        <div
                            id="increment-btn"
                            class="bg-indigo-600 text-white px-6 py-3 rounded-xl font-medium cursor-pointer hover:bg-indigo-700 active:bg-indigo-800 shadow-sm transition-all flex items-center gap-2"
                            on_click=move |_event: &gpui::ClickEvent, _window: &mut Window, cx: &mut App| set_count(count + 1, cx)
                        >
                            <div class="w-5 h-5">{ icon(icondata::LuPlus) }</div>
                            { text(format!("Count: {}", count)) }
                        </div>
                        <div
                            id="animated-box"
                            class="bg-emerald-500 text-white px-6 py-3 rounded-xl font-medium cursor-pointer hover:bg-emerald-600 shadow-sm transition-all flex items-center justify-center gap-2"
                            w=px(width)
                            on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| start_anim(300.0, 500, cx)
                        >
                            <div class="w-5 h-5">{ icon(icondata::LuPlay) }</div>
                            "Animate"
                        </div>
                    </div>
                </div>
                <div class="w-64 h-64 rounded-2xl overflow-hidden shadow-lg border-4 border-white shrink-0">
                    { NextImage(cx, "https://images.unsplash.com/photo-1550745165-9bc0b252726f?q=80&w=800".to_string(), 256.0, 256.0).into_any_element() }
                </div>
            </div>

            // Features Grid
            <div class="w-full max-w-4xl grid grid-cols-1 md:grid-cols-2 gap-6">

                // Advanced Text Input
                <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                    <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                        <div class="w-6 h-6 text-blue-500">{ icon(icondata::LuKeyboard) }</div>
                        "Rich Input & IME"
                    </div>
                    <div class="text-gray-500 text-[14px]">"Full support for advanced text input and native IME integration."</div>
                    <div class="w-full mt-2">
                        { text_input_view.clone() }
                    </div>
                </div>

                // Media Showcase
                <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                    <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                        <div class="w-6 h-6 text-purple-500">{ icon(icondata::LuImage) }</div>
                        "Optimized Media"
                    </div>
                    <div class="text-gray-500 text-[14px]">"Next.js style image components and rich SVG icon sets."</div>
                    <div class="flex flex-row items-center gap-4 mt-2">
                        <div class="w-[100px] h-[70px] rounded-lg shadow-sm overflow-hidden border border-gray-100">
                            { image(test_img_path.clone()) }
                        </div>
                        <div class="flex flex-row gap-3">
                            <div class="w-10 h-10 text-gray-800 p-2 bg-gray-100 rounded-full">{ icon(icondata::SiGithub) }</div>
                            <div class="w-10 h-10 text-blue-500 p-2 bg-blue-50 rounded-full">{ icon(icondata::SiReact) }</div>
                        </div>
                    </div>
                </div>

                // Asynchronous State
                <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                    <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                        <div class="w-6 h-6 text-orange-500">{ icon(icondata::LuActivity) }</div>
                        "Async State"
                    </div>
                    <div class="text-gray-500 text-[14px]">"Seamless future resolution with use_future hook."</div>
                    <div class="mt-2 p-4 bg-gray-50 rounded-xl border border-gray-100 text-[14px] text-gray-700">
                        {
                            if let Some(data) = async_data {
                                view! { <div>{ text(data) } </div> }.into_any()
                            } else {
                                view! { <div class="text-gray-400 flex items-center gap-2"><div class="w-4 h-4 animate-spin">{ icon(icondata::LuLoader) }</div>"Loading state..."</div> }.into_any()
                            }
                        }
                    </div>
                </div>

                // Animations & Portals
                <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                    <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                        <div class="w-6 h-6 text-pink-500">{ icon(icondata::LuSparkles) }</div>
                        "Portals & Animations"
                    </div>
                    <div class="text-gray-500 text-[14px]">"Render components anywhere in the tree and apply smooth CSS-like animations."</div>
                    <div class="flex flex-row gap-4 mt-2 items-center">
                        <div class="text-pink-500 font-bold animate-bounce">
                            "Bouncing!"
                        </div>
                        <div
                            id="open-portal-btn"
                            class="bg-gray-800 text-white px-4 py-2 rounded-lg text-[14px] cursor-pointer hover:bg-gray-700 ml-auto"
                            on_click={ let set_show_modal = set_show_modal.clone(); move |_, _, cx| set_show_modal(true, cx) }
                        >
                            "Open Portal Modal"
                        </div>
                    </div>
                </div>

                // Native Integrations & Drag and Drop
                <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                    <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                        <div class="w-6 h-6 text-indigo-500">{ icon(icondata::LuHardDrive) }</div>
                        "Native Integrations & Drag and Drop"
                    </div>
                    <div class="text-gray-500 text-[14px]">"Drag and drop an image file below, or click to open a native file dialog."</div>
                    <div
                        id="drop-zone"
                        class="w-full h-48 border-2 border-dashed border-gray-300 rounded-xl flex flex-col items-center justify-center cursor-pointer hover:bg-gray-50 transition-colors relative overflow-hidden"
                        on_click={
                            let set_dropped_image = set_dropped_image.clone();
                            move |_, _, cx| {
                                if let Some(path) = gpux::system::file_system::dialog::open_file() {
                                    set_dropped_image(Some(format!("file://{}", path.display())), cx);
                                }
                            }
                        }
                        on_drop={
                            let set_dropped_image = set_dropped_image.clone();
                            move |event: &gpui::ExternalPaths, _, cx| {
                                if let Some(path) = event.paths().first() {
                                    set_dropped_image(Some(format!("file://{}", path.display())), cx);
                                }
                            }
                        }
                    >
                        {
                            if let Some(url) = dropped_image.clone() {
                                NextImage(cx, url, 800.0, 400.0).into_any_element()
                            } else {
                                view! {
                                    <div class="flex flex-col items-center gap-2 text-gray-400">
                                        <div class="w-8 h-8">{ icon(icondata::FiUploadCloud) }</div>
                                        <div class="text-[14px] font-medium">"Drop an image here or click to browse"</div>
                                    </div>
                                }.into_any()
                            }
                        }
                    </div>
                </div>
            </div>

            // Selection and scrolling
            <div class="w-full max-w-4xl bg-white p-6 rounded-2xl shadow-sm border border-gray-200 flex flex-col gap-4">
                <div class="flex items-center gap-2 text-[18px] font-bold text-gray-800">
                    <div class="w-6 h-6 text-teal-500">{ icon(icondata::LuMousePointerClick) }</div>
                    "Selection & Scrolling"
                </div>
                <div class="p-4 bg-yellow-50/50 rounded-xl border border-yellow-100 text-gray-700 text-[14px]">
                    { gpui::Component::new(SelectableText::new("This text is natively selectable across the GPUI view tree. Try selecting and copying it!")) }
                </div>

                <div
                    id="my-scroll-area"
                    class="w-full h-[120px] bg-gray-50 border border-gray-200 rounded-xl overflow-y-scroll p-2 mt-2"
                    children={
                        (1..=20).map(|i| {
                            view! {
                                <div class="px-4 py-2 border-b border-gray-100 last:border-0 text-gray-600 hover:bg-white transition-colors">
                                    { text(format!("Virtual Scrollable Item #{}", i)) }
                                </div>
                            }.into_any()
                        })
                    }
                >
                </div>
            </div>

          </div>
        </div>
    }
}

#[gpui_component]
fn SettingsPage(app_cx: &mut App) -> impl IntoElement {
    view! {
        <div class="flex flex-col items-center justify-center w-full h-full bg-gray-50 gap-6 p-8">
            <div class="w-full max-w-lg bg-white p-10 rounded-2xl shadow-sm border border-gray-200 flex flex-col items-center gap-6 animate-fade-in">
                <div class="w-16 h-16 text-indigo-500 bg-indigo-50 p-4 rounded-full flex items-center justify-center">
                    { icon(icondata::LuSettings) }
                </div>
                <div class="text-[32px] font-extrabold text-gray-900">
                    "Settings Page"
                </div>
                <div class="text-[16px] text-gray-500 text-center leading-relaxed">
                    "This is a separate screen demonstrating the built-in React-style router. Notice how seamlessly the views transition."
                </div>
                <div
                    id="settings-back-btn"
                    class="bg-gray-900 text-white px-8 py-3 rounded-xl font-medium cursor-pointer hover:bg-gray-800 transition-colors mt-4 flex items-center gap-2"
                    on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Home!"); gpux::router::Navigator::push("/", cx); }
                >
                    <div class="w-5 h-5">{ icon(icondata::LuArrowLeft) }</div>
                    "Go Back Home"
                </div>
            </div>
        </div>
    }
}

#[gpui_component]
fn AppRoot(app_cx: &mut App) -> impl IntoElement {
    let (path, set_path) = use_state!("/".to_string());
    let (show_modal, set_show_modal) = use_state!(false);
    let set_show_modal_arc =
        std::sync::Arc::new(set_show_modal) as std::sync::Arc<dyn Fn(bool, &mut gpui::App)>;

    let counter_page = use_memo!(
        show_modal,
        CounterPage(
            cx,
            100,
            "My React-like Router!".to_string(),
            set_show_modal_arc.clone()
        )
    );
    let settings_page = use_memo!((), SettingsPage(cx));

    use_effect!((), {
        let set_path = set_path.clone();
        gpux::router::Navigator::init(
            "/".to_string(),
            move |new_path, cx| {
                set_path(new_path, cx);
            },
            cx,
        );
    });

    view! {
        <div class="w-full h-full flex flex-col absolute top-0 left-0 overflow-hidden bg-gray-50">
            // Navigation Bar
            <div class="h-[64px] w-full bg-white flex flex-row items-center px-6 gap-6 shadow-sm border-b border-gray-200 z-10">
                <div class="flex items-center gap-2 mr-4">
                    <div class="w-8 h-8 text-indigo-600">{ icon(icondata::LuLayers) }</div>
                    <div class="text-gray-900 font-extrabold text-[22px] tracking-tight">"GPUX Demo"</div>
                </div>
                <div
                    id="nav-home-btn"
                    class="text-gray-600 hover:text-indigo-600 font-medium cursor-pointer transition-colors"
                    on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Home!"); gpux::router::Navigator::push("/", cx); }
                >
                    "Home"
                </div>
                <div
                    id="nav-settings-btn"
                    class="text-gray-600 hover:text-indigo-600 font-medium cursor-pointer transition-colors"
                    on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Settings!"); gpux::router::Navigator::push("/settings", cx); }
                >
                    "Settings"
                </div>
                <div class="flex-1"></div>
                <div class="text-gray-400 hover:text-gray-800 cursor-pointer w-6 h-6 transition-colors">
                    { icon(icondata::SiGithub) }
                </div>
            </div>

            // Page Content
            <div class="flex-1 w-full relative overflow-hidden min-h-0">
                {
                    if path == "/" {
                        counter_page.clone().into_any_element()
                    } else if path == "/settings" {
                        settings_page.clone().into_any_element()
                    } else {
                        view! { <div class="p-8">"404 Not Found"</div> }.into_any_element()
                    }
                }
            </div>

            // App-level global overlay modal
            {
                if show_modal {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center bg-black/60 z-50">
                            <div class="bg-white p-8 rounded-2xl shadow-2xl flex flex-col items-center gap-4 w-[400px] animate-slide-in">
                                <div class="w-12 h-12 text-indigo-500 mb-2">{ icon(icondata::LuMonitor) }</div>
                                <div class="text-[24px] font-bold text-gray-900">"Global Portal Modal"</div>
                                <div class="text-gray-500 text-center text-[15px] mb-4">
                                    "This dialog is rendered at the root of the app, completely outside the scroll view, ensuring it perfectly covers the whole screen including the navigation bar!"
                                </div>
                                <div
                                    id="close-portal-btn"
                                    class="bg-indigo-600 text-white px-6 py-2 rounded-xl cursor-pointer hover:bg-indigo-700 w-full text-center transition-colors"
                                    on_click={ let set_show_modal = set_show_modal_arc.clone(); move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| set_show_modal(false, cx) }
                                >
                                    "Close Modal"
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    gpui::div().into_any()
                }
            }
        </div>
    }
}

// アプリのルート。Counterコンポーネントを保持する。
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Counterを画面に配置
        selection_root(&self.focus_handle, self.router_root.clone())
    }
}

fn main() {
    env_logger::init();

    Application::new()
        .with_assets(gpux::with_assets(()))
        .run(|app_cx: &mut App| {
            app_cx.set_menus(vec![
                Menu {
                    name: "demo_app".into(),
                    items: vec![],
                },
                Menu {
                    name: "Edit".into(),
                    items: vec![
                        MenuItem::action("Cut", Cut),
                        MenuItem::action("Copy", Copy),
                        MenuItem::action("Paste", Paste),
                        MenuItem::action("Select All", GlobalSelectAll),
                    ],
                },
            ]);

            app_cx.bind_keys([
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
            app_cx.set_global(MyAppState {
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
                        // Counterコンポーネントを初期化（プロパティを渡す）
                        let router_root = AppRoot(window_cx);
                        window_cx.new(|cx| MinimalApp::new(router_root, cx))
                    },
                )
                .unwrap();

            app_cx.activate(true);
        });
}
