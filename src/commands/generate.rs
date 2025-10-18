use crate::agent::PrdAgent;
use crate::error::Result;
use crate::project::ProjectContext;
use std::path::PathBuf;
use tracing::info;

pub async fn generate_prd(
    project_name: &str,
    feature: &str,
    _template: Option<&str>,
) -> Result<()> {
    info!(
        "Handling PRD generation for project: {} and feature: {}",
        project_name, feature
    );

    let project = ProjectContext::ensure_project_context()?;
    let agent = PrdAgent::new(project.config.project_name.clone())?;

    // Use the provided feature description as user input
    let user_input = format!("Generate a comprehensive PRD for the feature: {}", feature);

    let generated_content = agent.generate_prd_content(user_input).await?;

    let output_path = save_generated_prd(&project, &generated_content, feature)?;

    println!("✅ PRD generated successfully!");
    println!("📄 Saved to: {}", output_path.display());

    Ok(())
}

fn save_generated_prd(project: &ProjectContext, content: &str, feature: &str) -> Result<PathBuf> {
    // Create a safe filename from the feature name
    let safe_feature = feature
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_");
    let filename = format!("{}_{}.md", project.config.project_name, safe_feature);
    let output_path = project.root_path.join(&filename);

    std::fs::write(&output_path, content)?;
    Ok(output_path)
}
