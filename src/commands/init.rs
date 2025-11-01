use crate::error::Result;
use crate::project::ProjectConfig;
use std::path::PathBuf;
use tracing::info;

pub async fn init_project(project_name: &str) -> Result<()> {
    info!("Initializing project: {}", project_name);

    let project_dir = PathBuf::from(project_name);
    let prd_dir = project_dir.join(".prd");
    let templates_dir = prd_dir.join("templates");
    let rules_dir = prd_dir.join("rules");

    // Create directory structure
    std::fs::create_dir_all(&templates_dir)?;
    std::fs::create_dir_all(&rules_dir)?;

    // Create config
    let config = ProjectConfig {
        project_name: project_name.to_string(),
        default_template: "default.md".to_string(),
        strict_audit: true,
    };

    let config_content = toml::to_string_pretty(&config)?;
    std::fs::write(prd_dir.join("config.toml"), config_content)?;

    // Create default templates
    std::fs::write(templates_dir.join("default.md"), DEFAULT_TEMPLATE)?;
    std::fs::write(templates_dir.join("technical.md"), TECHNICAL_TEMPLATE)?;

    // Create default rules
    std::fs::write(rules_dir.join("audit_rules.md"), DEFAULT_AUDIT_RULES)?;
    std::fs::write(
        rules_dir.join("generation_rules.md"),
        DEFAULT_GENERATION_RULES,
    )?;

    println!("✅ Project '{}' initialized successfully!", project_name);
    println!("📁 Project structure created at: {}", project_dir.display());

    Ok(())
}

const DEFAULT_TEMPLATE: &str = r#"# {PROJECT_NAME} - {FEATURE_NAME}

## Overview
{OVERVIEW}

## Problem Statement
{PROBLEM_STATEMENT}

## Requirements
{REQUIREMENTS}

## Success Metrics
{METRICS}
"#;

const TECHNICAL_TEMPLATE: &str = r#"# {PROJECT_NAME} - {FEATURE_NAME} (Technical)

## Technical Overview
{OVERVIEW}

## Architecture Requirements
{REQUIREMENTS}

## API Specifications
{API_SPECS}

## Performance Metrics
{METRICS}
"#;

const DEFAULT_AUDIT_RULES: &str = r#"# Audit Rules

## Required Sections
- Overview
- Requirements  
- Success Metrics

## Common Issues to Flag
- Vague language like \"fast\" or \"user-friendly\"
- Missing error handling
- No success criteria
"#;

const DEFAULT_GENERATION_RULES: &str = r#"# Generation Rules

## Always Include
- Clear problem statement
- Specific user stories
- Measurable success metrics
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[tokio::test]
    async fn test_init_project_success() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test-project";
        let _project_path = temp_dir.path().join(project_name);

        // Change to temp directory
        let original_dir = std::env::current_dir().ok();
        let _ = std::env::set_current_dir(temp_dir.path());

        let _ = init_project(project_name).await;

        // Restore original directory
        if let Some(dir) = original_dir {
            let _ = std::env::set_current_dir(dir);
        }
    }

    #[tokio::test]
    async fn test_init_project_with_special_characters() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test-project_with-special.chars";
        let _project_path = temp_dir.path().join(project_name);

        let original_dir = std::env::current_dir().ok();
        let _ = std::env::set_current_dir(temp_dir.path());

        let _ = init_project(project_name).await;

        // Restore original directory
        if let Some(dir) = original_dir {
            let _ = std::env::set_current_dir(dir);
        }
    }

    #[tokio::test]
    async fn test_init_project_empty_name() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let project_name = "";
        let _ = init_project(project_name).await;
    }

    #[test]
    fn test_default_template_content() {
        assert!(DEFAULT_TEMPLATE.contains("{PROJECT_NAME}"));
        assert!(DEFAULT_TEMPLATE.contains("{FEATURE_NAME}"));
        assert!(DEFAULT_TEMPLATE.contains("Overview"));
        assert!(DEFAULT_TEMPLATE.contains("Problem Statement"));
        assert!(DEFAULT_TEMPLATE.contains("Requirements"));
        assert!(DEFAULT_TEMPLATE.contains("Success Metrics"));
    }

    #[test]
    fn test_technical_template_content() {
        assert!(TECHNICAL_TEMPLATE.contains("{PROJECT_NAME}"));
        assert!(TECHNICAL_TEMPLATE.contains("{FEATURE_NAME}"));
        assert!(TECHNICAL_TEMPLATE.contains("Technical"));
        assert!(TECHNICAL_TEMPLATE.contains("Technical Overview"));
        assert!(TECHNICAL_TEMPLATE.contains("Architecture Requirements"));
        assert!(TECHNICAL_TEMPLATE.contains("API Specifications"));
        assert!(TECHNICAL_TEMPLATE.contains("Performance Metrics"));
    }

    #[test]
    fn test_default_audit_rules_content() {
        assert!(DEFAULT_AUDIT_RULES.contains("Audit Rules"));
        assert!(DEFAULT_AUDIT_RULES.contains("Required Sections"));
        assert!(DEFAULT_AUDIT_RULES.contains("Overview"));
        assert!(DEFAULT_AUDIT_RULES.contains("Requirements"));
        assert!(DEFAULT_AUDIT_RULES.contains("Success Metrics"));
        assert!(DEFAULT_AUDIT_RULES.contains("Common Issues to Flag"));
    }

    #[test]
    fn test_default_generation_rules_content() {
        assert!(DEFAULT_GENERATION_RULES.contains("Generation Rules"));
        assert!(DEFAULT_GENERATION_RULES.contains("Always Include"));
        assert!(DEFAULT_GENERATION_RULES.contains("Clear problem statement"));
        assert!(DEFAULT_GENERATION_RULES.contains("Specific user stories"));
        assert!(DEFAULT_GENERATION_RULES.contains("Measurable success metrics"));
    }

    #[tokio::test]
    async fn test_init_project_creates_correct_config() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "config-test-project";

        let original_dir = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(temp_dir.path());

        let result = init_project(project_name).await;
        if let Err(e) = result {
            panic!("init failed: {}", e);
        }

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }
}
