# prd-assistant

A powerful CLI tool for generating, auditing, and managing Product Requirement Documents (PRDs) using AI assistance.

## Features

- **Initialize PRD Projects** - Set up new projects with templates and rules
- **Generate PRDs** - Create comprehensive PRDs using AI with customizable templates
- **Audit PRDs** - Review and improve existing PRDs with detailed feedback
- **Template Support** - Multiple PRD templates (default, technical)
- **Custom Rules** - Define custom audit rules for specialized reviews
- **AI-Powered** - Uses Ollama with Gemma3n model for intelligent content generation

## Installation

### Prerequisites

- Rust (latest stable version)
- Ollama with Gemma3n model installed

### Build from Source

```bash
git clone <repository-url>
cd prd-assistant
cargo build --release
```

The binary will be available at `target/release/prd-assistant`.

## Quick Start

### 1. Initialize a New Project

```bash
prd-assistant init-project my-awesome-project
```

This creates a project structure with:
- Configuration file (`.prd/config.toml`)
- PRD templates (default and technical)
- Audit and generation rules

### 2. Generate a PRD

```bash
# Using default template
prd-assistant generate-prd my-awesome-project "User authentication system with login and registration"

# Using technical template
prd-assistant generate-prd my-awesome-project "API rate limiting system" technical
```

### 3. Audit a PRD

```bash
# Using default audit rules
prd-assistant audit-prd my-awesome-project "User authentication system with login and registration"

# Using custom audit rules
prd-assistant audit-prd my-awesome-project "API rate limiting system" "Focus on security requirements and performance metrics"
```

## Commands

### `init-project <PROJECT>`

Initialize a new PRD project with the specified name.

**Arguments:**
- `PROJECT` - Project name

**Example:**
```bash
prd-assistant init-project my-project
```

### `generate-prd <PROJECT> <FEATURE> [TEMPLATE]`

Generate a new PRD for the specified project and feature.

**Arguments:**
- `PROJECT` - Project name
- `FEATURE` - Feature description
- `TEMPLATE` - Optional template to use (default, technical)

**Examples:**
```bash
prd-assistant generate-prd my-project "User dashboard with analytics"
prd-assistant generate-prd my-project "Microservices architecture" technical
```

### `audit-prd <PROJECT> <FEATURE> [RULES]`

Audit an existing PRD and provide detailed feedback.

**Arguments:**
- `PROJECT` - Project name
- `FEATURE` - Feature name (must match generated PRD filename)
- `RULES` - Optional custom audit rules

**Examples:**
```bash
prd-assistant audit-prd my-project "User dashboard with analytics"
prd-assistant audit-prd my-project "Microservices architecture" "Focus on scalability and security"
```

## Project Structure

When you initialize a project, the following structure is created:

```
my-project/
├── .prd/
│   ├── config.toml              # Project configuration
│   ├── templates/
│   │   ├── default.md           # Default PRD template
│   │   └── technical.md         # Technical PRD template
│   └── rules/
│       ├── audit_rules.md       # Default audit rules
│       └── generation_rules.md  # Default generation rules
└── [generated PRD files]
```

## Configuration

The `.prd/config.toml` file contains project settings:

```toml
project_name = "my-project"
default_template = "default.md"
strict_audit = true
```

## Templates

### Default Template
Standard PRD template with sections for:
- Overview
- Problem Statement
- Requirements
- Success Metrics
- User Stories
- Technical Considerations

### Technical Template
Technical-focused template with sections for:
- Technical Overview
- Architecture Requirements
- API Specifications
- Performance Metrics

## Examples

### Example 1: User Authentication System

```bash
# Initialize project
prd-assistant init-project auth-system

# Generate PRD
prd-assistant generate-prd auth-system "User authentication with login, registration, and password reset"

# Audit the generated PRD
prd-assistant audit-prd auth-system "User authentication with login, registration, and password reset"
```

### Example 2: API Rate Limiting

```bash
# Generate technical PRD
prd-assistant generate-prd auth-system "API rate limiting system" technical

# Audit with custom rules
prd-assistant audit-prd auth-system "API rate limiting system" "Focus on security and performance requirements"
```

## AI Model

The tool uses Ollama with the Gemma3n model for:
- PRD content generation
- PRD auditing and feedback
- Quality assessment

Make sure Ollama is running and the Gemma3n model is available:
```bash
ollama pull gemma3n:latest
```

## Error Handling

The tool provides clear error messages for common issues:
- Missing project context
- PRD file not found
- AI model connection issues
- Invalid configuration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions:
1. Check the existing issues
2. Create a new issue with detailed description
3. Include error messages and steps to reproduce

