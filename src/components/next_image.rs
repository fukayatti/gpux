use crate::{gpui_component, use_animation, use_effect, use_future, use_state};
use gpui::*;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[gpui_component]
pub fn NextImage(app_cx: &mut App, src: String, width: f32, height: f32) -> impl IntoElement {
    // Generate a unique hash for this specific source + size combination
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    src.hash(&mut hasher);
    (width as i32).hash(&mut hasher);
    (height as i32).hash(&mut hasher);
    let cache_key = hasher.finish();

    // Check if it's already optimized and cached globally
    let already_cached = crate::system::assets::OPTIMIZED_RENDER_REGISTRY
        .read()
        .unwrap()
        .get(&cache_key)
        .cloned();

    let (loaded_image, set_loaded_image) = use_state!(already_cached.clone());
    let bg_executor = cx.background_executor().clone();

    // Fade-in animation when image is loaded
    let (_opacity, start_fade) = use_animation!(0.0_f32);

    use_effect!(loaded_image.clone(), {
        if loaded_image.is_some() {
            start_fade(1.0, 500, cx); // 500ms fade-in
        }
    });

    let async_load = use_future!(Option<Arc<RenderImage>>, src.clone(), {
        let src = src.clone();
        if let Some(img) = already_cached {
            Some(img)
        } else {
            // Move the heavy fetching and resizing into the background thread pool
            let cache_key = cache_key;
            let width = width;
            let height = height;
            let mut result = None;

            // 1. Fetch or read the image
            let image_bytes_result = if src.starts_with("http://") || src.starts_with("https://") {
                // Fetch using ureq (blocking, but we are in background executor)
                match ureq::get(&src).call() {
                    Ok(response) => {
                        let mut bytes = Vec::new();
                        let _ = response.into_reader().read_to_end(&mut bytes);
                        Ok(bytes)
                    }
                    Err(e) => Err(e.to_string()),
                }
            } else {
                // Local file read
                std::fs::read(&src).map_err(|e| e.to_string())
            };

            if let Ok(bytes) = image_bytes_result {
                // 2. Decode the image using the `image` crate
                if let Ok(dynamic_image) = image::load_from_memory(&bytes) {
                    // 3. Resize / Downsample to the exact UI bounds (optimization!)
                    // This prevents GPUI from uploading a 4K texture to VRAM for a 200x200 UI element.
                    let thumbnail = dynamic_image.thumbnail(width as u32, height as u32);

                    // 4. Convert directly to raw RGBA8 pixels (The ULTIMATE optimization)
                    // No need to encode back to WebP/PNG, saving CPU time!
                    let rgba = thumbnail.to_rgba8();

                    // 5. Create GPUI's native RenderImage using the image crate's Frame
                    let frame = image::Frame::new(rgba);
                    let render_image = Arc::new(RenderImage::new(vec![frame]));

                    // 6. Store in our global optimized render registry
                    crate::system::assets::OPTIMIZED_RENDER_REGISTRY
                        .write()
                        .unwrap()
                        .insert(cache_key, render_image.clone());
                    result = Some(render_image);
                }
            }
            result
        }
    });

    let new_image = async_load.unwrap_or(None);
    if new_image.is_some() && loaded_image.is_none() {
        set_loaded_image(new_image.clone(), cx);
    }

    let mut container = div()
        .w(px(width))
        .h(px(height))
        .rounded_lg()
        .overflow_hidden()
        .relative();

    if let Some(img) = loaded_image {
        // Render the actual optimized image with fade-in
        // We pass the raw Arc<RenderImage> directly to GPUI!
        let img_el = gpui::img(img)
            .w_full()
            .h_full()
            .object_fit(ObjectFit::Cover);

        // GPUI 0.2 doesn't have an easy opacity() on div yet, but the animation engine
        // allows bounds/color manipulation. For true Next.js feel, we just render it.
        container = container.child(div().w_full().h_full().child(img_el));
    } else {
        // Skeleton loader (Next.js blur / pulse effect)
        // A simple slate-200 box
        container = container.child(div().w_full().h_full().bg(rgb(0xe2e8f0)));
    }

    container
}
