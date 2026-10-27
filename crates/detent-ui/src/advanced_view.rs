//! A page's advanced keys: one drill-in row per section; a row pushes a page of that section's
//! rows over the page, with a back button, through quire's `PaneStack`.

use crate::files::{Edit, Files};
use crate::key_row::{HIT_CLASS, Match};
use crate::section_view::{SectionView, is_hit};
use detent_model::{PageModel, PageSlug, SectionModel, advanced_name};
use dioxus::prelude::*;
use ds::components::fields::field_row::FieldGroup;
use ds::components::lists::row::size::RowSize;
use ds::prelude::*;
use ds::root::common::Common;
use ds::root::pass_through::ExtraClass;
use ds_settings::schema::KeyPath;

/// A page of the stack a settings page is drawn in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Level {
    /// The page itself.
    Page,
    /// The advanced section titled so.
    Advanced(String),
}

/// The name of `level` on `page`: the page's own title, or the section's name beside the page's
/// basic sections.
pub(crate) fn level_title(level: &Level, page: &PageModel) -> String {
    match level {
        Level::Page => page.title.clone(),
        Level::Advanced(title) => page
            .advanced
            .iter()
            .find(|group| &group.title == title)
            .map_or_else(|| title.clone(), |group| advanced_name(group, &page.sections)),
    }
}

/// The advanced `groups` of the page `slug`, one drill-in row each in a group of their own, named
/// by [`advanced_name`] against the page's `basic` sections. A search hit inside a section
/// highlights its row; a press hands the section's title to `on_open`.
#[component]
pub fn AdvancedView(
    slug: PageSlug,
    groups: Vec<SectionModel>,
    basic: Vec<SectionModel>,
    hits: Vec<KeyPath>,
    on_open: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "advanced", "data-page-advanced": "{slug.0}",
            FieldGroup {
                for group in groups.iter().cloned() {
                    DrillRow {
                        key: "{group.title}",
                        name: advanced_name(&group, &basic),
                        found: if group.keys.iter().any(|entry| is_hit(entry, &hits)) { Match::Hit } else { Match::Miss },
                        title: group.title.clone(),
                        on_open,
                    }
                }
            }
        }
    }
}

/// One section's row: its name and a chevron.
#[component]
fn DrillRow(name: String, title: String, found: Match, on_open: EventHandler<String>) -> Element {
    let class = match found {
        Match::Hit => ExtraClass::parse(HIT_CLASS).ok(),
        Match::Miss => None,
    };
    rsx! {
        Row {
            title: TextLine::from(name),
            accessory: Accessory::Chevron,
            size: RowSize::Settings,
            common: Common { extra_class: class, ..Common::default() },
            onclick: move |_| on_open.call(title.clone()),
        }
    }
}

/// The page pushed for one advanced section: its rows, under the stack's own header.
#[component]
pub(crate) fn AdvancedPage(
    section: SectionModel,
    hits: Vec<KeyPath>,
    files: Signal<Files>,
    onedit: EventHandler<Edit>,
) -> Element {
    let untitled = SectionModel {
        title: String::new(),
        ..section
    };
    rsx! {
        div { class: "advanced-page",
            SectionView { section: untitled, hits, files, onedit }
        }
    }
}
