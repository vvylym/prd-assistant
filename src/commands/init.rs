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
