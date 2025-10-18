use crate::agent::PrdAgent;
use crate::error::Result;
use crate::project::ProjectContext;
use colored::*;
use tracing::info;

pub async fn audit_prd(project_name: &str, feature: &str, rules: Option<&str>) -> Result<()> {
    info!(
        "Auditing PRD for project: {} and feature: {}",
        project_name, feature
    );

    let project = ProjectContext::ensure_project_context()?;
    let agent = PrdAgent::new(project.config.project_name.clone())?;

    // Create a safe filename from the feature name
    let safe_feature = feature
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_");
    let filename = format!("{}_{}.md", project.config.project_name, safe_feature);
    let file_path = project.root_path.join(&filename);

    if !file_path.exists() {
        return Err(crate::error::Error::Project(format!(
            "PRD file not found: {}. Please generate a PRD first using the generate-prd command.",
            file_path.display()
        )));
    }

    let prd_content = std::fs::read_to_string(&file_path)?;

    // Use custom rules if provided, otherwise use default project rules
    let audit_rules = if let Some(custom_rules) = rules {
        custom_rules.to_string()
    } else {
        project.rules.audit_rules.clone()
    };

    let audit_report = agent
        .audit_prd_content_with_rules(prd_content, &audit_rules)
        .await?;

    display_audit_report(&audit_report);

    Ok(())
}

fn display_audit_report(report: &str) {
    println!("\n{}", "📋 PRD AUDIT REPORT".bold().green());
    println!("{}", "=".repeat(50).green());
    println!("{}", report);
    println!("{}", "=".repeat(50).green());
}
