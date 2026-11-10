//! The list: a row for each app with its name, when it was allowed, and Remove; or the line that
//! says no app is. Under it, the installed apps that can be allowed, each with Allow.

use crate::copy::privacy::program_name;
use crate::copy::screen_sharing::grant_line;
use crate::key_row::row_id;
use detent_model::{
    ProgramBook, ProgramSubject, ScreenGrant, ScreenSharingInput, ScreenSharingReady, UnixSeconds,
    allow_candidates,
};
use dioxus::prelude::*;
use ds::components::content::label::{LabelRole, LabelStyle};
use ds::components::controls::button_model::ButtonRole;
use ds::components::controls::disclosure::Disclosure;
use ds::components::fields::field_row::{FieldGroup, FieldRow};
use ds::components::overlays::empty_state::EmptyState;
use ds::prelude::*;
use ds::root::common::Common;
use ds::style::tokens::control_size::ControlSize;

/// The apps, or the line that says none can share the screen without asking.
#[component]
pub(super) fn Listed(
    ready: ScreenSharingReady,
    book: ProgramBook,
    now: UnixSeconds,
    send: EventHandler<ScreenSharingInput>,
) -> Element {
    let busy = if ready.is_changing() {
        Availability::Disabled
    } else {
        Availability::Enabled
    };
    let candidates: Vec<(ProgramSubject, String)> = allow_candidates(&book, &ready.grants)
        .into_iter()
        .map(|app| {
            let name = program_name(&book.look_of(&app));
            (app, name)
        })
        .collect();
    rsx! {
        div { class: "screen-sharing", "data-screen-sharing": "list",
            if ready.grants.is_empty() {
                EmptyState {
                    title: crate::copy::screen_sharing::NO_APPS_CAN_SHARE_YOUR_SCREEN,
                    description: crate::copy::screen_sharing::NO_REMEMBERED_APPS_HINT,
                }
            } else {
                FieldGroup { title: Some(crate::copy::screen_sharing::SCREEN_SHARING.to_owned()),
                    for grant in ready.grants.iter().cloned() {
                        AppLine {
                            key: "{grant.app.0}",
                            name: program_name(&book.look_of(&grant.app)),
                            line: grant_line(&grant, now),
                            grant,
                            availability: busy,
                            remove: send,
                        }
                    }
                }
            }
            AllowApps { candidates, availability: busy, allow: send }
        }
    }
}

/// The apps that can be let in without asking, under a disclosure, each with Allow.
#[component]
fn AllowApps(
    candidates: Vec<(ProgramSubject, String)>,
    availability: Availability,
    allow: EventHandler<ScreenSharingInput>,
) -> Element {
    let mut opened = use_signal(|| Shown::Hidden);
    rsx! {
        div { class: "screen-sharing-allow", "data-screen-sharing": "allow",
            FieldGroup { title: Some(crate::copy::screen_sharing::LET_AN_APP_IN.to_owned()),
                Label {
                    text: crate::copy::screen_sharing::LET_AN_APP_IN_HELP,
                    role: LabelRole::Secondary,
                    style: LabelStyle::Footnote,
                }
                Disclosure {
                    label: Some(crate::copy::screen_sharing::ALLOW_AN_APP.to_owned()),
                    common: Common { id: Some("screen-sharing-allow".to_owned()), ..Common::default() },
                    shown: opened(),
                    on_toggle: move |next| opened.set(next),
                    if candidates.is_empty() {
                        Label {
                            text: crate::copy::screen_sharing::NO_APPS_TO_ALLOW,
                            role: LabelRole::Secondary,
                            style: LabelStyle::Footnote,
                        }
                    } else {
                        FieldGroup {
                            for (app, name) in candidates {
                                AllowLine { key: "{app.0}", app: app.clone(), name, availability, allow }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One app that can be let in: its name and Allow.
#[component]
fn AllowLine(
    app: ProgramSubject,
    name: String,
    availability: Availability,
    allow: EventHandler<ScreenSharingInput>,
) -> Element {
    rsx! {
        FieldRow {
            label: TextLine::from(name.clone()),
            common: Common { id: Some(row_id("screen-sharing-allow", &app.0)), ..Common::default() },
            Button {
                common: Common {
                    aria_label: Some(crate::copy::screen_sharing::allow(&name)),
                    ..Common::default()
                },
                label: crate::copy::screen_sharing::ALLOW,
                size: ControlSize::Small,
                availability,
                onclick: move |_| allow.call(ScreenSharingInput::Allow(app.clone())),
            }
        }
    }
}

/// One app: its name, when it was allowed, and Remove.
#[component]
fn AppLine(
    grant: ScreenGrant,
    name: String,
    line: String,
    availability: Availability,
    remove: EventHandler<ScreenSharingInput>,
) -> Element {
    let app: ProgramSubject = grant.app.clone();
    rsx! {
        FieldRow {
            label: TextLine::from(name.clone()),
            help: Some(TextLine::from(line)),
            common: Common { id: Some(row_id("screen-sharing", &app.0)), ..Common::default() },
            Button {
                common: Common {
                    aria_label: Some(crate::copy::screen_sharing::remove(&name)),
                    ..Common::default()
                },
                label: crate::copy::common::REMOVE,
                role: ButtonRole::Destructive,
                size: ControlSize::Small,
                availability,
                onclick: move |_| remove.call(ScreenSharingInput::Remove(app.clone())),
            }
        }
    }
}
