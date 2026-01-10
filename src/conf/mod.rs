use std::{
    fs::{create_dir_all, write},
    path::Path,
    process,
};

use miette::{IntoDiagnostic, miette};

pub const REFRESH_SECS: u64 = 86400;
pub const INVALIDATE_SECS: u64 = 86400 * 7;

#[derive(knuffel::Decode, Debug, PartialEq, Default)]
pub struct Conf {
    #[knuffel(child, unwrap(argument), default)]
    pub drive_id: Option<String>,
    #[knuffel(child, unwrap(argument), default)]
    pub client_id: Option<String>,
    #[knuffel(child, unwrap(argument), default)]
    pub client_secret: Option<String>,
}

impl Conf {
    fn load(path: &Path) -> miette::Result<Self> {
        let contents = match std::fs::read_to_string(path).into_diagnostic() {
            Ok(contents) => contents,
            Err(err) => {
                tracing::debug!("failed to read config from {path:?}: {err}");
                return Err(err);
            }
        };

        let config: Conf = knuffel::parse(
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("config.kdl"),
            &contents,
        )
        .map_err(|e| miette!(e))?;

        tracing::debug!("loaded config from {path:?}");
        Ok(config)
    }

    pub fn load_or_create() -> miette::Result<Self> {
        const PATH: &str = "config.kdl";

        let path = dirs::config_dir()
            .expect("Failed to get config directory")
            .join("gugugaga")
            .join(PATH);
        if !path.exists() {
            tracing::info!("config file {PATH} does not exist, creating default config");
            create_dir_all(path.parent().unwrap()).into_diagnostic()?;
            write(path, include_str!("example.kdl")).into_diagnostic()?;
            process::exit(0);
        }

        Self::load(&path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conf() {
        let conf = r#"
            drive-id "123456"
        "#;
        let conf: Conf = knuffel::parse("example.kdl", conf).unwrap();
        assert_eq!(conf.drive_id, Some("123456".to_string()));
        assert_eq!(conf.client_secret, None);
        assert_eq!(conf.client_id, None);
    }
}
