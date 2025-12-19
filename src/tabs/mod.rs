mod content;
mod tab;
#[allow(clippy::module_inception)]
mod tabs;

pub use content::TabContent;
pub use tab::{Tab, TabComponent};
pub use tabs::Tabs;
