use gpui::{AssetSource, RenderImage, SharedString};
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Global registry for dynamic SVG icons generated from `icondata`.
/// We map a hash to the actual SVG string so `GpuxAssetSource` can load them.
pub static ICON_REGISTRY: Lazy<RwLock<HashMap<u64, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Global registry for dynamically optimized (downsampled/compressed) images.
/// We map a hash to the encoded image bytes so `GpuxAssetSource` can load them.
pub static OPTIMIZED_IMAGE_REGISTRY: Lazy<RwLock<HashMap<u64, Vec<u8>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Global registry for ultra-optimized (downsampled and decoded) raw GPUI RenderImages.
/// This bypasses standard AssetSource string resolution completely.
pub static OPTIMIZED_RENDER_REGISTRY: Lazy<RwLock<HashMap<u64, Arc<RenderImage>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// A custom `AssetSource` wrapper that enables GPUI to load dynamically
/// registered SVG strings (like `icondata` icons) via virtual paths.
///
/// It delegates to an `inner` AssetSource for all other paths (e.g., local files).
pub struct GpuxAssetSource<T: AssetSource> {
    pub inner: T,
}

impl<T: AssetSource> GpuxAssetSource<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: AssetSource> AssetSource for GpuxAssetSource<T> {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        println!("📝 [GpuxAssetSource] load() requested path: {}", path);
        // Intercept requests for dynamically registered gpux icons
        if path.starts_with("gpux-icon/") {
            println!("📝 [GpuxAssetSource] intercepted gpux-icon/ path");
            // Strip prefix and the .svg extension required by GPUI's image cache
            let hash_str = path
                .trim_start_matches("gpux-icon/")
                .trim_end_matches(".svg");
            if let Ok(hash) = hash_str.parse::<u64>() {
                if let Some(svg_str) = ICON_REGISTRY.read().unwrap().get(&hash) {
                    println!(
                        "✅ [GpuxAssetSource] found SVG in registry for hash: {}",
                        hash
                    );
                    return Ok(Some(Cow::Owned(svg_str.as_bytes().to_vec())));
                } else {
                    println!(
                        "❌ [GpuxAssetSource] missing SVG in registry for hash: {}",
                        hash
                    );
                }
            } else {
                println!(
                    "❌ [GpuxAssetSource] failed to parse hash from: {}",
                    hash_str
                );
            }
        }

        // Intercept requests for dynamically optimized images
        if path.starts_with("gpux-optimized/") {
            println!(
                "📝 [GpuxAssetSource] intercepted gpux-optimized/ path: {}",
                path
            );
            let hash_str = path
                .trim_start_matches("gpux-optimized/")
                .trim_end_matches(".webp")
                .trim_end_matches(".png");
            if let Ok(hash) = hash_str.parse::<u64>() {
                if let Some(image_bytes) = OPTIMIZED_IMAGE_REGISTRY.read().unwrap().get(&hash) {
                    println!(
                        "✅ [GpuxAssetSource] found optimized image in registry for hash: {}",
                        hash
                    );
                    return Ok(Some(Cow::Owned(image_bytes.clone())));
                } else {
                    println!(
                        "❌ [GpuxAssetSource] missing optimized image in registry for hash: {}",
                        hash
                    );
                }
            } else {
                println!(
                    "❌ [GpuxAssetSource] failed to parse optimized hash from: {}",
                    hash_str
                );
            }
        }

        // Delegate to the underlying AssetSource
        self.inner.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        self.inner.list(path)
    }
}

/// Helper function to wrap your base `AssetSource` with `GpuxAssetSource`.
/// Typically used in `App::new().with_assets(gpux::with_assets(()))`.
pub fn with_assets<T: AssetSource>(inner: T) -> GpuxAssetSource<T> {
    GpuxAssetSource::new(inner)
}
