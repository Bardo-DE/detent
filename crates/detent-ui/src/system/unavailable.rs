//! What a system page shows when there is nothing to show: its glyph, its name and one plain
//! sentence, and the action when one exists.

use dioxus::prelude::*;
use ds::components::overlays::empty_state::EmptyState;
use ds::prelude::*;

/// A page's place when it has nothing to show: `icon` over `title`, then `words`, then `action`
/// when the page offers one.
#[component]
pub(crate) fn Absent(
    icon: Icon,
    #[props(into)] title: String,
    #[props(into)] words: String,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        EmptyState { title, description: Some(TextLine::from(words)), icon: Some(icon), action }
    }
}

/// A system page whose service is not there: the page's glyph and name, and why.
#[component]
pub(crate) fn Unavailable(
    icon: Icon,
    #[props(into)] title: String,
    #[props(into)] words: String,
) -> Element {
    rsx! {
        div { class: "system-page", "data-system": "unavailable",
            Absent { icon, title, words }
        }
    }
}
