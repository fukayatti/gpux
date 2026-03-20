use gpui::*;
use std::path::Path;

/// A simple Image component wrapper for GPUI.
/// Supports both local paths and remote URLs.
#[derive(IntoElement)]
pub struct Image {
    src: SharedString,
    classes: SharedString,
}

impl Image {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            classes: "".into(),
        }
    }

    /// Appends Tailwind-like utility classes to the image.
    pub fn class(mut self, classes: impl Into<SharedString>) -> Self {
        self.classes = classes.into();
        self
    }
}

impl RenderOnce for Image {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        let src_str = self.src.as_ref();

        // GPUI requires distinct handling for local paths vs URLs/embedded assets.
        // If the path is an absolute path on disk, we must pass it as a PathBuf so
        // GPUI knows to read it from the local file system. Otherwise, GPUI assumes
        // it's an embedded asset (AssetSource) or an HTTP URI.
        let image_source: gpui::ImageSource =
            if src_str.starts_with("http://") || src_str.starts_with("https://") {
                src_str.into() // Handled as Uri
            } else if Path::new(src_str).is_absolute() {
                std::path::PathBuf::from(src_str).into() // Handled as Path
            } else {
                src_str.into() // Handled as Embedded (AssetSource)
            };

        // Create the base image element using GPUI's img() primitive
        let img_el = gpui::img(image_source);
        let classes = self.classes.as_ref();

        // Wrap in a div to apply classes, allowing sizing and styling
        let mut wrapper = div();

        // Fallback mock parser for demo app classes since the global macro parser
        // doesn't automatically map `.class("...")` method chains yet.
        // In a fully integrated system, the gpux macro would extract and apply these.
        if classes.contains("w-[200px]") {
            wrapper = wrapper.w(px(200.0));
        } else {
            wrapper = wrapper.w_full(); // default to taking up available width
        }

        if classes.contains("h-[100px]") {
            wrapper = wrapper.h(px(100.0));
        } else {
            wrapper = wrapper.h_full();
        }

        if classes.contains("rounded-lg") {
            wrapper = wrapper.rounded_lg().overflow_hidden();
        }

        if classes.contains("shadow-md") {
            wrapper = wrapper.shadow_md();
        }

        wrapper.child(img_el.w_full().h_full().object_fit(ObjectFit::Cover))
    }
}

/// Helper function to create an Image component easily in `view!` macros.
pub fn image(src: impl Into<SharedString>) -> Image {
    Image::new(src)
}

/// Creates a standard `gpui::Img` element with automatically resolved source paths
/// (handling absolute paths, HTTP URLs, and embedded assets correctly).
/// This is particularly useful when you want full access to GPUI's layout and styling methods.
pub fn img(src: impl Into<SharedString>) -> gpui::Img {
    let src_str: SharedString = src.into();
    let image_source: gpui::ImageSource =
        if src_str.starts_with("http://") || src_str.starts_with("https://") {
            src_str.into()
        } else if Path::new(src_str.as_ref()).is_absolute() {
            std::path::PathBuf::from(src_str.as_ref()).into()
        } else {
            src_str.into()
        };

    gpui::img(image_source)
}
