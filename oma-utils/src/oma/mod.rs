use once_cell::sync::Lazy;
use std::path::PathBuf;

type IOResult<T> = std::io::Result<T>;
static LOCK: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/run/lock/oma.lock"));

/// lock oma
pub fn lock_oma() -> IOResult<()> {
    if !LOCK.is_file() {
        std::fs::create_dir_all("/run/lock")?;
        std::fs::File::create(&*LOCK)?;
    }

    Ok(())
}

/// Unlock oma
pub fn unlock_oma() -> IOResult<()> {
    if LOCK.is_file() {
        std::fs::remove_file(&*LOCK)?;
    }

    Ok(())
}

/// terminal bell character
pub fn terminal_ring() {
    eprint!("\x07"); // bell character
}