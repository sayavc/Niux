use crate::{
    structures::{
        AutoGenNiuxConfig,
        models::{ConfigPath, HooksPath},
    },
    utils::writer_init,
};
use std::path::PathBuf;
use std::sync::OnceLock;

pub trait ConfigPathKind {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig;
}

impl ConfigPathKind for ConfigPath {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: path,
            hooks_config_path: s.hooks_config_path,
        }
    }
}

impl ConfigPathKind for HooksPath {
    fn transform(s: AutoGenNiuxConfig, path: PathBuf) -> AutoGenNiuxConfig {
        AutoGenNiuxConfig {
            config_path: s.config_path,
            hooks_config_path: path,
        }
    }
}

impl Default for AutoGenNiuxConfig {
    fn default() -> Self {
        Self {
            config_path: "/etc/niux.kdl".into(),
            hooks_config_path: "/etc/niux_hooks.kdl".into(),
        }
    }
}

impl AutoGenNiuxConfig {
    pub fn create<T>(path: PathBuf) -> crate::NiuxResult<()>
    where
        T: ConfigPathKind,
    {
        let s = T::transform(Self::load().unwrap_or_default(), path);
        writer_init(s)?;
        Ok(())
    }

    pub fn init() -> crate::NiuxResult<()> {
        writer_init(Self::default())?;
        Ok(())
    }

    pub fn load() -> crate::NiuxResult<AutoGenNiuxConfig> {
        let content = std::fs::read_to_string("/var/lib/niux/niux_autogen.kdl")
            .map_err(|e| crate::ConfigIoErr::Read {
                path: PathBuf::from("/var/lib/niux_autogen.kdl"),
                e,
            })
            .map_err(crate::IoErr::from)?;

        match knuffel::parse::<Self>("niux_autogen.kdl", &content) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                let mut s = String::new();

                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .unwrap_or_else(|e| panic!("Failed to render diagnostic\nKdl err: {e}"));

                eprintln!("{s}");

                Err(crate::IoErr::from(crate::ConfigIoErr::Parse).into())
            }
        }
    }

    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AutoGenNiuxConfig> = OnceLock::new();
        CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                let mut s = String::new();

                miette::GraphicalReportHandler::new()
                    .render_report(&mut s, &e)
                    .unwrap_or_else(|e| panic!("Failed to render diagnostic\nKdl err: {e}"));

                eprintln!("{s}");
                std::process::exit(1);
            })
        })
    }
}
