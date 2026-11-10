//! The apps that can be allowed to share the screen without asking: the installed ones a system
//! desktop entry lists, that are not in the list yet.

use super::grant::ScreenGrant;
use crate::origin::SchemaOrigin;
use crate::privacy::{EntryVisibility, ProgramBook, ProgramSubject};
use std::collections::BTreeSet;

/// The apps of `book` that may be allowed, by name (then id): each has a system entry that is
/// listed, and is not among `grants`. An app is allowed by its entry's id, so an entry anyone
/// but an administrator can change is never offered. An id in more than one folder is offered once.
pub fn allow_candidates(book: &ProgramBook, grants: &[ScreenGrant]) -> Vec<ProgramSubject> {
    let mut seen: BTreeSet<&str> = grants.iter().map(|grant| grant.app.0.as_str()).collect();
    let mut found: Vec<(String, ProgramSubject)> = book
        .apps
        .iter()
        .filter(|app| {
            app.origin == SchemaOrigin::System && app.visibility == EntryVisibility::Listed
        })
        .filter(|app| seen.insert(app.id.as_str()))
        .map(|app| {
            let subject = ProgramSubject(app.id.clone());
            let name = book.look_of(&subject).name.unwrap_or_default();
            (name.to_lowercase(), subject)
        })
        .collect();
    found.sort();
    found.into_iter().map(|(_, subject)| subject).collect()
}
