use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use crate::action::Action;
use crate::vfs::entry::FileEntry;

/// Asynchronously scan a directory and post results back via channel.
/// This runs on the tokio runtime so the UI thread never blocks.
pub async fn scan_directory(path: PathBuf, tx: UnboundedSender<Action>) {
    match scan_inner(&path).await {
        Ok(entries) => {
            let _ = tx.send(Action::DirectoryLoaded { path, entries });
        }
        Err(e) => {
            let _ = tx.send(Action::OperationError {
                message: format!("Failed to read {}: {}", path.display(), e),
            });
        }
    }
}

async fn scan_inner(path: &PathBuf) -> anyhow::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(path).await?;

    while let Some(dir_entry) = read_dir.next_entry().await? {
        match FileEntry::from_dir_entry(dir_entry).await {
            Ok(entry) => entries.push(entry),
            Err(_) => {
                // Skip entries we can't read (permission denied, etc.)
                continue;
            }
        }
    }

    Ok(entries)
}

/// Load preview content for a path.
/// - Directories: scan and list their entries.
/// - Files: return Empty (blank pane — no file content preview).
/// Accepts a `CancellationToken` — if cancelled (user scrolled past), the task
/// stops early without posting stale results.
/// `show_hidden` filters directory previews to match the global setting.
pub async fn load_preview(
    path: PathBuf,
    tx: UnboundedSender<Action>,
    cancel: CancellationToken,
    show_hidden: bool,
) {
    use crate::ui::preview::PreviewContent;

    // Check cancellation before starting I/O
    if cancel.is_cancelled() {
        return;
    }

    let content = if path.is_dir() {
        // Directory preview: list its entries
        match scan_inner(&path).await {
            Ok(mut entries) => {
                // Check cancellation after scan
                if cancel.is_cancelled() {
                    return;
                }

                // Apply global show_hidden filter
                if !show_hidden {
                    entries.retain(|e| !e.is_hidden);
                }

                // Sort: dirs first, then alphabetical
                entries.sort_by(|a, b| {
                    b.is_dir
                        .cmp(&a.is_dir)
                        .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
                });

                if entries.is_empty() {
                    PreviewContent::EmptyDir
                } else {
                    PreviewContent::Directory(entries)
                }
            }
            Err(e) => {
                let msg = format!("{}", e);
                if msg.contains("Permission denied") {
                    PreviewContent::PermissionDenied
                } else {
                    PreviewContent::Error(msg)
                }
            }
        }
    } else if path.is_symlink() && tokio::fs::metadata(&path).await.is_err() {
        // Broken symlink
        PreviewContent::Error("Broken symlink — target does not exist".to_string())
    } else {
        // File selected — blank pane (no file content preview)
        PreviewContent::Empty
    };

    // Final cancellation check before sending
    if cancel.is_cancelled() {
        return;
    }

    let _ = tx.send(Action::PreviewLoaded { path, content });
}
