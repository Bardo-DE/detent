//! The apps the screen-sharing portal lets share the screen without asking again, as the Privacy
//! page meets them: the list, and the changes a person makes to it, taking an app back so it asks
//! next time and letting another in. Pure; the portal is the window's (`detent-ui`).

mod candidates;
mod grant;
mod rows;
mod step;

pub use candidates::allow_candidates;
pub use grant::{APP_ID_MAX, GRANTS_MAX, ScreenGrant, ShareLength};
pub use rows::{ScreenGrantRow, screen_grants_of};
pub use step::{
    ScreenSharingEffect, ScreenSharingInput, ScreenSharingReady, ScreenSharingState,
    screen_sharing_step,
};

#[cfg(test)]
mod tests;
