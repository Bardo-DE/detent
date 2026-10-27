//! The coloured tile beside each page in the sidebar: one glyph on one ground, as System
//! Settings has them. The grounds are one table here; nothing else names a tile colour.

use detent_model::{
    is_assistant_model, is_bluetooth, is_displays, is_extensions, is_software_update, is_wallpaper,
    is_wifi,
};
use ds::prelude::{Icon, TileFace};
use ds::style::tokens::hex::Hex;
use ds_settings::schema::Page;

/// The ground a tile is painted on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ground {
    /// Connection, screens, privacy and accounts.
    Blue,
    /// Notifications.
    Red,
    /// Appearance, and the desktop and Dock.
    Graphite,
    /// Wallpaper.
    Cyan,
    /// Power.
    Green,
    /// The plain system pages: updates, input devices, extensions, programs.
    Grey,
    /// The assistant and Spaces.
    Indigo,
}

impl Ground {
    /// The ground's colour.
    pub(crate) const fn hex(self) -> Hex {
        match self {
            Ground::Blue => Hex([0x0a, 0x84, 0xff]),
            Ground::Red => Hex([0xff, 0x3b, 0x30]),
            Ground::Graphite => Hex([0x3a, 0x3a, 0x3c]),
            Ground::Cyan => Hex([0x32, 0xad, 0xe6]),
            Ground::Green => Hex([0x34, 0xc7, 0x59]),
            Ground::Grey => Hex([0x8e, 0x8e, 0x93]),
            Ground::Indigo => Hex([0x5e, 0x5c, 0xe6]),
        }
    }
}

/// The glyph and the ground of `page`.
pub(crate) fn look_of(page: &Page) -> (Icon, Ground) {
    match page {
        Page::App(_) if is_wifi(page) => (Icon::Wifi, Ground::Blue),
        Page::App(_) if is_bluetooth(page) => (Icon::Bluetooth, Ground::Blue),
        Page::Notifications => (Icon::Bell, Ground::Red),
        Page::Appearance => (Icon::SunMoon, Ground::Graphite),
        Page::Intelligence => (Icon::Sparkles, Ground::Indigo),
        Page::App(_) if is_assistant_model(page) => (Icon::Switches, Ground::Indigo),
        Page::App(_) if is_extensions(page) => (Icon::Puzzle, Ground::Grey),
        Page::Privacy => (Icon::Hand, Ground::Blue),
        Page::Dock => (Icon::Panel, Ground::Graphite),
        Page::App(_) if is_displays(page) => (Icon::Monitor, Ground::Blue),
        Page::App(_) if is_wallpaper(page) => (Icon::Image, Ground::Cyan),
        Page::Spaces => (Icon::Group, Ground::Indigo),
        Page::Power => (Icon::Power, Ground::Green),
        Page::App(_) if is_software_update(page) => (Icon::Download, Ground::Grey),
        Page::Accounts => (Icon::Key, Ground::Blue),
        Page::KeyboardAndShortcuts => (Icon::Keyboard, Ground::Grey),
        Page::MouseAndGestures => (Icon::Mouse, Ground::Grey),
        Page::Apps => (Icon::Grid, Ground::Grey),
        Page::Viewer => (Icon::Image, Ground::Grey),
        Page::App(_) => (Icon::Window, Ground::Grey),
    }
}

/// The sidebar tile of `page`.
pub(crate) fn tile_of(page: &Page) -> TileFace {
    let (icon, ground) = look_of(page);
    TileFace::Glyph(icon, ground.hex())
}

#[cfg(test)]
mod tests {
    use super::{Ground, look_of, tile_of};
    use ds::prelude::{Icon, TileFace};
    use ds_settings::schema::Page;

    fn pages() -> Vec<Page> {
        vec![
            Page::Appearance,
            Page::Dock,
            Page::MouseAndGestures,
            Page::KeyboardAndShortcuts,
            Page::Notifications,
            Page::Spaces,
            Page::Accounts,
            Page::Privacy,
            Page::Power,
            Page::Apps,
            Page::Intelligence,
            Page::Viewer,
            detent_model::wifi_page(),
            detent_model::bluetooth_page(),
            detent_model::displays_page(),
            detent_model::extensions_page(),
            detent_model::assistant_model_page(),
            Page::declared("wallpaper"),
            Page::App("Software Update".to_owned()),
            Page::App("files".to_owned()),
        ]
    }

    #[test]
    fn every_page_has_a_coloured_tile() {
        for page in pages() {
            assert!(
                matches!(tile_of(&page), TileFace::Glyph(..)),
                "{page:?} has a glyph tile"
            );
        }
    }

    #[test]
    fn intelligence_and_the_assistant_model_have_different_glyphs() {
        let glyph = |page: &Page| look_of(page).0;
        assert_ne!(
            glyph(&Page::Intelligence),
            glyph(&detent_model::assistant_model_page())
        );
        assert_eq!(glyph(&Page::Intelligence), Icon::Sparkles);
    }

    #[test]
    fn the_grounds_are_all_different() {
        let grounds = [
            Ground::Blue,
            Ground::Red,
            Ground::Graphite,
            Ground::Cyan,
            Ground::Green,
            Ground::Grey,
            Ground::Indigo,
        ];
        for (at, one) in grounds.iter().enumerate() {
            for other in &grounds[at + 1..] {
                assert_ne!(one.hex(), other.hex(), "{one:?} and {other:?}");
            }
        }
    }
}
