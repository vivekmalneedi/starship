use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JavaConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for JavaConfig<'a> {
    fn new() -> Self {
        JavaConfig {
            format: "via [$symbol($version )]($style)",
            disabled: false,
            style: "red dimmed",
            symbol: "☕ ",
            detect_extensions: vec!["java", "class", "jar", "gradle", "clj", "cljc"],
            detect_files: vec![
                "pom.xml",
                "build.gradle.kts",
                "build.sbt",
                ".java-version",
                "deps.edn",
                "project.clj",
                "build.boot",
            ],
            detect_folders: vec![],
        }
    }
}
