//! The caller's side of `org.quire.Portal1`, the members only the Settings app may call.

use crate::bus::Pinned;

/// What the portal remembers (`org.quire.Portal1`): it lists apps and dates, never a token. It is
/// always called at the owner that was confirmed, so it names no service of its own.
#[zbus::proxy(
    interface = "org.quire.Portal1",
    default_path = "/org/freedesktop/portal/desktop"
)]
pub trait Portal {
    /// The apps that may share the screen without asking again, one entry each: app id, the
    /// portal's name for it, the mode (1 while the app runs, 2 until taken back) and when it was
    /// allowed, in seconds since 1970.
    fn list_grants(&self) -> zbus::Result<Vec<(String, String, u32, u64)>>;

    /// Takes back everything remembered for the app: it is asked next time. Answers how many
    /// grants went.
    fn revoke(&self, app_id: &str) -> zbus::Result<u32>;

    /// Lets the app share the screen and take over the keyboard and pointer without asking, until
    /// it is taken back. `screens` is `none`, `every` or `output:<name>`; `input` is the set of
    /// devices (1 keyboard, 2 pointer, 4 touchscreen; 0 none). It replaces what the app had.
    fn allow(&self, app_id: &str, screens: &str, input: u32) -> zbus::Result<()>;
}

/// The portal whose owner was confirmed: calls go to that owner by its unique name.
pub async fn portal(running: &Pinned) -> zbus::Result<PortalProxy<'_>> {
    PortalProxy::builder(&running.connection)
        .destination(running.destination())?
        .build()
        .await
}
