//! The sidebar's groups: which pages sit together, in which order, with a gap between groups.

use crate::catalogue::PageModel;
use crate::{
    is_assistant_model, is_bluetooth, is_displays, is_extensions, is_software_update,
    is_wallpaper, is_wifi,
};
use ds_settings::schema::Page;

/// A run of pages the sidebar sets apart from the next with a gap, in the order they are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SidebarGroup {
    /// Wi-Fi and Bluetooth.
    Connection,
    /// Notifications.
    Alerts,
    /// How the desktop looks, the assistant, and what programs may reach.
    Look,
    /// The desktop, its screens, its pictures and its Spaces.
    Desktop,
    /// Power and updates.
    System,
    /// The person's accounts.
    Accounts,
    /// Keyboard and pointing devices.
    Input,
    /// Every page a program brings.
    Programs,
}

/// The group of `page` and its place inside the group.
fn slot_of(page: &Page) -> (SidebarGroup, u8) {
    use SidebarGroup::{Accounts, Alerts, Connection, Desktop, Input, Look, Programs, System};
    match page {
        Page::App(_) if is_wifi(page) => (Connection, 0),
        Page::App(_) if is_bluetooth(page) => (Connection, 1),
        Page::Notifications => (Alerts, 0),
        Page::Appearance => (Look, 0),
        Page::Intelligence => (Look, 1),
        Page::App(_) if is_assistant_model(page) => (Look, 2),
        Page::App(_) if is_extensions(page) => (Look, 3),
        Page::Privacy => (Look, 4),
        Page::Dock => (Desktop, 0),
        Page::App(_) if is_displays(page) => (Desktop, 1),
        Page::App(_) if is_wallpaper(page) => (Desktop, 2),
        Page::Spaces => (Desktop, 3),
        Page::Power => (System, 0),
        Page::App(_) if is_software_update(page) => (System, 1),
        Page::Accounts => (Accounts, 0),
        Page::KeyboardAndShortcuts => (Input, 0),
        Page::MouseAndGestures => (Input, 1),
        Page::Apps | Page::Viewer | Page::App(_) => (Programs, 0),
    }
}

/// The group `page` sits in.
pub fn group_of(page: &Page) -> SidebarGroup {
    slot_of(page).0
}

/// `pages` in groups, each group in its own order; a group with no page is left out. The pages of
/// the last group keep the order they came in.
pub fn sidebar_groups<'a>(pages: &[&'a PageModel]) -> Vec<Vec<&'a PageModel>> {
    let mut sorted: Vec<&PageModel> = pages.to_vec();
    sorted.sort_by_key(|page| slot_of(&page.page));
    let mut groups: Vec<Vec<&PageModel>> = Vec::new();
    let mut last = None;
    for page in sorted {
        let group = group_of(&page.page);
        if last != Some(group) {
            groups.push(Vec::new());
            last = Some(group);
        }
        if let Some(open) = groups.last_mut() {
            open.push(page);
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::{SidebarGroup, group_of, sidebar_groups};
    use crate::catalogue::PageModel;
    use crate::page::PageSlug;
    use ds_settings::schema::Page;

    fn model(page: Page) -> PageModel {
        PageModel {
            slug: PageSlug::of(&page),
            title: String::new(),
            page,
            sections: Vec::new(),
            advanced: Vec::new(),
        }
    }

    fn all() -> Vec<PageModel> {
        [
            Page::App("zed".to_owned()),
            Page::Viewer,
            crate::updates::software_update_page(),
            Page::MouseAndGestures,
            Page::KeyboardAndShortcuts,
            Page::Accounts,
            Page::Power,
            Page::Spaces,
            Page::declared("wallpaper"),
            crate::displays_page(),
            Page::Dock,
            Page::Privacy,
            crate::extensions_page(),
            crate::assistant_model_page(),
            Page::Intelligence,
            Page::Appearance,
            Page::Notifications,
            crate::bluetooth_page(),
            crate::wifi_page(),
        ]
        .into_iter()
        .map(model)
        .collect()
    }

    #[test]
    fn pages_come_in_the_groups_and_order_the_sidebar_draws() {
        let pages = all();
        let refs: Vec<&PageModel> = pages.iter().collect();
        let drawn: Vec<Vec<String>> = sidebar_groups(&refs)
            .iter()
            .map(|group| group.iter().map(|page| page.slug.0.clone()).collect())
            .collect();
        let expected: &[&[&str]] = &[
            &["wifi", "bluetooth"],
            &["notifications"],
            &["appearance", "intelligence", "assistant-model", "extensions", "privacy"],
            &["dock", "displays", "wallpaper", "spaces"],
            &["power", "software-update"],
            &["accounts"],
            &["keyboard_and_shortcuts", "mouse_and_gestures"],
            &["viewer", "app:zed"],
        ];
        assert_eq!(drawn, expected);
    }

    #[test]
    fn a_group_with_no_page_leaves_no_gap() {
        let pages = vec![model(Page::Power), model(Page::Notifications)];
        let refs: Vec<&PageModel> = pages.iter().collect();
        assert_eq!(sidebar_groups(&refs).len(), 2);
        assert_eq!(group_of(&Page::Power), SidebarGroup::System);
    }
}
