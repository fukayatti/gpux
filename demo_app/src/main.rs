use gpui::{
    App, Application, Bounds, Context, Entity, Menu, MenuItem, OsAction, Window, WindowBounds,
    WindowOptions, prelude::*, px, size, FocusHandle,
};
use gpux::{
    gpui_component, text, use_animation, use_context, use_effect, use_future, use_memo, use_state,
    view,
};

mod input;
use input::TextInput;

mod advanced_input;
use advanced_input::{AdvancedTextInput, Backspace, Left, Right, SelectLeft, SelectRight};

use gpux::system::{
    Copy, Cut, GlobalSelectionState, Paste, SelectAll as GlobalSelectAll, selection_root,
};
use gpux::components::SelectableText;

struct MyAppState {
    theme: String,
}

impl gpui::Global for MyAppState {}

#[gpui_component]
fn CounterPage(app_cx: &mut App, initial_value: i32, title: String) -> impl IntoElement {
    let ctx = use_context!(MyAppState);
    // println!("Theme is: {}", ctx.theme);

    // 状態を宣言（Svelte/Reactライク）
    let (count, set_count) = use_state!(initial_value);
    let (width, start_anim): (f32, _) = use_animation!(150.0f32);
    let (show_modal, set_show_modal) = use_state!(false);

    let advanced_input_view = use_memo!((), cx.new(|cx| AdvancedTextInput::new(cx)));

    let bg_executor = cx.background_executor().clone();

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
        <div id="main-scroll" class="w-full h-full overflow-y-scroll bg-slate-200 sm:bg-slate-300 md:bg-slate-400 lg:bg-slate-500 xl:bg-slate-600">
          <div class="flex flex-col min-h-full justify-center items-center gap-4 py-8">
            <div class="text-[16px] text-gray-800 font-bold mb-4">
                "Resize the window to see background color change!"
            </div>
            <div
                id="animated-box"
                class="bg-green-500 h-[50px] rounded-lg flex items-center justify-center text-white cursor-pointer"
                w=px(width)
                on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| start_anim(300.0, 500, cx)
            >
                "Click to Animate"
            </div>
            <div class="text-blue-700 text-[40px] font-bold animate-pulse">
                { text(title) }
            </div>
            <div class="text-black text-[32px] font-bold animate-bounce">
                { text(count_str) }
            </div>
            <div class="text-red-500 text-[24px] font-bold animate-ping">
                "Ping Animation"
            </div>
            <div class="text-purple-600 text-[24px] font-bold animate-spin">
                "Spin Animation"
            </div>
            <div class="text-green-600 text-[24px] font-bold animate-wiggle">
                "Wiggle Animation"
            </div>
            <div class="text-orange-500 text-[24px] font-bold animate-slide-in">
                "Slide In Animation"
            </div>
            <div class="text-teal-600 text-[24px] font-bold animate-fade-in">
                "Fade In Animation"
            </div>

            <div class="my-4 flex flex-col gap-2">
                <div>"Basic Input:"</div>
                { TextInput(cx) }
                <div>"Advanced IME Input:"</div>
                <div class="w-[300px]">
                    { advanced_input_view.clone() }
                </div>
                <div class="mt-4">"Selectable Text (Outside Input):"</div>
                <div class="w-[500px] p-2 bg-gray-100 rounded-md border border-gray-300">
                    { gpui::Component::new(SelectableText::new("This is a normal text that is now selectable! You can drag to select and use Cmd+C to copy.")) }
                </div>
            </div>

            <div class="text-gray-800 text-[20px]">
                {
                    if let Some(data) = async_data {
                        view! { <div>{ text(data) }</div> }.into_any()
                    } else {
                        view! { <div class="text-gray-400">"Loading..."</div> }.into_any()
                    }
                }
            </div>

            <div
                id="increment-btn"
                class="bg-blue-500 text-white p-[16px] rounded-md cursor-pointer hover:bg-blue-600 active:bg-blue-700"
                on_click={move |_event: &gpui::ClickEvent, _window: &mut Window, cx: &mut App| {
                    println!("Clicked! current count: {}", count);
                    set_count(count + 1, cx);
                }}
            >
                "Click me to increment!"
            </div>

            <div class="mt-4 text-slate-500 text-[16px]">
                "Built with gpux & Svelte/React Hooks DX"
            </div>

            <div
                id="my-scroll-area"
                class="w-[300px] h-[100px] bg-white border border-gray-300 rounded-md overflow-y-scroll p-2"
                children={
                    (0..20).map(|i| {
                        view! {
                            <div class="p-1 border-b border-gray-100 text-gray-700">
                                { text(format!("Scrollable Item #{}", i)) }
                            </div>
                        }.into_any()
                    })
                }
            >
            </div>

            <div
                id="open-portal-btn"
                class="bg-indigo-500 text-white p-[8px] rounded-md cursor-pointer mt-2"
                on_click={
                    let set_show_modal = set_show_modal.clone();
                    move |_, _, cx| set_show_modal(true, cx)
                }
            >
                "Open Portal"
            </div>

            {
                if show_modal {
                    view! {
                        <Portal>
                            <div class="flex items-center justify-center bg-black/50 absolute top-0 left-0 w-full h-full">
                                <div class="bg-white p-8 rounded-lg shadow-lg flex flex-col items-center gap-4">
                                    <div class="text-[24px] font-bold">"Portal Modal"</div>
                                    <div class="text-gray-700">"This is rendered outside the normal layout tree!"</div>
                                    <div
                                        id="close-portal-btn"
                                        class="bg-red-500 text-white p-[8px] rounded-md cursor-pointer hover:bg-red-600"
                                        on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| set_show_modal(false, cx)
                                    >
                                        "Close Modal"
                                    </div>
                                </div>
                            </div>
                        </Portal>
                    }.into_any()
                } else {
                    gpui::div().into_any()
                }
            }
          </div>
        </div>
    }
}


#[gpui_component]
fn SettingsPage(app_cx: &mut App) -> impl IntoElement {
    view! {
        <div class="flex flex-col items-center justify-center w-full h-full bg-slate-100 gap-4">
            <div class="text-[32px] font-bold text-gray-800">
                "Settings Page"
            </div>
            <div class="text-[16px] text-gray-600">
                "This is another screen!"
            </div>
            <div
                id="settings-back-btn"
                class="bg-blue-500 text-white p-4 rounded-md cursor-pointer hover:bg-blue-600"
                on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Home!"); gpux::router::Navigator::push("/", cx); }
            >
                "Go Back Home"
            </div>
        </div>
    }
}

#[gpui_component]
fn AppRoot(app_cx: &mut App) -> impl IntoElement {
    let (path, set_path) = use_state!("/".to_string());

    let counter_page = use_memo!((), CounterPage(cx, 100, "My React-like Router!".to_string()));
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
        <div class="w-full h-full flex flex-col relative overflow-hidden">
            // Navigation Bar
            <div class="h-[50px] w-full bg-gray-800 flex flex-row items-center px-4 gap-4 shadow-md ">
                <div class="text-white font-bold text-[20px] mr-4">"MyApp"</div>
                <div 
                    id="nav-home-btn"
                    class="text-gray-300 hover:text-white cursor-pointer"
                    on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Home!"); gpux::router::Navigator::push("/", cx); }
                >
                    "Home"
                </div>
                <div 
                    id="nav-settings-btn"
                    class="text-gray-300 hover:text-white cursor-pointer"
                    on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| { println!("Clicked Settings!"); gpux::router::Navigator::push("/settings", cx); }
                >
                    "Settings"
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Counterを画面に配置
        selection_root(&self.focus_handle, self.router_root.clone())
    }
}

fn main() {
    env_logger::init();

    Application::new().run(|app_cx: &mut App| {
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
