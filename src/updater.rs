//! Automatic update support was removed from this build.
//!
//! The module is kept only so that the remaining call sites stay untouched,
//! every entry point below is a no-op.

use hbb_common::ResultType;
use std::path::PathBuf;

/// Kept for `ipc.rs`, no automatic update is running any more.
pub fn update_controlling_session_count(_count: usize) {}

/// Previously started the hourly background updater thread.
pub fn start_auto_update() {}

/// Previously triggered a manual update download + install.
pub fn manually_check_update() -> ResultType<()> {
    Ok(())
}

/// Previously stopped the background updater thread.
pub fn stop_auto_update() {}

/// Update downloads are gone, so there is no local file for an url.
pub fn get_download_file_from_url(_url: &str) -> Option<PathBuf> {
    None
}
