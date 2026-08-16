#[allow(dead_code)]
fn transform(s: &str) -> String {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}").expect("internal regex is invalid")
    });

    let res = re.replace_all(s, |caps: &regex::Captures| {
        std::env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    });
    res.into_owned()
}

#[allow(dead_code)]
pub trait ReplaceEnv {
    fn replace_env(&mut self);
}

use std::path::PathBuf;
impl ReplaceEnv for PathBuf {
    fn replace_env(&mut self) {
        *self = PathBuf::from(transform(&self.to_string_lossy()))
    }
}

impl ReplaceEnv for String {
    fn replace_env(&mut self) {
        *self = transform(self)
    }
}

impl<T: ReplaceEnv> ReplaceEnv for Vec<T> {
    fn replace_env(&mut self) {
        for item in self {
            item.replace_env();
        }
    }
}

impl<T: ReplaceEnv> ReplaceEnv for Option<T> {
    fn replace_env(&mut self) {
        if let Some(val) = self {
            val.replace_env();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use niux_macros::replace_env;

    #[derive(replace_env)]
    struct Config {
        just: String,
        path: PathBuf,
        no_env: String,
        broken_env: PathBuf,
        one_broken_env: String,
        trash_symbols: PathBuf,
    }

    #[test]
    fn replace() {
        unsafe {
            std::env::set_var("REPLACE_VAR_TEST", "ABC");
        }
        let mut cfg = Config {
            just: String::from("${REPLACE_VAR_TEST}"),
            path: PathBuf::from("/home/${REPLACE_VAR_TEST}/fire"),
            no_env: String::from("blablalba"),
            broken_env: PathBuf::from("${123ABC}"),
            one_broken_env: String::from("just: ${REPLACE_VAR_TEST}, broken: ${1RE}"),
            trash_symbols: PathBuf::from("123^;&}$${REPLACE_VAR_TEST}"),
        };

        cfg.replace_env();

        assert_eq!(cfg.just, String::from("ABC"));
        assert_eq!(cfg.path, PathBuf::from("/home/ABC/fire"));
        assert_eq!(cfg.no_env, String::from("blablalba"));
        assert_eq!(cfg.broken_env, PathBuf::from("${123ABC}"));
        assert_eq!(
            cfg.one_broken_env,
            String::from("just: ABC, broken: ${1RE}")
        );
        assert_eq!(cfg.trash_symbols, PathBuf::from("123^;&}$ABC"));
    }
}
