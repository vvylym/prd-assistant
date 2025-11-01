use crate::agent::PrdAgent;
use crate::error::Result;
use crate::project::ProjectContext;
use std::path::PathBuf;
use tracing::info;

pub async fn generate_tasks(
    project_name: &str,
    feature: &str,
    prd_file: Option<&str>,
) -> Result<()> {
    info!(
        "Generating technical tasks for project: {} and feature: {}",
        project_name, feature
    );

    let project = ProjectContext::ensure_project_context()?;
    let agent = PrdAgent::new(project.config.project_name.clone())?;

    // Determine the PRD file to use
    let prd_content = if let Some(prd_file) = prd_file {
        // Use specified PRD file
        let prd_path = project.root_path.join(prd_file);
        if !prd_path.exists() {
            return Err(crate::error::Error::Project(format!(
                "PRD file not found: {}. Please provide a valid PRD file path.",
                prd_path.display()
            )));
        }
        std::fs::read_to_string(&prd_path)?
    } else {
        // Try to find PRD file based on feature name
        let safe_feature = feature
            .replace(" ", "_")
            .replace("/", "_")
            .replace("\\", "_");
        let filename =
            format!("{}_{}.md", project.config.project_name, safe_feature).to_lowercase();
        let prd_path = project.root_path.join(&filename);

        if !prd_path.exists() {
            return Err(crate::error::Error::Project(format!(
                "PRD file not found: {}. Please generate a PRD first using the generate-prd command, or specify a PRD file with --prd-file option.",
                prd_path.display()
            )));
        }
        std::fs::read_to_string(&prd_path)?
    };

    // Generate technical tasks based on PRD content
    let tasks_content = agent.generate_technical_tasks(prd_content).await?;

    // Save the generated tasks
    let output_path = save_generated_tasks(&project, &tasks_content, feature)?;

    println!("✅ Technical tasks generated successfully!");
    println!("📄 Saved to: {}", output_path.display());

    Ok(())
}

fn save_generated_tasks(project: &ProjectContext, content: &str, feature: &str) -> Result<PathBuf> {
    // Create a safe filename from the feature name
    let safe_feature = feature
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_");

    // Truncate very long feature names to avoid filesystem limits
    // Most filesystems have a 255 character limit for filenames
    // We'll keep it under 200 to be safe, accounting for project name and extension
    let max_feature_length = 200 - project.config.project_name.len() - 12; // 12 for "tasks___.md"
    let truncated_feature = if safe_feature.len() > max_feature_length {
        &safe_feature[..max_feature_length]
    } else {
        &safe_feature
    };

    let filename = format!(
        "tasks_{}_{}.md",
        project.config.project_name, truncated_feature
    );
    let output_path = project.root_path.join(&filename);

    std::fs::write(&output_path, content)?;
    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{ProjectConfig, ProjectContext};
    use tempfile::TempDir;

    #[test]
    fn test_save_generated_tasks_success() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "test feature";

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        assert!(output_path.exists());

        let saved_content = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(saved_content, content);

        // Verify filename format
        let filename = output_path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("tasks_test-project_"));
        assert!(filename.ends_with(".md"));
        assert!(filename.contains("test_feature"));
    }

    #[test]
    fn test_save_generated_tasks_filename_sanitization() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "test/feature\\with spaces";

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        let filename = output_path.file_name().unwrap().to_str().unwrap();

        // Verify special characters are replaced
        assert!(!filename.contains("/"));
        assert!(!filename.contains("\\"));
        assert!(!filename.contains(" "));
        assert!(filename.contains("test_feature_with_spaces"));
    }

    #[test]
    fn test_save_generated_tasks_empty_feature() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "";

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        let filename = output_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "tasks_test-project_.md");
    }

    #[test]
    fn test_save_generated_tasks_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "feature@#$%^&*()";

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        let filename = output_path.file_name().unwrap().to_str().unwrap();
        assert!(filename.contains("feature"));
    }

    #[test]
    fn test_save_generated_tasks_long_feature_name() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "a".repeat(1000); // Very long feature name

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        assert!(output_path.exists());

        let saved_content = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(saved_content, content);

        // Verify that the filename was truncated
        let filename = output_path.file_name().unwrap().to_str().unwrap();
        assert!(filename.len() < 255); // Should be within filesystem limits
        assert!(filename.starts_with("tasks_test-project_"));
        assert!(filename.ends_with(".md"));
    }

    #[test]
    fn test_save_generated_tasks_unicode_feature() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        let content = "Test tasks content";
        let feature = "测试功能"; // Chinese characters

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        assert!(output_path.exists());

        let saved_content = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(saved_content, content);
    }

    #[test]
    fn test_save_generated_tasks_write_permission_error() {
        let temp_dir = TempDir::new().unwrap();
        let project_context = make_project_context(&temp_dir);

        // Create a read-only directory
        let read_only_dir = temp_dir.path().join("readonly");
        std::fs::create_dir(&read_only_dir).unwrap();

        // On Unix systems, we can make the directory read-only
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&read_only_dir).unwrap().permissions();
            perms.set_mode(0o444); // Read-only
            std::fs::set_permissions(&read_only_dir, perms).unwrap();
        }

        // Create a project context with read-only root
        let mut project_context = project_context.clone();
        project_context.root_path = read_only_dir;

        let content = "Test tasks content";
        let feature = "test feature";

        let result = save_generated_tasks(&project_context, content, &feature);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::error::Error::Io(_)));
    }
    fn make_project_context(temp_dir: &TempDir) -> ProjectContext {
        let root = temp_dir.path().to_path_buf();
        ProjectContext {
            root_path: root,
            config: ProjectConfig {
                project_name: "test-project".to_string(),
                default_template: "default.md".to_string(),
                strict_audit: true,
            },
            templates: crate::project::Templates {
                default: String::new(),
                technical: String::new(),
            },
            rules: crate::project::ProjectRules {
                audit_rules: String::new(),
                generation_rules: String::new(),
            },
        }
    }
}
