use std::path::PathBuf;
use tempfile::TempDir;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub mod fixtures;

pub fn create_test_project_context(
    temp_dir: &TempDir,
) -> std::result::Result<prd_assistant::project::ProjectContext, prd_assistant::error::Error> {
    let project_path = temp_dir.path().join("test-project");
    let prd_dir = project_path.join(".prd");
    let templates_dir = prd_dir.join("templates");
    let rules_dir = prd_dir.join("rules");

    std::fs::create_dir_all(&templates_dir)?;
    std::fs::create_dir_all(&rules_dir)?;

    let config = prd_assistant::project::ProjectConfig {
        project_name: "test-project".to_string(),
        default_template: "default.md".to_string(),
        strict_audit: true,
    };

    let config_content = toml::to_string_pretty(&config)?;
    std::fs::write(prd_dir.join("config.toml"), config_content)?;

    std::fs::write(
        templates_dir.join("default.md"),
        fixtures::DEFAULT_TEMPLATE,
    )?;
    std::fs::write(
        templates_dir.join("technical.md"),
        fixtures::TECHNICAL_TEMPLATE,
    )?;

    std::fs::write(
        rules_dir.join("audit_rules.md"),
        fixtures::DEFAULT_AUDIT_RULES,
    )?;
    std::fs::write(
        rules_dir.join("generation_rules.md"),
        fixtures::DEFAULT_GENERATION_RULES,
    )?;

    let original_dir = std::env::current_dir().unwrap();
    let project_root = prd_dir.parent().unwrap();

    if !project_root.exists() {
        return Err(prd_assistant::error::Error::Project(
            "Project root does not exist".to_string(),
        ));
    }

    let _ = std::env::set_current_dir(project_root);
    let result = prd_assistant::project::ProjectContext::find_current().unwrap();
    let _ = std::env::set_current_dir(original_dir);

    result.ok_or_else(|| {
        prd_assistant::error::Error::Project(
            "Failed to load project context".to_string(),
        )
    })
}

pub fn create_test_prd_file(
    temp_dir: &TempDir,
    filename: &str,
    content: &str,
) -> std::result::Result<PathBuf, prd_assistant::error::Error> {
    let prd_path = temp_dir.path().join(filename);
    std::fs::write(&prd_path, content)?;
    Ok(prd_path)
}

pub fn assert_file_content(file_path: &PathBuf, expected_content: &str) {
    assert!(file_path.exists(), "File should exist: {:?}", file_path);
    let content = std::fs::read_to_string(file_path).expect("Should read file");
    assert!(
        content.contains(expected_content),
        "File should contain expected content"
    );
}

pub fn assert_dir_exists(dir_path: &PathBuf) {
    assert!(dir_path.exists(), "Directory should exist: {:?}", dir_path);
    assert!(dir_path.is_dir(), "Path should be a directory: {:?}", dir_path);
}

pub fn create_mock_prd_content() -> String {
    fixtures::SAMPLE_PRD_CONTENT.to_string()
}


