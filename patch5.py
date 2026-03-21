with open("demo_app/src/main.rs", "r") as f:
    content = f.read()

patch = """
                    // Drag and Drop example
                    <div class="mt-4 pt-4 border-t border-gray-100">
                        <div class="text-[14px] font-bold text-gray-800 mb-2">"File System Integrations"</div>
                        <div class="flex flex-row gap-2">
                            <div
                                class="flex-1 bg-gray-50 border-2 border-dashed border-gray-300 rounded-xl p-4 flex flex-col items-center justify-center cursor-pointer hover:bg-gray-100 hover:border-indigo-400 transition-all text-center"
                                on_click=move |_event: &gpui::ClickEvent, _window: &mut gpui::Window, cx: &mut gpui::App| {
                                    if let Some(path) = gpux::system::dialog::open_file() {
                                        println!("Selected file: {:?}", path);
                                    }
                                }
                                on_drop=move |event: &gpui::ExternalPaths, _window: &mut gpui::Window, cx: &mut gpui::App| {
                                    println!("Dropped files: {:?}", event.paths());
                                }
                            >
                                <div class="w-6 h-6 text-indigo-400 mb-1">{ icon(icondata::LuFolderOpen) }</div>
                                <div class="text-[12px] text-gray-500">"Click to Open File or Drop Files Here"</div>
                            </div>
                        </div>
                    </div>
"""

content = content.replace('{ text_input_view.clone() }\n                    </div>', '{ text_input_view.clone() }\n                    </div>\n' + patch)

with open("demo_app/src/main.rs", "w") as f:
    f.write(content)
