use std::path::PathBuf;

pub const STORAGE_DB_FILE_NAME: &str = "dbx.db";

fn data_dir_override(primary: Option<std::ffi::OsString>, legacy: Option<std::ffi::OsString>) -> Option<PathBuf> {
    primary.filter(|value| !value.is_empty()).or_else(|| legacy.filter(|value| !value.is_empty())).map(PathBuf::from)
}

/// Mirrors `dirs::data_dir()` (same call the Tauri desktop app makes) so MCP/CLI and the desktop
/// app resolve the same `dbx.db`, including under `XDG_DATA_HOME` on Linux.
pub fn app_data_dir() -> Result<PathBuf, String> {
    if let Some(path) = data_dir_override(std::env::var_os("PANDB_DATA_DIR"), std::env::var_os("DBX_DATA_DIR")) {
        return Ok(path);
    }

    let base = dirs::data_dir()
        .ok_or_else(|| "Unable to resolve the user data directory. Set PANDB_DATA_DIR explicitly.".to_string())?;
    let pandb_data_dir = base.join("com.activeinai.pandb");
    let legacy_data_dir = base.join("com.dbx.app");
    if pandb_data_dir.join(STORAGE_DB_FILE_NAME).is_file() || !legacy_data_dir.join(STORAGE_DB_FILE_NAME).is_file() {
        Ok(pandb_data_dir)
    } else {
        Ok(legacy_data_dir)
    }
}

pub fn storage_db_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join(STORAGE_DB_FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pandb_data_dir_override_wins_over_legacy_dbx_data_dir() {
        let resolved = data_dir_override(Some("/tmp/pandb-mcp-data".into()), Some("/tmp/dbx-mcp-data".into()));
        assert_eq!(resolved, Some(PathBuf::from("/tmp/pandb-mcp-data")));
    }

    #[test]
    fn legacy_dbx_data_dir_remains_supported() {
        let resolved = data_dir_override(None, Some("/tmp/dbx-mcp-data".into()));
        assert_eq!(resolved, Some(PathBuf::from("/tmp/dbx-mcp-data")));
    }
}
