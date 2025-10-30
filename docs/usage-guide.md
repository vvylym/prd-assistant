# PRD Assistant - Usage Guide

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Project Management](#project-management)
- [PRD Generation](#prd-generation)
- [PRD Auditing](#prd-auditing)
- [Templates and Customization](#templates-and-customization)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

## Installation

### Prerequisites

Before using PRD Assistant, ensure you have the following installed:

1. **Rust Toolchain** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Ollama** with Gemma3n model
   ```bash
   # Install Ollama
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Start Ollama service
   ollama serve
   
   # Pull the required model (in another terminal)
   ollama pull gemma3n:latest
   ```

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd prd-assistant
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/prd-assistant`

### Installation Options

#### Option 1: Global Installation
```bash
cargo install --path .
```

#### Option 2: Local Development
```bash
cargo run -- <command>
```

## Quick Start

### 1. Initialize Your First Project

```bash
prd-assistant init-project my-awesome-product
```

This creates a project structure:
```
my-awesome-product/
├── .prd/
│   ├── config.toml
│   ├── templates/
│   │   ├── default.md
│   │   └── technical.md
│   └── rules/
│       ├── audit_rules.md
│       └── generation_rules.md
```

### 2. Generate Your First PRD

```bash
cd my-awesome-product
prd-assistant generate-prd my-awesome-product "User authentication system"
```

### 3. Audit the Generated PRD

```bash
prd-assistant audit-prd my-awesome-product "User authentication system"
```

### 4. Generate Technical Tasks

```bash
prd-assistant generate-tasks my-awesome-product "User authentication system"
```

## Project Management

### Project Structure

Each PRD project follows a standardized structure:

```
project-root/
├── .prd/                    # Project configuration directory
│   ├── config.toml         # Project settings
│   ├── templates/          # PRD templates
│   │   ├── default.md      # Standard template
│   │   └── technical.md    # Technical template
│   └── rules/              # Audit and generation rules
│       ├── audit_rules.md  # PRD auditing criteria
│       └── generation_rules.md # PRD generation guidelines
└── [generated-prd-files].md # Generated PRD documents
```

### Configuration Management

The `.prd/config.toml` file controls project behavior:

```toml
project_name = "my-awesome-product"
default_template = "default.md"
strict_audit = true
```

**Configuration Options:**
- `project_name`: Project identifier
- `default_template`: Default template for generation (`default.md` or `technical.md`)
- `strict_audit`: Enable strict auditing mode for more thorough reviews

### Working with Multiple Projects

You can work with multiple projects by navigating to different directories:

```bash
# Project A
cd project-a
prd-assistant generate-prd project-a "Feature X"

# Project B
cd ../project-b
prd-assistant generate-prd project-b "Feature Y"
```

## PRD Generation

### Basic Generation

Generate a PRD using the default template:

```bash
prd-assistant generate-prd <project-name> "<feature-description>"
```

**Example:**
```bash
prd-assistant generate-prd my-app "User dashboard with analytics and reporting"
```

### Using Different Templates

#### Default Template
```bash
prd-assistant generate-prd my-app "Payment processing system" default
```

#### Technical Template
```bash
prd-assistant generate-prd my-app "Microservices architecture" technical
```

### Feature Description Best Practices

**Good Examples:**
- "User authentication system with login, registration, and password reset"
- "Real-time chat functionality with file sharing and emoji reactions"
- "API rate limiting system with tiered access and monitoring"
- "Mobile app push notifications with user preferences and scheduling"

**Poor Examples:**
- "Make it better" (too vague)
- "Add features" (not specific)
- "Fix bugs" (not a feature)

### Generated File Naming

Generated PRDs are saved with sanitized filenames:
- Input: "User authentication system"
- Output: `my-app_user_authentication_system.md`

## PRD Auditing

### Basic Auditing

Audit a PRD using default rules:

```bash
prd-assistant audit-prd <project-name> "<feature-name>"
```

**Example:**
```bash
prd-assistant audit-prd my-app "User authentication system"
```

### Custom Audit Rules

Provide custom audit criteria:

```bash
prd-assistant audit-prd my-app "Payment system" "Focus on security requirements, PCI compliance, and error handling"
```

### Understanding Audit Reports

Audit reports include:

1. **Completeness Check**: Required sections present
2. **Clarity Assessment**: Language clarity and specificity
3. **User Focus**: User needs and stories addressed
4. **Technical Feasibility**: Realistic technical requirements
5. **Success Metrics**: Measurable success criteria
6. **Specific Recommendations**: Actionable improvement suggestions

**Sample Audit Output:**
```
📋 PRD AUDIT REPORT
==================================================
✅ Overview section present and clear
⚠️  Requirements need more specificity
❌ Missing error handling scenarios
✅ Success metrics are measurable
💡 Consider adding user personas
==================================================
```

## Technical Task Generation

### Basic Task Generation

Generate technical tasks from a PRD:

```bash
prd-assistant generate-tasks <project-name> "<feature-name>"
```

**Example:**
```bash
prd-assistant generate-tasks my-app "User authentication system"
```

### Using Specific PRD Files

Generate tasks from a specific PRD file:

```bash
prd-assistant generate-tasks my-app "feature" "custom_prd.md"
```

### Understanding Generated Tasks

Generated task lists include:

1. **Relevant Files**: List of files that need to be created or modified
2. **Parent Tasks**: High-level implementation phases (typically 5 tasks)
3. **Sub-tasks**: Detailed, actionable steps for each parent task
4. **Implementation Guidance**: Specific instructions for junior developers
5. **Testing Tasks**: Unit and integration test requirements

**Sample Task Output:**
```markdown
## Relevant Files

- `src/auth/mod.rs` - Main authentication module
- `src/auth/login.rs` - Login functionality
- `src/auth/register.rs` - User registration
- `tests/auth_test.rs` - Authentication tests

## Tasks

- [ ] 1.0 Set up authentication module structure
  - [ ] 1.1 Create auth module directory structure
  - [ ] 1.2 Define authentication traits and types
  - [ ] 1.3 Set up error handling for auth operations

- [ ] 2.0 Implement user registration
  - [ ] 2.1 Create user model and validation
  - [ ] 2.2 Implement password hashing
  - [ ] 2.3 Add email verification logic
```

## Templates and Customization

### Default Template Structure

The default template includes these sections:

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

### Technical Template Structure

The technical template focuses on implementation details:

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

### Customizing Templates

1. Edit template files in `.prd/templates/`:
   ```bash
   nano .prd/templates/default.md
   ```

2. Add custom placeholders:
   ```markdown
   ## Custom Section
   {CUSTOM_CONTENT}
   ```

3. Update generation rules in `.prd/rules/generation_rules.md`

### Custom Audit Rules

Modify audit criteria in `.prd/rules/audit_rules.md`:

```markdown
# Custom Audit Rules

## Required Sections
- Executive Summary
- User Stories
- Technical Requirements
- Security Considerations

## Quality Criteria
- All requirements must be testable
- User stories must include acceptance criteria
- Technical requirements must specify implementation approach
```

## Advanced Usage

### Batch Processing

Process multiple features:

```bash
#!/bin/bash
# batch-generate.sh

features=(
    "User authentication system"
    "Payment processing"
    "Notification system"
    "Analytics dashboard"
)

for feature in "${features[@]}"; do
    echo "Generating PRD for: $feature"
    prd-assistant generate-prd my-app "$feature"
done
```

### Integration with CI/CD

Add to your CI pipeline:

```yaml
# .github/workflows/prd-check.yml
name: PRD Quality Check
on: [pull_request]

jobs:
  prd-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build PRD Assistant
        run: cargo build --release
      - name: Audit PRDs
        run: |
          for prd in *.md; do
            ./target/release/prd-assistant audit-prd my-project "$prd"
          done
```

### Custom AI Models

Modify the agent to use different models:

```rust
// In src/agent/mod.rs
let model = self.client.completion_model("llama2:latest"); // Different model
```

### Environment Variables

Configure behavior via environment variables:

```bash
export PRD_ASSISTANT_DEBUG=1
export PRD_ASSISTANT_TEMPLATE_DIR=/custom/templates
export PRD_ASSISTANT_RULES_DIR=/custom/rules
```

## Troubleshooting

### Common Issues

#### 1. "No PRD project found"

**Problem:** Command fails with project not found error.

**Solution:**
```bash
# Ensure you're in a project directory
ls -la .prd/
# If missing, initialize project
prd-assistant init-project my-project
```

#### 2. "AI error: Connection refused"

**Problem:** Cannot connect to Ollama service.

**Solution:**
```bash
# Start Ollama service
ollama serve

# Verify model is available
ollama list
```

#### 3. "PRD file not found"

**Problem:** Audit command cannot find PRD file.

**Solution:**
```bash
# Check file exists
ls -la *feature_name*.md

# Use exact feature name from generation
prd-assistant audit-prd my-project "exact-feature-name"
```

#### 4. "Template error: Failed to load template"

**Problem:** Template file is missing or corrupted.

**Solution:**
```bash
# Recreate templates
rm -rf .prd/templates/
prd-assistant init-project my-project
```

### Debug Mode

Enable detailed logging:

```bash
RUST_LOG=debug prd-assistant generate-prd my-project "feature"
```

### Performance Issues

#### Slow AI Responses

1. **Check Ollama Status:**
   ```bash
   ollama ps
   ```

2. **Use Smaller Model:**
   ```bash
   ollama pull gemma2:2b  # Smaller model
   ```

3. **Optimize Prompts:**
   - Use more specific feature descriptions
   - Avoid overly complex requirements

#### Memory Usage

1. **Monitor Resource Usage:**
   ```bash
   htop
   ```

2. **Process Large PRDs in Chunks:**
   - Break down complex features
   - Generate multiple smaller PRDs

## Best Practices

### PRD Writing Guidelines

1. **Be Specific:**
   - Use concrete, measurable requirements
   - Avoid vague terms like "fast" or "user-friendly"
   - Include specific metrics and criteria

2. **User-Focused:**
   - Write from the user's perspective
   - Include user stories and personas
   - Address real user problems

3. **Technical Clarity:**
   - Specify technical requirements clearly
   - Include API specifications when relevant
   - Define performance criteria

### Project Organization

1. **Consistent Naming:**
   - Use descriptive project names
   - Follow consistent feature naming conventions
   - Use kebab-case for file names

2. **Version Control:**
   - Commit generated PRDs to version control
   - Use meaningful commit messages
   - Tag releases with PRD versions

3. **Documentation:**
   - Keep README files updated
   - Document custom templates and rules
   - Maintain change logs

### Collaboration

1. **Team Workflows:**
   - Establish PRD review processes
   - Use consistent templates across teams
   - Regular audit and improvement cycles

2. **Integration:**
   - Connect with project management tools
   - Automate PRD generation in workflows
   - Use CI/CD for quality checks

### Quality Assurance

1. **Regular Auditing:**
   - Audit PRDs before final approval
   - Use custom rules for specific domains
   - Track improvement over time

2. **Continuous Improvement:**
   - Update templates based on feedback
   - Refine audit rules regularly
   - Learn from successful PRDs

## Examples and Use Cases

### E-commerce Platform

```bash
# Initialize project
prd-assistant init-project ecommerce-platform

# Generate core features
prd-assistant generate-prd ecommerce-platform "Product catalog with search and filtering"
prd-assistant generate-prd ecommerce-platform "Shopping cart and checkout process"
prd-assistant generate-prd ecommerce-platform "User account management" technical
prd-assistant generate-prd ecommerce-platform "Payment processing with multiple gateways" technical

# Audit critical features
prd-assistant audit-prd ecommerce-platform "Payment processing with multiple gateways" "Focus on security, PCI compliance, and error handling"
```

### Mobile Application

```bash
# Initialize project
prd-assistant init-project mobile-app

# Generate mobile-specific features
prd-assistant generate-prd mobile-app "Push notification system"
prd-assistant generate-prd mobile-app "Offline data synchronization"
prd-assistant generate-prd mobile-app "User onboarding flow"
prd-assistant generate-prd mobile-app "In-app purchase system" technical

# Audit with mobile-specific rules
prd-assistant audit-prd mobile-app "Push notification system" "Focus on battery optimization, user privacy, and cross-platform compatibility"
```

### API Service

```bash
# Initialize project
prd-assistant init-project api-service

# Generate API-focused features
prd-assistant generate-prd api-service "RESTful API with authentication" technical
prd-assistant generate-prd api-service "Rate limiting and throttling" technical
prd-assistant generate-prd api-service "API documentation and versioning" technical
prd-assistant generate-prd api-service "Monitoring and analytics" technical

# Audit with API-specific rules
prd-assistant audit-prd api-service "RESTful API with authentication" "Focus on security, scalability, and API design best practices"
```

This usage guide provides comprehensive coverage of PRD Assistant's capabilities and should help users get the most out of the tool.
