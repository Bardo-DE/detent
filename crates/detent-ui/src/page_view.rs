//! One page: a stack whose root is the page's blocks and whose pushed pages are its advanced
//! sections.

use crate::advanced_view::{AdvancedPage, Level, level_title};
use crate::env::Env;
use crate::files::{Edit, Files};
use crate::live::start;
use crate::memory::MemoryConsent;
use crate::page_body::PageBody;
use detent_model::{Offers, PageModel, PageSlug, holds_memory_settings, modules_read};
use dioxus::prelude::*;
use ds::prelude::*;
use ds_settings::schema::KeyPath;

/// The page `slug`, scrolling; a drill-in row of its advanced keys pushes a page over it.
#[component]
pub fn PageView(
    slug: PageSlug,
    hits: Vec<KeyPath>,
    files: Signal<Files>,
    onedit: EventHandler<Edit>,
) -> Element {
    let env = use_context::<Env>();
    let mut path = use_signal(|| PanePath::new(Level::Page));
    // The daemons this page reads are asked to start once, when the page is shown (the window
    // mounts a page afresh for each visit), and never for a page that was not opened.
    let started = env
        .catalogue
        .page(&slug)
        .map_or(&[][..], |page| modules_read(&page.page));
    use_future({
        let bus = env.bus.clone();
        move || {
            let bus = bus.clone();
            async move { start(&bus, started).await }
        }
    });
    // What waits for the person's word is read each time a page that holds memory settings is
    // shown, since the file may have been edited by hand since.
    let consent = use_context::<MemoryConsent>();
    let holds_memory = env.catalogue.page(&slug).is_some_and(holds_memory_settings);
    use_future(move || {
        let consent = consent.clone();
        async move {
            if holds_memory {
                consent.refresh().await;
                consent.look_at_confinement().await;
            }
        }
    });
    // The choices of the installed Extensions are read once, when a page that has a key choosing
    // among them is shown; coming back to the page after turning one off reads them again.
    let wants_offers = env
        .catalogue
        .page(&slug)
        .is_some_and(PageModel::has_provided);
    let offers = use_hook({
        let extensions = env.extensions.clone();
        move || {
            if wants_offers {
                extensions.read().offers
            } else {
                Offers::default()
            }
        }
    });
    let Some(page) = env.catalogue.page(&slug).map(|page| page.offering(&offers)) else {
        return rsx! {};
    };
    let model = page;
    let title = Callback::new({
        let model = model.clone();
        move |level: Level| level_title(&level, &model)
    });
    let drawn = Callback::new({
        let model = model.clone();
        move |level: Level| match level {
            Level::Page => rsx! {
                PageBody {
                    page: model.clone(),
                    hits: hits.clone(),
                    files,
                    onedit,
                    on_open: move |name: String| path.set(path().pushed(Level::Advanced(name))),
                }
            },
            Level::Advanced(name) => {
                let section = model
                    .advanced
                    .iter()
                    .find(|group| group.title == name)
                    .cloned()
                    .unwrap_or_default();
                rsx! { AdvancedPage { section, hits: hits.clone(), files, onedit } }
            }
        }
    });
    rsx! {
        div { class: "scroll page-stack", "data-page": "{slug.0}",
            PaneStack::<Level> {
                path: path(),
                title,
                page: drawn,
                on_back: move |()| path.set(path().popped()),
            }
        }
    }
}
