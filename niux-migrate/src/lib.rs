mod config;
mod adaptors;
use config::niux_config::NiuxMigration;
use kdl::KdlDocument;
use miette::bail;

pub use config::niux_config::v1::NiuxConfigV1 as CurrentConfig;

pub fn migration(mut doc: KdlDocument, from: u32, to: u32) -> miette::Result<KdlDocument> {
    if from >= to {
        bail!("Target version {to} must me greater than or equal to version {from}")
    }
    for i in from..to {
        match i {
            0 => doc = doc.migrate_to_v1()?,
            _ => bail!("Unsupported migration version {i}"),
        }
    }
    Ok(doc)
}
