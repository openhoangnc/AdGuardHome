// Integration test helpers.
// Tests in this module are marked #[ignore] pending TASK-40 (agh-main wiring).

use std::sync::Arc;

pub async fn test_config() -> Arc<agh_core::config_io::ConfigManager> {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg_path = dir.path().join("AdGuardHome.yaml");
    // Leak the tempdir so it lives for the duration of the test.
    std::mem::forget(dir);
    Arc::new(agh_core::config_io::ConfigManager::load(&cfg_path).await.expect("load config"))
}
