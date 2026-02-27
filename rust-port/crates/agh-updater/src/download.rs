//! Binary download, checksum verification, and atomic replacement.

use std::path::Path;

use sha2::{Digest, Sha256};

use crate::UpdaterError;

/// Download a file from the given URL and verify its SHA256 checksum.
pub async fn download_and_verify(
    http: &reqwest::Client,
    url: &str,
    expected_sha256: &str,
) -> Result<Vec<u8>, UpdaterError> {
    let bytes = http.get(url).send().await?.bytes().await?;
    let actual_sha256 = compute_sha256(&bytes);
    if actual_sha256 != expected_sha256.to_lowercase() {
        return Err(UpdaterError::ChecksumMismatch {
            expected: expected_sha256.to_owned(),
            actual: actual_sha256,
        });
    }
    Ok(bytes.to_vec())
}

/// Atomically replace the current binary with the new one.
/// The new binary is written to `target.new` then renamed over `target`.
pub async fn atomic_replace(target: &Path, new_binary: &[u8]) -> Result<(), UpdaterError> {
    let tmp = target.with_extension("new");
    tokio::fs::write(&tmp, new_binary).await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&tmp).await?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(&tmp, perms).await?;
    }

    tokio::fs::rename(&tmp, target).await?;
    Ok(())
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_correct() {
        // SHA256 of empty bytes
        let hash = compute_sha256(b"");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[tokio::test]
    async fn test_atomic_replace() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("binary");
        tokio::fs::write(&target, b"old").await.expect("write old");
        atomic_replace(&target, b"new").await.expect("replace");
        let content = tokio::fs::read(&target).await.expect("read");
        assert_eq!(content, b"new");
    }
}
