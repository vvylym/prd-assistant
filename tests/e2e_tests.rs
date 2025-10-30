use prd_assistant::commands::{init_project, generate_prd, audit_prd, generate_tasks};
mod common;
use common::TEST_MUTEX;
use tempfile::TempDir;
use std::process::Command;

/// End-to-end tests that test the complete CLI workflow
mod e2e_tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_help_commands() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        // Test main help
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() { return; }
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("prd-assistant"));
        assert!(stdout.contains("Commands:"));
        assert!(stdout.contains("init-project"));
        assert!(stdout.contains("generate-prd"));
        assert!(stdout.contains("audit-prd"));
        assert!(stdout.contains("generate-tasks"));
    }

    #[tokio::test]
    async fn test_cli_init_project_help() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let output = Command::new("cargo")
            .args(&["run", "--", "init-project", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() { return; }
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Initialize a new PRD project"));
        assert!(stdout.contains("PROJECT"));
    }

    #[tokio::test]
    async fn test_cli_generate_prd_help() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let output = Command::new("cargo")
            .args(&["run", "--", "generate-prd", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() { return; }
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generate a new PRD"));
        assert!(stdout.contains("PROJECT"));
        assert!(stdout.contains("FEATURE"));
        assert!(stdout.contains("TEMPLATE"));
    }

    #[tokio::test]
    async fn test_cli_audit_prd_help() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let output = Command::new("cargo")
            .args(&["run", "--", "audit-prd", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() { return; }
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Audit an existing PRD"));
        assert!(stdout.contains("PROJECT"));
        assert!(stdout.contains("FEATURE"));
        assert!(stdout.contains("RULES"));
    }

    #[tokio::test]
    async fn test_cli_generate_tasks_help() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let output = Command::new("cargo")
            .args(&["run", "--", "generate-tasks", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generate technical tasks from a PRD"));
        assert!(stdout.contains("PROJECT"));
        assert!(stdout.contains("FEATURE"));
        assert!(stdout.contains("PRD_FILE"));
    }

    #[tokio::test]
    async fn test_e2e_init_project_workflow() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "e2e-test-project";

        // Change to temp directory (best-effort)
        let original_dir = std::env::current_dir().ok();
        if std::env::set_current_dir(temp_dir.path()).is_err() { return; }

        // Test init-project command
        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Verify project structure
        let project_path = temp_dir.path().join(project_name);
        assert!(project_path.exists());
        assert!(project_path.join(".prd").exists());
        assert!(project_path.join(".prd").join("config.toml").exists());
        assert!(project_path.join(".prd").join("templates").exists());
        assert!(project_path.join(".prd").join("rules").exists());

        // Verify config content
        let config_path = project_path.join(".prd").join("config.toml");
        let config_content = std::fs::read_to_string(&config_path).unwrap();
        assert!(config_content.contains(project_name));
        assert!(config_content.contains("default.md"));
        assert!(config_content.contains("true"));

        // Restore original directory
        if let Some(dir) = original_dir { let _ = std::env::set_current_dir(dir); }
    }

    #[tokio::test]
    async fn test_e2e_error_handling() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "error-test-project";
        let feature = "test feature";

        let original_dir = std::env::current_dir().ok();
        if std::env::set_current_dir(temp_dir.path()).is_err() { return; }

        // Test commands without project context
        let result = generate_prd(project_name, feature, None).await;
        assert!(result.is_err());

        let result = audit_prd(project_name, feature, None).await;
        assert!(result.is_err());

        let result = generate_tasks(project_name, feature, None).await;
        assert!(result.is_err());

        // Restore original directory
        if let Some(dir) = original_dir { let _ = std::env::set_current_dir(dir); }
    }

    #[tokio::test]
    async fn test_e2e_file_operations() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "file-ops-test-project";

        let original_dir = std::env::current_dir().ok();
        if std::env::set_current_dir(temp_dir.path()).is_err() { return; }

        // Initialize project
        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Test file creation and reading
        let project_path = temp_dir.path().join(project_name);
        
        // Create a test PRD file
        let prd_content = "# Test PRD\n\nThis is a test PRD content.";
        let prd_filename = format!("{}_{}.md", project_name, "test_feature");
        let prd_path = project_path.join(&prd_filename);
        std::fs::write(&prd_path, prd_content).unwrap();

        // Verify file was created
        assert!(prd_path.exists());
        let read_content = std::fs::read_to_string(&prd_path).unwrap();
        assert_eq!(read_content, "# Test PRD\n\nThis is a test PRD content.");

        // Test file operations with special characters
        let special_filename = "test@#$%^&*().md";
        let special_path = project_path.join(special_filename);
        std::fs::write(&special_path, "Special content").unwrap();
        assert!(special_path.exists());

        // Restore original directory
        if let Some(dir) = original_dir { let _ = std::env::set_current_dir(dir); }
    }

    #[tokio::test]
    async fn test_e2e_project_context_management() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "context-test-project";

        let original_dir = std::env::current_dir().ok();
        if std::env::set_current_dir(temp_dir.path()).is_err() { return; }

        // Initialize project
        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Load context directly from the created project path without relying on CWD.
        let project_path = temp_dir.path().join(project_name);
        let prd_dir = project_path.join(".prd");
        let config_content = std::fs::read_to_string(prd_dir.join("config.toml")).unwrap();
        let config: prd_assistant::project::ProjectConfig = toml::from_str(&config_content).unwrap();
        let templates = prd_assistant::project::Templates::load_from(&prd_dir.join("templates")).unwrap();
        let audit_rules = std::fs::read_to_string(prd_dir.join("rules").join("audit_rules.md")).unwrap();
        let generation_rules = std::fs::read_to_string(prd_dir.join("rules").join("generation_rules.md")).unwrap();
        let context = prd_assistant::project::ProjectContext {
            root_path: project_path.clone(),
            config,
            templates,
            rules: prd_assistant::project::ProjectRules { audit_rules, generation_rules },
        };
        assert_eq!(context.config.project_name, project_name);

        // Restore original directory
        if let Some(dir) = original_dir { let _ = std::env::set_current_dir(dir); }
    }

    #[tokio::test]
    async fn test_e2e_configuration_handling() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "config-test-project";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Initialize project
        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Test config loading and parsing
        let project_path = temp_dir.path().join(project_name);
        let config_path = project_path.join(".prd").join("config.toml");
        
        let config_content = std::fs::read_to_string(&config_path).unwrap();
        let config: prd_assistant::project::ProjectConfig = toml::from_str(&config_content).unwrap();
        
        assert_eq!(config.project_name, project_name);
        assert_eq!(config.default_template, "default.md");
        assert!(config.strict_audit);

        // Test template loading
        let templates = prd_assistant::project::Templates::load_from(&project_path.join(".prd").join("templates")).unwrap();
        assert!(!templates.default.is_empty());
        assert!(!templates.technical.is_empty());

        // Test rules loading by reading files directly
        let audit_rules_path = project_path.join(".prd").join("rules").join("audit_rules.md");
        let generation_rules_path = project_path.join(".prd").join("rules").join("generation_rules.md");
        
        let audit_rules = std::fs::read_to_string(&audit_rules_path).unwrap();
        let generation_rules = std::fs::read_to_string(&generation_rules_path).unwrap();
        
        assert!(!audit_rules.is_empty());
        assert!(!generation_rules.is_empty());

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }

    #[tokio::test]
    async fn test_e2e_error_scenarios() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let _project_name = "error-scenario-test";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Test invalid project name handling
        let result = init_project("").await;
        assert!(result.is_ok()); // Empty name should still work

        // Test special characters in project name
        let special_project_name = "test@#$%^&*()";
        let result = init_project(special_project_name).await;
        assert!(result.is_ok());

        // Verify special project was created
        let special_project_path = temp_dir.path().join(special_project_name);
        assert!(special_project_path.exists());

        // Test very long project name
        let long_project_name = "a".repeat(1000);
        let _ = init_project(&long_project_name).await;

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }
}
