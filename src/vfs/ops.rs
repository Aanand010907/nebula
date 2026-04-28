use std::path::PathBuf;
use std::process::{Command, Stdio};

use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

/// Open a file with the OS default application via xdg-open.
/// Spawned as a fully detached background process with null stdio
/// so the TUI is not blocked or corrupted by external output.
pub fn open_file(path: &PathBuf, tx: &UnboundedSender<Action>) {
    match Command::new("xdg-open")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(_) => {
            let _ = tx.send(Action::OperationComplete {
                message: format!(
                    "Opened: {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
            });
        }
        Err(e) => {
            let _ = tx.send(Action::OperationError {
                message: format!(
                    "Failed to open \"{}\": {}",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    e
                ),
            });
        }
    }
}

/// Create a new empty file.
pub async fn create_file(path: PathBuf, tx: UnboundedSender<Action>) {
    match tokio::fs::File::create(&path).await {
        Ok(_) => {
            let _ = tx.send(Action::OperationComplete {
                message: format!("Created file: {}", path.display()),
            });
        }
        Err(e) => {
            let _ = tx.send(Action::OperationError {
                message: format_io_error("create file", &path, e),
            });
        }
    }
}

/// Create a new directory.
pub async fn create_dir(path: PathBuf, tx: UnboundedSender<Action>) {
    match tokio::fs::create_dir(&path).await {
        Ok(_) => {
            let _ = tx.send(Action::OperationComplete {
                message: format!("Created directory: {}", path.display()),
            });
        }
        Err(e) => {
            let _ = tx.send(Action::OperationError {
                message: format_io_error("create directory", &path, e),
            });
        }
    }
}

/// Rename a file or directory.
pub async fn rename(from: PathBuf, to: PathBuf, tx: UnboundedSender<Action>) {
    // Guard: check source still exists
    if !from.exists() && from.symlink_metadata().is_err() {
        let _ = tx.send(Action::OperationError {
            message: format!(
                "Cannot rename — source no longer exists: {}",
                from.file_name().unwrap_or_default().to_string_lossy()
            ),
        });
        return;
    }

    match tokio::fs::rename(&from, &to).await {
        Ok(_) => {
            let _ = tx.send(Action::OperationComplete {
                message: format!(
                    "Renamed: {} → {}",
                    from.file_name().unwrap_or_default().to_string_lossy(),
                    to.file_name().unwrap_or_default().to_string_lossy()
                ),
            });
        }
        Err(e) => {
            let _ = tx.send(Action::OperationError {
                message: format_io_error("rename", &from, e),
            });
        }
    }
}

/// Delete files and directories.
/// Handles externally-deleted files gracefully (NotFound is not an error).
pub async fn delete(paths: Vec<PathBuf>, tx: UnboundedSender<Action>) {
    let count = paths.len();
    let mut deleted = 0;
    let mut errors = Vec::new();
    let mut already_gone = 0;

    for path in &paths {
        // Check if path still exists (might have been deleted externally)
        if !path.exists() && path.symlink_metadata().is_err() {
            already_gone += 1;
            continue;
        }

        let result = if path.is_dir() {
            tokio::fs::remove_dir_all(path).await
        } else {
            tokio::fs::remove_file(path).await
        };

        match result {
            Ok(_) => deleted += 1,
            Err(e) => {
                // NotFound during delete is fine — something else deleted it
                if e.kind() == std::io::ErrorKind::NotFound {
                    already_gone += 1;
                } else {
                    errors.push(format!(
                        "{}: {}",
                        path.file_name().unwrap_or_default().to_string_lossy(),
                        friendly_io_error(e)
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        let mut msg = format!(
            "Deleted {} item{}",
            deleted,
            if deleted == 1 { "" } else { "s" }
        );
        if already_gone > 0 {
            msg.push_str(&format!(" ({} already removed)", already_gone));
        }
        let _ = tx.send(Action::OperationComplete { message: msg });
    } else {
        let _ = tx.send(Action::OperationError {
            message: format!("Delete errors: {}", errors.join("; ")),
        });
    }
}

/// Format an IO error with a user-friendly message.
fn format_io_error(operation: &str, path: &PathBuf, e: std::io::Error) -> String {
    let detail = friendly_io_error(e);
    format!(
        "Failed to {} \"{}\": {}",
        operation,
        path.file_name().unwrap_or_default().to_string_lossy(),
        detail
    )
}

/// Convert raw IO errors to user-friendly descriptions.
fn friendly_io_error(e: std::io::Error) -> String {
    match e.kind() {
        std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
        std::io::ErrorKind::NotFound => "File not found (may have been deleted externally)".to_string(),
        std::io::ErrorKind::AlreadyExists => "Already exists".to_string(),
        _ => format!("{}", e),
    }
}
