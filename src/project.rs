use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project_name: String,
    pub default_template: String,
    pub strict_audit: bool,
}

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub root_path: PathBuf,
    pub config: ProjectConfig,
    pub templates: Templates,
    pub rules: ProjectRules,
}

#[derive(Debug, Clone)]
pub struct Templates {
    pub default: String,
    pub technical: String,
}

#[derive(Debug, Clone)]
pub struct ProjectRules {
    pub audit_rules: String,
    pub generation_rules: String,
}

impl ProjectContext {
    pub fn find_current() -> Result<Option<Self>> {
        let current_dir = std::env::current_dir()?;
        Self::find_in_directory(&current_dir)
    }

    fn find_in_directory(dir: &Path) -> Result<Option<Self>> {
        let prd_dir = dir.join(".prd");
        if prd_dir.exists() {
            Some(Self::load_from(&prd_dir)).transpose()
        } else {
            if let Some(parent) = dir.parent() {
                Self::find_in_directory(parent)
            } else {
                Ok(None)
            }
        }
    }

    fn load_from(prd_dir: &Path) -> Result<Self> {
        let config_path = prd_dir.join("config.toml");
        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| Error::Project(format!("Failed to read config: {}", e)))?;

        let config: ProjectConfig = toml::from_str(&config_content)
            .map_err(|e| Error::Project(format!("Invalid config: {}", e)))?;

        let templates = Templates::load_from(&prd_dir.join("templates"))?;
        let rules = ProjectRules::load_from(&prd_dir.join("rules"))?;

        Ok(Self {
            root_path: prd_dir.parent().unwrap().to_path_buf(),
            config,
            templates,
            rules,
        })
    }

    pub fn ensure_project_context() -> Result<Self> {
        Self::find_current()?.ok_or_else(|| {
            Error::Project(
                "No PRD project found. Run `prd-assistant init <project-name>` first.".into(),
            )
        })
    }
}

impl Templates {
    fn load_from(templates_dir: &Path) -> Result<Self> {
        let default_path = templates_dir.join("default.md");
        let technical_path = templates_dir.join("technical.md");

        let default = std::fs::read_to_string(&default_path)
            .map_err(|e| Error::Template(format!("Failed to load default template: {}", e)))?;

        let technical = std::fs::read_to_string(&technical_path)
            .map_err(|e| Error::Template(format!("Failed to load technical template: {}", e)))?;

        Ok(Self { default, technical })
    }
}

impl ProjectRules {
    fn load_from(rules_dir: &Path) -> Result<Self> {
        let audit_rules_path = rules_dir.join("audit_rules.md");
        let generation_rules_path = rules_dir.join("generation_rules.md");

        let audit_rules = std::fs::read_to_string(&audit_rules_path)
            .unwrap_or_else(|_| DEFAULT_AUDIT_RULES.into());

        let generation_rules = std::fs::read_to_string(&generation_rules_path)
            .unwrap_or_else(|_| DEFAULT_GENERATION_RULES.into());

        Ok(Self {
            audit_rules,
            generation_rules,
        })
    }
}

const DEFAULT_AUDIT_RULES: &str = r#"# Default Audit Rules

## Required Sections
- Overview
- Requirements  
- Success Metrics

## Common Issues to Flag
- Vague language like \"fast\" or \"user-friendly\"
- Missing error handling
- No success criteria
"#;

const DEFAULT_GENERATION_RULES: &str = r#"# Default Generation Rules

## Always Include
- Clear problem statement
- Specific user stories
- Measurable success metrics
"#;
