pub mod v0;
pub mod v1;
use kdl::KdlDocument;
use miette::{IntoDiagnostic, WrapErr};
use colored::Colorize;

pub trait NiuxMigration: Sized {
    fn migrate_to_v1(&self) -> miette::Result<Self>;
}

impl NiuxMigration for KdlDocument {
    fn migrate_to_v1(&self) -> miette::Result<Self> {
        let v0 = serde_kdl2::from_doc::<v0::NiuxConfigV0>(self)
            .inspect_err(|e| eprintln!("{}", e.to_string().red()))
            .into_diagnostic()
            .wrap_err("Failed to deserialize NiuxConfigV0")?;

        let config_paths = v1::ConfigPaths {
            config_path_system: v0.config_paths.config_path_system,
            config_path_home: v0.config_paths.config_path_home,
        };

        let features = v0.features.map(|f| v1::Features {
            nvd_integration: Some(f.nvd_integration),
        });

        let commands = v1::Commands {
            rebuild_home: v0.commands.rebuild_home,
            rebuild_system: v0.commands.rebuild_system,
            update_flake: v0.commands.update_flakes,
            update_inputs: String::new(),
        };

        let result = v1::NiuxConfigV1 {
            version: 1,
            config_paths,
            config_markers: v0.config_markers,
            features,
            environment: v0.environment,
            commands,
        };

        serde_kdl2::to_doc(&result)
            .inspect_err(|e| eprintln!("{}", e.to_string().red()))
            .into_diagnostic()
            .wrap_err("Internal error: Failed to serialize migrated config")
    }
}
