use gpui::{App, Global};
use std::sync::Arc;

#[derive(Clone)]
pub struct Navigator {
    navigate_fn: Arc<dyn Fn(String, &mut App)>,
    pub current_path: String,
}

impl Global for Navigator {}

impl Navigator {
    pub fn init(path: String, navigate_fn: impl Fn(String, &mut App) + 'static, cx: &mut App) {
        println!("Navigator::init called with path: {}", path);
        cx.set_global(Self {
            navigate_fn: Arc::new(navigate_fn),
            current_path: path,
        });
    }

    pub fn push(path: impl Into<String>, cx: &mut App) {
        let path = path.into();
        println!("Navigator::push called with path: {}", path);
        if cx.has_global::<Navigator>() {
            let mut nav = cx.global::<Navigator>().clone();
            nav.current_path = path.clone();
            cx.set_global(nav.clone());
            (nav.navigate_fn)(path, cx);
        } else {
            println!("Navigator is not initialized. Cannot navigate to {}", path);
        }
    }
    
    pub fn current_path(cx: &mut App) -> Option<String> {
        if cx.has_global::<Navigator>() {
            Some(cx.global::<Navigator>().current_path.clone())
        } else {
            None
        }
    }
}
