use prd_assistant::commands::{init_project, generate_prd, audit_prd, generate_tasks};
mod common;
use common::{create_test_project_context, create_mock_prd_content, TEST_MUTEX};
use tempfile::TempDir;

/// Integration tests for the complete PRD Assistant workflow
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_workflow() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "integration-test-project";
        let feature = "User authentication system";

        // Change to temp directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Step 1: Initialize project
        let result = init_project(project_name).await;
        assert!(result.is_ok(), "Project initialization should succeed");

        // Verify project structure
        let project_path = temp_dir.path().join(project_name);
        assert!(project_path.exists());
        assert!(project_path.join(".prd").exists());
        assert!(project_path.join(".prd").join("config.toml").exists());
        assert!(project_path.join(".prd").join("templates").exists());
        assert!(project_path.join(".prd").join("rules").exists());

        // Step 2: Generate PRD (this will fail without AI, but we can test the file structure)
        // Note: In a real integration test, you'd mock the AI agent
        let result = generate_prd(project_name, feature, None).await;
        // This will fail without AI, but we can test the error handling
        assert!(result.is_err());

        // Step 3: Create a mock PRD file for testing audit and tasks
        let prd_content = create_mock_prd_content();
        let prd_filename = format!("{}_{}.md", project_name, feature.replace(" ", "_"));
        let prd_path = project_path.join(&prd_filename);
        std::fs::write(&prd_path, &prd_content).unwrap();

        // Step 4: Test audit (this will fail without AI, but we can test the file loading)
        let result = audit_prd(project_name, feature, None).await;
        // This will fail without AI, but we can test the error handling
        assert!(result.is_err());

        // Step 5: Test task generation (this will fail without AI, but we can test the file loading)
        let result = generate_tasks(project_name, feature, None).await;
        // This will fail without AI, but we can test the error handling
        assert!(result.is_err());

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }

    #[tokio::test]
    async fn test_init_project_integration() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "init-test-project";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Verify all files are created correctly
        let project_path = temp_dir.path().join(project_name);
        
        // Check config file
        let config_path = project_path.join(".prd").join("config.toml");
        assert!(config_path.exists());
        let config_content = std::fs::read_to_string(&config_path).unwrap();
        assert!(config_content.contains(project_name));
        assert!(config_content.contains("default.md"));
        assert!(config_content.contains("true"));

        // Check template files
        let default_template = project_path.join(".prd").join("templates").join("default.md");
        let technical_template = project_path.join(".prd").join("templates").join("technical.md");
        assert!(default_template.exists());
        assert!(technical_template.exists());

        // Check rule files
        let audit_rules = project_path.join(".prd").join("rules").join("audit_rules.md");
        let generation_rules = project_path.join(".prd").join("rules").join("generation_rules.md");
        assert!(audit_rules.exists());
        assert!(generation_rules.exists());

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }

    #[tokio::test]
    async fn test_project_context_discovery() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "context-test-project";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Initialize project
        let result = init_project(project_name).await;
        assert!(result.is_ok());

        // Change to project directory
        let project_path = temp_dir.path().join(project_name);
        let _ = std::env::set_current_dir(&project_path);

        // Test project context discovery
        let project_context = prd_assistant::project::ProjectContext::find_current().unwrap();
        assert!(project_context.is_some());
        
        let context = project_context.unwrap();
        assert_eq!(context.config.project_name, project_name);

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }

    #[tokio::test]
    async fn test_file_operations_integration() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let _project_context = create_test_project_context(&temp_dir).unwrap();

        // Test PRD file creation
        let prd_content = create_mock_prd_content();
        let prd_path = temp_dir.path().join("test_prd.md");
        std::fs::write(&prd_path, &prd_content).unwrap();

        // Verify file was created
        assert!(prd_path.exists());
        let read_content = std::fs::read_to_string(&prd_path).unwrap();
        assert_eq!(read_content, prd_content);

        // Test task file creation
        let task_content = "Test task content";
        let task_path = temp_dir.path().join("test_tasks.md");
        std::fs::write(&task_path, task_content).unwrap();

        // Verify file was created
        assert!(task_path.exists());
        let read_task_content = std::fs::read_to_string(&task_path).unwrap();
        assert_eq!(read_task_content, task_content);
    }

    #[tokio::test]
    async fn test_error_handling_integration() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project_name = "error-test-project";
        let feature = "test feature";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Test generate_prd without project context
        let result = generate_prd(project_name, feature, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), prd_assistant::error::Error::Project(_)));

        // Test audit_prd without project context
        let result = audit_prd(project_name, feature, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), prd_assistant::error::Error::Project(_)));

        // Test generate_tasks without project context
        let result = generate_tasks(project_name, feature, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), prd_assistant::error::Error::Project(_)));

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }

    #[tokio::test]
    async fn test_multiple_projects_integration() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = TempDir::new().unwrap();
        let project1_name = "project1";
        let project2_name = "project2";

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Initialize first project
        let result1 = init_project(project1_name).await;
        assert!(result1.is_ok());

        // Initialize second project
        let result2 = init_project(project2_name).await;
        assert!(result2.is_ok());

        // Verify both projects exist
        assert!(temp_dir.path().join(project1_name).exists());
        assert!(temp_dir.path().join(project2_name).exists());

        // Verify they have separate configurations
        let config1_path = temp_dir.path().join(project1_name).join(".prd").join("config.toml");
        let config2_path = temp_dir.path().join(project2_name).join(".prd").join("config.toml");

        let config1_content = std::fs::read_to_string(&config1_path).unwrap();
        let config2_content = std::fs::read_to_string(&config2_path).unwrap();

        assert!(config1_content.contains(project1_name));
        assert!(config2_content.contains(project2_name));
        assert!(!config1_content.contains(project2_name));
        assert!(!config2_content.contains(project1_name));

        // Restore original directory
        let _ = std::env::set_current_dir(original_dir);
    }
}
