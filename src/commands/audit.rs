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

    // Determine which rules to use
    let audit_rules = match rules {
        Some("generation") => &project.rules.generation_rules,
        Some("audit") | None => &project.rules.audit_rules,
        Some(rules_name) => {
            return Err(crate::error::Error::Template(format!(
                "Unknown rules: {}. Available rules: audit, generation",
                rules_name
            )));
        }
    };

    // Create a safe filename from the feature name
    let safe_feature = feature
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_");
    let filename = format!(
        "{}_{}.md", project.config.project_name, safe_feature
    );
    let file_path = project.root_path.join(&filename);

    if !file_path.exists() {
        return Err(crate::error::Error::Project(format!(
            "PRD file not found: {}. Please generate a PRD first using the generate-prd command.",
            file_path.display()
        )));
    }

    let prd_content = std::fs::read_to_string(&file_path)?;

    let audit_report = agent
        .audit_prd_content_with_rules(prd_content, audit_rules)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_audit_report() {
        let report = "Test audit report content";
        
        // Capture stdout to test the display function
        // Note: This is a simple test - in a real scenario you might want to use
        // a more sophisticated approach to capture and verify stdout
        display_audit_report(report);
        
        // The function should not panic
        assert!(true);
    }

    #[test]
    fn test_display_audit_report_empty() {
        let report = "";
        display_audit_report(report);
        assert!(true);
    }

    #[test]
    fn test_display_audit_report_long_content() {
        let report = "A".repeat(1000);
        display_audit_report(&report);
        assert!(true);
    }

    #[test]
    fn test_display_audit_report_special_characters() {
        let report = "Report with special chars: @#$%^&*()[]{}|\\:;\"'<>?,./";
        display_audit_report(report);
        assert!(true);
    }

    #[test]
    fn test_display_audit_report_unicode() {
        let report = "Report with unicode: 测试报告 🚀 ✅ ❌";
        display_audit_report(report);
        assert!(true);
    }

    #[test]
    fn test_display_audit_report_multiline() {
        let report = "Line 1\nLine 2\nLine 3\n\nEmpty line above";
        display_audit_report(report);
        assert!(true);
    }
}
