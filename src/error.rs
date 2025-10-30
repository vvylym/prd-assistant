pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Project error: {0}")]
    Project(String),
    #[error("AI error: {0}")]
    AgentError(String),
    #[error("Template error: {0}")]
    Template(String),
    #[error("TOML serialization error: {0}")]
    Toml(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_display() {
        let io_error = Error::Io(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        assert!(io_error.to_string().contains("IO error"));
        assert!(io_error.to_string().contains("File not found"));

        let project_error = Error::Project("Test project error".to_string());
        assert_eq!(project_error.to_string(), "Project error: Test project error");

        let agent_error = Error::AgentError("Test agent error".to_string());
        assert_eq!(agent_error.to_string(), "AI error: Test agent error");

        let template_error = Error::Template("Test template error".to_string());
        assert_eq!(template_error.to_string(), "Template error: Test template error");
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let prd_error: Error = io_error.into();
        
        match prd_error {
            Error::Io(e) => {
                assert_eq!(e.kind(), io::ErrorKind::PermissionDenied);
                assert_eq!(e.to_string(), "Permission denied");
            }
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_error_from_toml_error() {
        // Create a TOML error by trying to deserialize invalid TOML
        let invalid_toml = "invalid = toml content with [unclosed bracket";
        let toml_error = toml::from_str::<toml::Value>(invalid_toml).unwrap_err();
        let prd_error: Error = toml_error.into();
        
        match prd_error {
            Error::TomlDe(e) => {
                assert!(!e.to_string().is_empty());
            }
            _ => panic!("Expected TOML deserialization error"),
        }
    }

    #[test]
    fn test_error_debug() {
        let error = Error::Project("Test error".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Project"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<String> {
            Ok("test".to_string())
        }

        fn returns_error() -> Result<String> {
            Err(Error::Project("test error".to_string()))
        }

        assert!(returns_result().is_ok());
        assert!(returns_error().is_err());
    }
}
