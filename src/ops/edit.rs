use crate::structures::NiuxConfig;
use crate::structures::models::{Home, Just, Package, System, Target};
use crate::utils::{PathExt, run_bash_interactive, write_changes_to_config};
use colored::Colorize;
use tempfile::NamedTempFile;
impl Package {
    pub fn edit(&self) -> crate::NiuxResult<()> {
        let config = NiuxConfig::get();

        let config_path = match self.ptype {
            Target::Home => &config.config_paths.home,
            Target::System => &config.config_paths.system,
            _ => unreachable!(),
        };

        let content = config_path.read_to_string()?;

        let range = match self.ptype {
            Target::Home => config.get_range::<Home>(&content),
            Target::System => config.get_range::<System>(&content),
            _ => unreachable!(),
        }?;

        let old_packages = range.packages.join("\n");

        let tmp = NamedTempFile::new()
            .map_err(|e| crate::TmpErr::Create { e })
            .map_err(crate::IoErr::from)?;

        std::fs::write(tmp.path(), &old_packages)
            .map_err(|e| crate::TmpErr::Write { e })
            .map_err(crate::IoErr::from)?;

        run_bash_interactive::<Just>(&[
            &config.environment.editor,
            tmp.path().to_str().ok_or(crate::Utf8Err::InvalidUtf8)?,
        ])?;

        let new = std::fs::read_to_string(tmp.path())
            .map_err(|e| crate::TmpErr::Read { e })
            .map_err(crate::IoErr::from)?
            .trim_end()
            .to_string();

        let new_packages: Vec<String> = new.lines().map(String::from).collect();

        if old_packages == new {
            println!("{}", "Nothing has changed...".yellow());
            return Ok(());
        }

        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        lines.splice(range.start..range.end, new_packages);

        let new_content = lines.join("\n");

        write_changes_to_config(&new_content, config_path.to_path_buf())?;

        println!("{}", "Packages edited".green());

        Ok(())
    }
}
