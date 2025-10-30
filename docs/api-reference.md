# PRD Assistant - API Reference

## Overview

This document provides comprehensive API documentation for the PRD Assistant crate. The API is organized into several modules, each serving a specific purpose in the PRD management workflow.

## Table of Contents

- [Core Modules](#core-modules)
- [Command API](#command-api)
- [Agent API](#agent-api)
- [Project Management API](#project-management-api)
- [Error Types](#error-types)
- [Configuration Schema](#configuration-schema)

## Core Modules

### `prd_assistant::lib`

The main library module that re-exports all public APIs.

```rust
pub mod agent;
pub mod commands;
pub mod error;
pub mod project;
```

## Command API

### `prd_assistant::commands`

The command module provides high-level functions for PRD operations.

#### `init_project(project_name: &str) -> Result<()>`

Initializes a new PRD project with the specified name.

**Parameters:**
- `project_name`: The name of the project to initialize

**Returns:**
- `Result<()>`: Success or error result

**Example:**
```rust
use prd_assistant::commands::init_project;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_project("my-awesome-project").await?;
    println!("Project initialized successfully!");
    Ok(())
}
```

**Side Effects:**
- Creates project directory structure
- Generates default configuration files
- Creates template files
- Creates rule files

#### `generate_prd(project_name: &str, feature: &str, template: Option<&str>) -> Result<()>`

Generates a new PRD for the specified project and feature.

**Parameters:**
- `project_name`: The name of the project
- `feature`: Description of the feature to generate PRD for
- `template`: Optional template name (default, technical)

**Returns:**
- `Result<()>`: Success or error result

**Example:**
```rust
use prd_assistant::commands::generate_prd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_prd(
        "my-project", 
        "User authentication system", 
        Some("technical")
    ).await?;
    println!("PRD generated successfully!");
    Ok(())
}
```

**Side Effects:**
- Creates a new PRD file in the project directory
- Uses AI to generate content based on the feature description

#### `audit_prd(project_name: &str, feature: &str, rules: Option<&str>) -> Result<()>`

Audits an existing PRD and provides detailed feedback.

**Parameters:**
- `project_name`: The name of the project
- `feature`: The feature name (must match generated PRD filename)
- `rules`: Optional custom audit rules

**Returns:**
- `Result<()>`: Success or error result

**Example:**
```rust
use prd_assistant::commands::audit_prd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    audit_prd(
        "my-project", 
        "User authentication system", 
        Some("Focus on security requirements")
    ).await?;
    Ok(())
}
```

**Side Effects:**
- Reads existing PRD file
- Generates audit report using AI
- Displays formatted audit results to console

#### `generate_tasks(project_name: &str, feature: &str, prd_file: Option<&str>) -> Result<()>`

Generates technical tasks from a PRD for implementation planning.

**Parameters:**
- `project_name`: The name of the project
- `feature`: The feature name (used to find PRD if prd_file not provided)
- `prd_file`: Optional path to specific PRD file

**Returns:**
- `Result<()>`: Success or error result

**Example:**
```rust
use prd_assistant::commands::generate_tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate tasks from auto-detected PRD
    generate_tasks("my-project", "User authentication system", None).await?;
    
    // Generate tasks from specific PRD file
    generate_tasks("my-project", "feature", Some("custom_prd.md")).await?;
    Ok(())
}
```

**Side Effects:**
- Reads PRD content (auto-detected or specified file)
- Generates detailed technical task list using AI
- Saves task list as markdown file

## Agent API

### `prd_assistant::agent::PrdAgent`

The AI agent responsible for PRD generation and auditing.

#### `PrdAgent::new(project_name: String) -> Result<Self>`

Creates a new PRD agent instance.

**Parameters:**
- `project_name`: The name of the project

**Returns:**
- `Result<Self>`: New agent instance or error

**Example:**
```rust
use prd_assistant::agent::PrdAgent;

let agent = PrdAgent::new("my-project".to_string())?;
```

#### `generate_prd_content(&self, user_input: String) -> Result<String>`

Generates PRD content using AI based on user input.

**Parameters:**
- `user_input`: Description of the feature or requirements

**Returns:**
- `Result<String>`: Generated PRD content or error

**Example:**
```rust
let content = agent.generate_prd_content(
    "User authentication system with login and registration".to_string()
).await?;
```

**AI Model Configuration:**
- Model: `gemma3n:latest`
- Temperature: `0.7`
- Max Tokens: `None` (unlimited)

#### `audit_prd_content(&self, prd_content: String) -> Result<String>`

Audits PRD content using default rules.

**Parameters:**
- `prd_content`: The PRD content to audit

**Returns:**
- `Result<String>`: Audit report or error

**Example:**
```rust
let audit_report = agent.audit_prd_content(prd_content).await?;
```

#### `audit_prd_content_with_rules(&self, prd_content: String, audit_rules: &str) -> Result<String>`

Audits PRD content using custom rules.

**Parameters:**
- `prd_content`: The PRD content to audit
- `audit_rules`: Custom audit rules

**Returns:**
- `Result<String>`: Audit report or error

**Example:**
```rust
let custom_rules = "Focus on security and performance requirements";
let audit_report = agent.audit_prd_content_with_rules(
    prd_content, 
    custom_rules
).await?;
```

#### `validate_prd_content(&self, prd_content: String) -> Result<String>`

Validates PRD content against quality criteria.

**Parameters:**
- `prd_content`: The PRD content to validate

**Returns:**
- `Result<String>`: Validation report or error

**Example:**
```rust
let validation_report = agent.validate_prd_content(prd_content).await?;
```

#### `generate_technical_tasks(&self, prd_content: String) -> Result<String>`

Generates detailed technical tasks from PRD content for implementation planning.

**Parameters:**
- `prd_content`: The PRD content to generate tasks from

**Returns:**
- `Result<String>`: Generated task list or error

**Example:**
```rust
let prd_content = std::fs::read_to_string("my_prd.md")?;
let tasks = agent.generate_technical_tasks(prd_content).await?;
println!("Generated tasks: {}", tasks);
```

**AI Model Configuration:**
- Model: `gemma3n:latest`
- Temperature: `0.7`
- Max Tokens: `None` (unlimited)

## Project Management API

### `prd_assistant::project::ProjectContext`

Manages project context and configuration.

#### `ProjectContext::find_current() -> Result<Option<Self>>`

Finds the current project context by searching up the directory tree.

**Returns:**
- `Result<Option<Self>>`: Project context if found, or None

**Example:**
```rust
use prd_assistant::project::ProjectContext;

let project = ProjectContext::find_current()?;
if let Some(project) = project {
    println!("Found project: {}", project.config.project_name);
}
```

#### `ProjectContext::ensure_project_context() -> Result<Self>`

Ensures a project context exists, returning an error if not found.

**Returns:**
- `Result<Self>`: Project context or error

**Example:**
```rust
let project = ProjectContext::ensure_project_context()?;
```

### `prd_assistant::project::ProjectConfig`

Project configuration structure.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project_name: String,
    pub default_template: String,
    pub strict_audit: bool,
}
```

**Fields:**
- `project_name`: The name of the project
- `default_template`: Default template to use for generation
- `strict_audit`: Whether to use strict auditing mode

### `prd_assistant::project::Templates`

Template management structure.

```rust
#[derive(Debug, Clone)]
pub struct Templates {
    pub default: String,
    pub technical: String,
}
```

**Fields:**
- `default`: Default PRD template content
- `technical`: Technical PRD template content

### `prd_assistant::project::ProjectRules`

Project rules structure.

```rust
#[derive(Debug, Clone)]
pub struct ProjectRules {
    pub audit_rules: String,
    pub generation_rules: String,
}
```

**Fields:**
- `audit_rules`: Rules for PRD auditing
- `generation_rules`: Rules for PRD generation

## Error Types

### `prd_assistant::error::Error`

The main error type for the crate.

```rust
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
}
```

**Error Variants:**
- `Io`: File system operation errors
- `Project`: Project-related errors (missing config, invalid project, etc.)
- `AgentError`: AI service errors (connection, response parsing, etc.)
- `Template`: Template processing errors
- `Toml`: Configuration serialization/deserialization errors

### `prd_assistant::error::Result<T>`

Type alias for `std::result::Result<T, Error>`.

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Configuration Schema

### Project Configuration (`config.toml`)

```toml
project_name = "my-project"
default_template = "default.md"
strict_audit = true
```

**Fields:**
- `project_name`: String - Project identifier
- `default_template`: String - Default template name
- `strict_audit`: Boolean - Enable strict auditing mode

### Template Structure

#### Default Template (`templates/default.md`)

```markdown
# {PROJECT_NAME} - {FEATURE_NAME}

## Overview
{OVERVIEW}

## Problem Statement
{PROBLEM_STATEMENT}

## Requirements
{REQUIREMENTS}

## Success Metrics
{METRICS}
```

#### Technical Template (`templates/technical.md`)

```markdown
# {PROJECT_NAME} - {FEATURE_NAME} (Technical)

## Technical Overview
{OVERVIEW}

## Architecture Requirements
{REQUIREMENTS}

## API Specifications
{API_SPECS}

## Performance Metrics
{METRICS}
```

### Rules Structure

#### Audit Rules (`rules/audit_rules.md`)

```markdown
# Audit Rules

## Required Sections
- Overview
- Requirements  
- Success Metrics

## Common Issues to Flag
- Vague language like "fast" or "user-friendly"
- Missing error handling
- No success criteria
```

#### Generation Rules (`rules/generation_rules.md`)

```markdown
# Generation Rules

## Always Include
- Clear problem statement
- Specific user stories
- Measurable success metrics
```

## Usage Examples

### Complete Workflow Example

```rust
use prd_assistant::commands::{init_project, generate_prd, audit_prd};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize project
    init_project("my-product").await?;
    
    // Generate PRD
    generate_prd(
        "my-product", 
        "User authentication system", 
        Some("technical")
    ).await?;
    
    // Audit PRD
    audit_prd(
        "my-product", 
        "User authentication system", 
        Some("Focus on security requirements")
    ).await?;
    
    Ok(())
}
```

### Custom Agent Usage

```rust
use prd_assistant::agent::PrdAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = PrdAgent::new("my-project".to_string())?;
    
    // Generate content
    let content = agent.generate_prd_content(
        "API rate limiting system".to_string()
    ).await?;
    
    // Validate content
    let validation = agent.validate_prd_content(content.clone()).await?;
    println!("Validation: {}", validation);
    
    // Audit with custom rules
    let audit = agent.audit_prd_content_with_rules(
        content,
        "Focus on performance and security"
    ).await?;
    println!("Audit: {}", audit);
    
    Ok(())
}
```

### Project Context Management

```rust
use prd_assistant::project::ProjectContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find current project
    if let Some(project) = ProjectContext::find_current()? {
        println!("Project: {}", project.config.project_name);
        println!("Default template: {}", project.config.default_template);
        println!("Strict audit: {}", project.config.strict_audit);
        
        // Access templates
        println!("Default template length: {}", project.templates.default.len());
        println!("Technical template length: {}", project.templates.technical.len());
        
        // Access rules
        println!("Audit rules: {}", project.rules.audit_rules);
        println!("Generation rules: {}", project.rules.generation_rules);
    } else {
        println!("No project found in current directory");
    }
    
    Ok(())
}
```

## Best Practices

### Error Handling

```rust
use prd_assistant::error::{Error, Result};

async fn handle_prd_operation() -> Result<()> {
    match generate_prd("project", "feature", None).await {
        Ok(()) => println!("Success!"),
        Err(Error::Project(msg)) => {
            eprintln!("Project error: {}", msg);
            // Handle project-specific errors
        }
        Err(Error::AgentError(msg)) => {
            eprintln!("AI error: {}", msg);
            // Handle AI service errors
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            // Handle other errors
        }
    }
    Ok(())
}
```

### Async/Await Usage

All API functions are async and should be used with `await`:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Always use await for async functions
    init_project("project").await?;
    generate_prd("project", "feature", None).await?;
    audit_prd("project", "feature", None).await?;
    Ok(())
}
```

### Resource Management

The agent manages its own resources and should be created per operation:

```rust
async fn process_multiple_features() -> Result<()> {
    let features = vec!["auth", "payment", "notifications"];
    
    for feature in features {
        let agent = PrdAgent::new("project".to_string())?;
        let content = agent.generate_prd_content(feature.to_string()).await?;
        // Process content...
    }
    
    Ok(())
}
```
