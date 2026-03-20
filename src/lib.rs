extern crate self as gpux;

pub use gpux_macros::*;

pub mod components;
pub mod hooks;
pub mod router;
pub mod system;

// Re-export commonly used items for convenience
pub use components::icon::{Icon, icon};
pub use components::image::{Image, image};

pub use components::{SelectableText, text};
pub use router::Navigator;
pub use system::{
    Copy, Cut, GlobalSelectionState, GpuxAssetSource, Paste, SelectAll, selection_root, with_assets,
};
