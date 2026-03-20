use gpui::prelude::FluentBuilder;
use gpui::*;
use std::hash::{Hash, Hasher};

/// Represents the source of an icon, either a custom file path or a bundled `icondata` SVG.
pub enum IconSource {
    Path(SharedString),
    Icondata(icondata::Icon),
}

impl From<&'static str> for IconSource {
    fn from(path: &'static str) -> Self {
        IconSource::Path(path.into())
    }
}

impl From<SharedString> for IconSource {
    fn from(path: SharedString) -> Self {
        IconSource::Path(path)
    }
}

impl From<String> for IconSource {
    fn from(path: String) -> Self {
        IconSource::Path(path.into())
    }
}

impl From<icondata::Icon> for IconSource {
    fn from(icon: icondata::Icon) -> Self {
        IconSource::Icondata(icon)
    }
}

/// A unified Icon component for GPUI.
/// It renders both custom local SVGs and `icondata` icons flawlessly.
#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    classes: SharedString,
}

impl Icon {
    /// Creates a new Icon from either a file path or an `icondata::Icon` variant.
    ///
    /// # Example
    /// ```rust
    /// // From icondata (Lucide, Heroicons, SimpleIcons, etc.)
    /// Icon::new(icondata::LuCheck)
    ///
    /// // From a custom local asset path
    /// Icon::new("assets/my-icon.svg")
    /// ```
    pub fn new(source: impl Into<IconSource>) -> Self {
        let path = match source.into() {
            IconSource::Path(p) => p,
            IconSource::Icondata(icon) => {
                // Generate a unique hash for the SVG data to act as the virtual path
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                icon.data.hash(&mut hasher);
                let hash = hasher.finish();
                // Important: The .svg extension is required for GPUI's image cache to recognize it as SVG
                // Using `/` instead of `://` prevents GPUI from assuming it's an external HTTP URL
                let virtual_path = format!("gpux-icon/{}.svg", hash);

                // Register the full constructed SVG string into the global registry
                // so that `GpuxAssetSource` can serve it to GPUI's rendering engine.
                let mut cache = crate::system::assets::ICON_REGISTRY.write().unwrap();
                if !cache.contains_key(&hash) {
                    // GPUI relies on xmlns to properly parse the document.
                    // We dynamically build attributes to correctly support both solid and line icons.
                    let width = icon.width.unwrap_or("24");
                    let height = icon.height.unwrap_or("24");
                    let view_box = icon.view_box.unwrap_or("0 0 24 24");

                    let mut attrs = vec![
                        format!(r#"xmlns="http://www.w3.org/2000/svg""#),
                        format!(r#"width="{}""#, width),
                        format!(r#"height="{}""#, height),
                        format!(r#"viewBox="{}""#, view_box),
                    ];

                    if let Some(fill) = icon.fill {
                        attrs.push(format!(r#"fill="{}""#, fill));
                    } else if icon.stroke.is_none() {
                        // If both fill and stroke are absent (like GitHub SimpleIcons),
                        // assume it is a solid icon colored by text color.
                        attrs.push(r#"fill="currentColor""#.to_string());
                    } else {
                        // Otherwise, it has stroke but no fill, so we clear the fill.
                        attrs.push(r#"fill="none""#.to_string());
                    }

                    if let Some(stroke) = icon.stroke {
                        attrs.push(format!(r#"stroke="{}""#, stroke));
                    }
                    if let Some(sw) = icon.stroke_width {
                        attrs.push(format!(r#"stroke-width="{}""#, sw));
                    }
                    if let Some(slc) = icon.stroke_linecap {
                        attrs.push(format!(r#"stroke-linecap="{}""#, slc));
                    }
                    if let Some(slj) = icon.stroke_linejoin {
                        attrs.push(format!(r#"stroke-linejoin="{}""#, slj));
                    }

                    let svg_str = format!("<svg {}>{}</svg>", attrs.join(" "), icon.data);
                    cache.insert(hash, svg_str);
                }

                virtual_path.into()
            }
        };

        Self {
            path,
            classes: "".into(),
        }
    }

    /// Appends Tailwind-like utility classes to the icon container.
    pub fn class(mut self, classes: impl Into<SharedString>) -> Self {
        self.classes = classes.into();
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _app: &mut App) -> impl IntoElement {
        println!("🎨 [Icon Component] render called for path: {}", self.path);
        // Create the base svg element using the resolved path.
        // If it's a gpux-icon/ path, GpuxAssetSource will intercept and serve it!

        // GPUI's `svg()` component silently ignores rendering if `style.text.color` is not set
        // on the SVG element itself. We must explicitly forward the parent's text color.
        let current_text_color = window.text_style().color;

        let svg_el = svg()
            .path(self.path.clone())
            .w_full()
            .h_full()
            .text_color(current_text_color);

        // Wrap in a div to apply classes, allowing sizing and styling
        div()
            .w_full()
            .h_full()
            .map(|div| {
                if !self.classes.is_empty() {
                    // Note: In a full integration, you'd parse and apply the tailwind classes here.
                    // For now we assume typical use cases rely on parent wrappers or inline styles for complex things,
                    // but the w-full h-full on the inner svg allows the outer div to constrain its size.
                    div
                } else {
                    div
                }
            })
            // GPUI sets the currentColor based on the text_color of the parent.
            // By default, the icon wrapper assumes it will inherit color from its parent tree.
            .child(svg_el)
    }
}

/// Helper function to create an Icon component easily in `view!` macros.
pub fn icon(source: impl Into<IconSource>) -> Icon {
    Icon::new(source)
}
