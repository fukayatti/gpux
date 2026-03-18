use gpui::{
    App, Application, Bounds, Context, Entity, Window, WindowBounds, WindowOptions, prelude::*, px,
    size,
};
use gpux::{
    gpui_component, use_animation, use_context, use_effect, use_future, use_memo, use_state, view,
};

struct MyAppState {
    theme: String,
}

impl gpui::Global for MyAppState {}

#[gpui_component]
fn Counter(app_cx: &mut App, initial_value: i32, title: String) -> impl IntoElement {
    let ctx = use_context!(MyAppState);
    println!("Theme is: {}", ctx.theme);

    // 状態を宣言（Svelte/Reactライク）
    let (count, set_count) = use_state!(initial_value);
    let (width, start_anim): (f32, _) = use_animation!(150.0f32);
    let (show_modal, set_show_modal) = use_state!(false);

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
        <div class="flex flex-col w-full h-full bg-slate-200 sm:bg-slate-300 md:bg-slate-400 lg:bg-slate-500 xl:bg-slate-600 justify-center items-center gap-4">
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
                title
            </div>
            <div class="text-black text-[32px] font-bold animate-bounce">
                count_str
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

            <div class="text-gray-800 text-[20px]">
                {
                    if let Some(data) = async_data {
                        view! { <div>data</div> }.into_any()
                    } else {
                        view! { <div class="text-gray-400">"Loading..."</div> }.into_any()
                    }
                }
            </div>

            <div
                id="increment-btn"
                class="bg-blue-500 text-white p-[16px] rounded-md cursor-pointer hover:bg-blue-600 active:bg-blue-700"
                on_click={move |_event: &gpui::ClickEvent, _window: &mut Window, cx: &mut App| {
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
                                { format!("Scrollable Item #{}", i) }
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
                                        on_click=move |_, _, cx| set_show_modal(false, cx)
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
    }
}

// アプリのルート。Counterコンポーネントを保持する。
struct MinimalApp {
    counter: Entity<CounterComponent>,
}

impl Render for MinimalApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Counterを画面に配置
        self.counter.clone()
    }
}

fn main() {
    env_logger::init();

    Application::new().run(|app_cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), app_cx);
        app_cx.set_global(MyAppState {
            theme: "Dark".to_string(),
        });
        app_cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, window_cx| {
                    // Counterコンポーネントを初期化（プロパティを渡す）
                    let counter = Counter(window_cx, 100, "My React-like Counter!".to_string());
                    window_cx.new(|_| MinimalApp { counter })
                },
            )
            .unwrap();

        app_cx.activate(true);
    });
}
