# Contributing to PRD Assistant

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style and Standards](#code-style-and-standards)
- [Project Structure](#project-structure)
- [Adding New Features](#adding-new-features)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)
- [Community Guidelines](#community-guidelines)

## Getting Started

Thank you for your interest in contributing to PRD Assistant! This guide will help you get started with development and understand our contribution process.

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Ollama with Gemma3n model (for testing AI features)
- Basic understanding of Rust and CLI development

### Quick Start

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/prd-assistant.git
   cd prd-assistant
   ```

3. Set up the development environment:
   ```bash
   # Install dependencies
   cargo build
   
   # Set up pre-commit hooks (optional)
   cargo install cargo-husky
   cargo husky install
   ```

## Development Setup

### Environment Configuration

1. **Rust Toolchain:**
   ```bash
   rustup update stable
   rustup component add rustfmt clippy
   ```

2. **Ollama Setup:**
   ```bash
   # Install Ollama
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Start Ollama service
   ollama serve
   
   # Pull required model (in another terminal)
   ollama pull gemma3n:latest
   ```

3. **Development Dependencies:**
   ```bash
   # Install useful development tools
   cargo install cargo-watch
   cargo install cargo-expand
   cargo install cargo-audit
   ```

### IDE Configuration

#### VS Code

Recommended extensions:
- `rust-analyzer` - Rust language server
- `CodeLLDB` - Debugging support
- `crates` - Cargo.toml support
- `Better TOML` - TOML file support

#### Configuration Files

Create `.vscode/settings.json`:
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "editor.rulers": [100]
}
```

### Running the Development Version

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- init-project test-project

# Run specific command
cargo run -- generate-prd test-project "Test feature"

# Run with watch mode for development
cargo watch -x "run -- generate-prd test-project 'Test feature'"
```

## Code Style and Standards

### Rust Conventions

We follow standard Rust conventions and the official style guide:

1. **Formatting:**
   ```bash
   cargo fmt
   ```

2. **Linting:**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Documentation:**
   - All public APIs must be documented
   - Use `///` for doc comments
   - Include examples in documentation
   - Document error conditions

### Code Organization

1. **Module Structure:**
   - Keep modules focused and cohesive
   - Use `mod.rs` for module declarations
   - Group related functionality together

2. **Error Handling:**
   - Use `thiserror` for error types
   - Provide meaningful error messages
   - Include context in error chains

3. **Async/Await:**
   - Use `async`/`await` consistently
   - Handle errors properly in async functions
   - Use `tokio::main` for main functions

### Naming Conventions

- **Functions:** `snake_case`
- **Variables:** `snake_case`
- **Types:** `PascalCase`
- **Constants:** `SCREAMING_SNAKE_CASE`
- **Modules:** `snake_case`

### Documentation Standards

```rust
/// Generates a new PRD for the specified project and feature.
///
/// # Arguments
///
/// * `project_name` - The name of the project
/// * `feature` - Description of the feature to generate PRD for
/// * `template` - Optional template name (default, technical)
///
/// # Returns
///
/// * `Result<()>` - Success or error result
///
/// # Errors
///
/// This function will return an error if:
/// - The project context cannot be found
/// - The AI service is unavailable
/// - File I/O operations fail
///
/// # Examples
///
/// ```rust
/// use prd_assistant::commands::generate_prd;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     generate_prd("my-project", "User authentication", None).await?;
///     Ok(())
/// }
/// ```
pub async fn generate_prd(
    project_name: &str,
    feature: &str,
    template: Option<&str>,
) -> Result<()> {
    // Implementation...
}
```

## Project Structure

```
prd-assistant/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── agent/               # AI agent module
│   │   └── mod.rs
│   ├── commands/            # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── generate.rs
│   │   └── audit.rs
│   ├── project.rs           # Project management
│   └── error.rs             # Error types
├── docs/                    # Documentation
├── examples/                # Example projects
├── tests/                   # Integration tests
├── Cargo.toml
└── README.md
```

### Key Modules

1. **`src/main.rs`**: CLI interface using Clap
2. **`src/commands/`**: Business logic for each command
3. **`src/agent/`**: AI integration and content generation
4. **`src/project.rs`**: Project context and configuration management
5. **`src/error.rs`**: Centralized error handling

## Adding New Features

### Feature Development Process

1. **Create an Issue:**
   - Describe the feature or bug
   - Provide context and use cases
   - Label appropriately (enhancement, bug, etc.)

2. **Create a Branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Implement the Feature:**
   - Follow the code style guidelines
   - Add comprehensive tests
   - Update documentation

4. **Test Thoroughly:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Submit a Pull Request:**
   - Reference the related issue
   - Provide a clear description
   - Include test results

### Example: Adding a New Command

Let's say we want to add a `validate-prd` command:

1. **Add Command to CLI:**
   ```rust
   // In src/main.rs
   #[derive(Subcommand)]
   enum Commands {
       // ... existing commands
       /// Validate an existing PRD
       ValidatePRD {
           /// Project name
           project: String,
           /// Feature name
           feature: String,
       },
   }
   ```

2. **Implement Command Logic:**
   ```rust
   // In src/commands/validate.rs
   use crate::agent::PrdAgent;
   use crate::error::Result;
   use crate::project::ProjectContext;

   pub async fn validate_prd(project_name: &str, feature: &str) -> Result<()> {
       let project = ProjectContext::ensure_project_context()?;
       let agent = PrdAgent::new(project.config.project_name.clone())?;
       
       // Load PRD content
       let prd_content = load_prd_content(&project, feature)?;
       
       // Validate using AI
       let validation_report = agent.validate_prd_content(prd_content).await?;
       
       // Display results
       display_validation_report(&validation_report);
       
       Ok(())
   }
   ```

3. **Export Command:**
   ```rust
   // In src/commands/mod.rs
   pub mod validate;
   pub use validate::validate_prd;
   ```

4. **Add to Main:**
   ```rust
   // In src/main.rs
   match cli.command {
       // ... existing commands
       Commands::ValidatePRD { project, feature } => {
           validate_prd(&project, &feature).await
       }
   }
   ```

### Adding New AI Features

1. **Extend PrdAgent:**
   ```rust
   // In src/agent/mod.rs
   impl PrdAgent {
       pub async fn analyze_prd_complexity(&self, prd_content: String) -> Result<String> {
           let prompt = format!(
               "Analyze the complexity of this PRD and provide recommendations: {}",
               prd_content
           );
           
           // AI processing...
       }
   }
   ```

2. **Add Error Handling:**
   ```rust
   // In src/error.rs
   #[derive(thiserror::Error, Debug)]
   pub enum Error {
       // ... existing errors
       #[error("Analysis error: {0}")]
       AnalysisError(String),
   }
   ```

## Testing

### Test Structure

```
tests/
├── integration/              # Integration tests
│   ├── test_init.rs
│   ├── test_generate.rs
│   └── test_audit.rs
├── fixtures/                 # Test data
│   ├── sample_config.toml
│   └── sample_prd.md
└── helpers/                  # Test utilities
    └── mod.rs
```

### Writing Tests

#### Unit Tests

```rust
// In src/commands/init.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_init_project_creates_structure() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");
        
        // Test implementation...
    }
}
```

#### Integration Tests

```rust
// In tests/integration/test_init.rs
use prd_assistant::commands::init_project;
use tempfile::TempDir;

#[tokio::test]
async fn test_init_project_integration() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");
    
    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Test project initialization
    init_project("test-project").await.unwrap();
    
    // Verify project structure
    assert!(project_path.join(".prd").exists());
    assert!(project_path.join(".prd/config.toml").exists());
}
```

#### Mock AI Responses

```rust
// In tests/helpers/mock_agent.rs
use prd_assistant::agent::PrdAgent;

pub struct MockPrdAgent {
    responses: Vec<String>,
}

impl MockPrdAgent {
    pub fn new() -> Self {
        Self {
            responses: vec!["Mock PRD content".to_string()],
        }
    }
    
    pub async fn generate_prd_content(&self, _input: String) -> Result<String> {
        Ok(self.responses[0].clone())
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_init_project

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Data Management

Use fixtures for consistent test data:

```rust
// In tests/fixtures/sample_config.toml
project_name = "test-project"
default_template = "default.md"
strict_audit = true
```

```rust
// In tests/helpers/mod.rs
pub fn load_fixture(name: &str) -> String {
    let path = format!("tests/fixtures/{}", name);
    std::fs::read_to_string(path).unwrap()
}
```

## Documentation

### Documentation Standards

1. **API Documentation:**
   - Document all public functions, types, and modules
   - Include examples in doc comments
   - Document error conditions

2. **User Documentation:**
   - Keep usage guides up to date
   - Include practical examples
   - Document configuration options

3. **Architecture Documentation:**
   - Document design decisions
   - Explain complex algorithms
   - Maintain architecture diagrams

### Building Documentation

```bash
# Build library documentation
cargo doc --open

# Build with private items
cargo doc --document-private-items

# Build for specific target
cargo doc --target x86_64-unknown-linux-gnu
```

### Documentation Review Checklist

- [ ] All public APIs documented
- [ ] Examples compile and run
- [ ] Error conditions documented
- [ ] Configuration options explained
- [ ] Usage examples provided
- [ ] Architecture decisions explained

## Pull Request Process

### Before Submitting

1. **Code Quality:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo doc
   ```

2. **Commit Messages:**
   - Use conventional commits format
   - Be descriptive and concise
   - Reference issues when applicable

   Examples:
   ```
   feat: add validate-prd command
   fix: handle missing template files gracefully
   docs: update API reference for new features
   test: add integration tests for audit command
   ```

3. **Branch Management:**
   ```bash
   # Keep branch up to date
   git fetch origin
   git rebase origin/main
   
   # Squash commits if needed
   git rebase -i HEAD~3
   ```

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests pass locally
- [ ] No breaking changes (or documented)

## Related Issues
Closes #123
```

### Review Process

1. **Automated Checks:**
   - CI/CD pipeline runs tests
   - Code quality checks
   - Security scanning

2. **Manual Review:**
   - Code quality and style
   - Architecture alignment
   - Test coverage
   - Documentation completeness

3. **Approval Process:**
   - At least one maintainer approval required
   - All CI checks must pass
   - No merge conflicts

## Release Process

### Version Management

We use semantic versioning (SemVer):
- `MAJOR`: Breaking changes
- `MINOR`: New features (backward compatible)
- `PATCH`: Bug fixes (backward compatible)

### Release Checklist

1. **Pre-Release:**
   - [ ] Update version in `Cargo.toml`
   - [ ] Update `CHANGELOG.md`
   - [ ] Run full test suite
   - [ ] Update documentation

2. **Release:**
   - [ ] Create release tag
   - [ ] Build release artifacts
   - [ ] Publish to crates.io
   - [ ] Update GitHub releases

3. **Post-Release:**
   - [ ] Verify installation works
   - [ ] Monitor for issues
   - [ ] Update documentation if needed

### Release Commands

```bash
# Update version
cargo set-version 0.2.0

# Create release commit
git commit -m "chore: release v0.2.0"

# Create and push tag
git tag v0.2.0
git push origin v0.2.0

# Publish to crates.io
cargo publish
```

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please:

1. **Be Respectful:** Treat everyone with respect and kindness
2. **Be Constructive:** Provide helpful feedback and suggestions
3. **Be Patient:** Remember that everyone is learning and growing
4. **Be Inclusive:** Welcome contributors from all backgrounds

### Getting Help

1. **Documentation:** Check existing docs first
2. **Issues:** Search existing issues before creating new ones
3. **Discussions:** Use GitHub Discussions for questions
4. **Chat:** Join our community chat (if available)

### Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project documentation

### Reporting Issues

When reporting issues, please include:

1. **Environment:**
   - OS and version
   - Rust version
   - PRD Assistant version

2. **Reproduction:**
   - Steps to reproduce
   - Expected behavior
   - Actual behavior

3. **Additional Context:**
   - Error messages
   - Logs (with sensitive data removed)
   - Screenshots if applicable

### Feature Requests

When requesting features, please include:

1. **Use Case:** Why is this feature needed?
2. **Proposed Solution:** How should it work?
3. **Alternatives:** What alternatives have you considered?
4. **Additional Context:** Any other relevant information

## Development Tools

### Useful Cargo Commands

```bash
# Development
cargo watch -x run
cargo expand --lib
cargo audit
cargo outdated

# Testing
cargo test -- --nocapture
cargo test --test integration
cargo tarpaulin

# Documentation
cargo doc --open
cargo doc --document-private-items

# Code Quality
cargo fmt
cargo clippy -- -D warnings
cargo miri test  # If using Miri
```

### Git Hooks

Set up pre-commit hooks:

```bash
# Install cargo-husky
cargo install cargo-husky

# Set up hooks
cargo husky install
```

Pre-commit hook example:
```bash
#!/bin/sh
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Conclusion

Thank you for contributing to PRD Assistant! Your contributions help make this tool better for everyone. If you have any questions or need help getting started, don't hesitate to reach out through GitHub Issues or Discussions.

Happy coding! 🦀
