//! The screen-sharing portal from memory: the apps it lets share, and what was taken back.

use super::ScreenSharingLink;
use crate::fake_kit::Kit;
use crate::lock::locked;
use crate::updates::NOW;
use detent_model::{Absence, ProgramSubject, ScreenGrant, ShareLength, Work};
use std::sync::{Arc, Mutex};

/// A portal that answers from what it was given.
#[derive(Debug, Clone)]
pub struct FakeScreenSharing {
    kit: Arc<Kit<Vec<ScreenGrant>, ProgramSubject>>,
    allowed: Arc<Mutex<Vec<ProgramSubject>>>,
}

impl FakeScreenSharing {
    /// A portal that lets these apps share.
    pub fn granting(grants: Vec<ScreenGrant>) -> FakeScreenSharing {
        FakeScreenSharing {
            kit: Kit::new(Ok(grants)),
            allowed: Arc::default(),
        }
    }

    /// No portal, for this reason.
    pub fn missing(why: Absence) -> FakeScreenSharing {
        FakeScreenSharing {
            kit: Kit::new(Err(why)),
            allowed: Arc::default(),
        }
    }

    /// The next revokes and allows are refused, in these words.
    pub fn refuses(&self, words: &str) {
        self.kit.refuse_with(Some(words.to_owned()));
    }

    /// The apps that were taken back, in order.
    pub fn revoked(&self) -> Vec<ProgramSubject> {
        self.kit.asked()
    }

    /// The apps that were let in, in order.
    pub fn allowed(&self) -> Vec<ProgramSubject> {
        locked(&self.allowed).clone()
    }
}

impl ScreenSharingLink for FakeScreenSharing {
    fn list(&self) -> Work<'_, Result<Vec<ScreenGrant>, Absence>> {
        Box::pin(std::future::ready(self.kit.standing()))
    }

    fn revoke(&self, app: ProgramSubject) -> Work<'_, Result<(), String>> {
        self.kit.record(app.clone());
        let done = self.kit.admit();
        if done.is_ok() {
            self.kit
                .edit(|grants| grants.retain(|grant| grant.app != app));
        }
        Box::pin(std::future::ready(done))
    }

    fn allow(&self, app: ProgramSubject) -> Work<'_, Result<(), String>> {
        locked(&self.allowed).push(app.clone());
        let done = self.kit.admit();
        if done.is_ok() {
            self.kit.edit(|grants| {
                grants.retain(|grant| grant.app != app);
                grants.push(ScreenGrant {
                    app,
                    length: ShareLength::UntilRemoved,
                    granted: NOW,
                });
            });
        }
        Box::pin(std::future::ready(done))
    }
}
