# PRD Assistant - Architecture Documentation

## Overview

PRD Assistant is a Rust-based CLI tool designed to streamline the creation, management, and auditing of Product Requirement Documents (PRDs) using AI assistance. The tool leverages Ollama with the Gemma3n model to provide intelligent content generation and quality assessment.

## System Architecture

### High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Interface │    │  Command Layer  │    │  Business Logic │
│   (clap)        │◄──►│   (commands)    │◄──►│    (agent)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                       │
                                ▼                       ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │ Project Context │    │  AI Integration │
                       │   (project)     │    │    (rig/ollama) │
                       └─────────────────┘    └─────────────────┘
```

### Core Components

#### 1. CLI Interface (`src/main.rs`)
- **Purpose**: Entry point and command-line interface
- **Technology**: Clap for argument parsing
- **Responsibilities**:
  - Parse command-line arguments
  - Route commands to appropriate handlers
  - Initialize logging and error handling

#### 2. Command Layer (`src/commands/`)
- **Purpose**: Business logic orchestration
- **Modules**:
  - `init.rs`: Project initialization
  - `generate.rs`: PRD generation
  - `audit.rs`: PRD auditing
  - `generate_tasks.rs`: Technical task generation
- **Responsibilities**:
  - Coordinate between CLI and business logic
  - Handle file operations
  - Format output and user feedback

#### 3. AI Agent (`src/agent/mod.rs`)
- **Purpose**: AI-powered content generation and analysis
- **Technology**: Rig framework with Ollama provider
- **Responsibilities**:
  - Generate PRD content using AI
  - Audit existing PRDs
  - Validate PRD quality
  - Generate technical tasks from PRDs
  - Manage AI model interactions

#### 4. Project Management (`src/project.rs`)
- **Purpose**: Project context and configuration management
- **Responsibilities**:
  - Load and validate project configurations
  - Manage templates and rules
  - Handle project discovery and initialization

#### 5. Error Handling (`src/error.rs`)
- **Purpose**: Centralized error management
- **Technology**: ThisError for error derivation
- **Error Types**:
  - `Io`: File system operations
  - `Project`: Project-related errors
  - `AgentError`: AI service errors
  - `Template`: Template processing errors
  - `Toml`: Configuration serialization errors

## Data Flow

### PRD Generation Flow

```
User Input → CLI Parser → Generate Command → PrdAgent → AI Model → Generated Content → File System
```

1. User provides feature description via CLI
2. Command parser validates and routes to generate handler
3. Generate command loads project context
4. PrdAgent creates AI request with structured prompt
5. Ollama processes request and returns generated content
6. Content is saved to project directory with sanitized filename

### PRD Auditing Flow

```
PRD File → Audit Command → PrdAgent → AI Model → Audit Report → Console Output
```

1. User specifies PRD file to audit
2. Audit command loads PRD content and audit rules
3. PrdAgent creates audit request with rules context
4. AI model analyzes PRD against criteria
5. Formatted audit report is displayed to user

### Technical Task Generation Flow

```
PRD File → Generate Tasks Command → PrdAgent → AI Model → Task List → File System
```

1. User specifies PRD file or feature name
2. Generate tasks command loads PRD content
3. PrdAgent creates task generation request with PRD context
4. AI model analyzes PRD and generates detailed task breakdown
5. Task list is saved to project directory as markdown file

## Configuration Management

### Project Structure
```
project-root/
├── .prd/
│   ├── config.toml          # Project configuration
│   ├── templates/
│   │   ├── default.md       # Standard PRD template
│   │   └── technical.md     # Technical PRD template
│   └── rules/
│       ├── audit_rules.md   # Audit criteria
│       └── generation_rules.md # Generation guidelines
└── [generated-prd-files].md
```

### Configuration Schema
```toml
[project]
project_name = "string"        # Project identifier
default_template = "string"    # Default template name
strict_audit = bool           # Enable strict auditing mode
```

## AI Integration

### Model Configuration
- **Provider**: Ollama
- **Model**: Gemma3N (latest)
- **Temperature**: 0.7 (balanced creativity/consistency)
- **Max Tokens**: None (unlimited)

### Prompt Engineering
- **Generation**: Structured prompts with clear sections
- **Auditing**: Context-aware prompts with custom rules
- **Validation**: Specific criteria-based validation prompts

## Error Handling Strategy

### Error Propagation
```
IO Error → Project Error → Agent Error → CLI Error → User Display
```

### Error Recovery
- Graceful degradation for missing files
- Fallback to default configurations
- Clear error messages with suggested actions

## Security Considerations

### Input Sanitization
- Filename sanitization for generated files
- Input validation for user-provided content
- Safe path handling to prevent directory traversal

### AI Safety
- Structured prompts to prevent prompt injection
- Content validation before file writing
- Error handling for AI service failures

## Performance Characteristics

### Memory Usage
- Minimal memory footprint
- Streaming AI responses
- Efficient string handling

### I/O Operations
- Lazy loading of project configurations
- Cached template and rule loading
- Optimized file operations

## Extensibility Points

### Template System
- Pluggable template architecture
- Custom template support
- Template validation

### Rule System
- Customizable audit rules
- Rule inheritance and composition
- Rule validation

### AI Provider
- Abstracted AI interface via Rig
- Easy provider switching
- Model configuration flexibility

## Dependencies

### Core Dependencies
- `clap`: Command-line argument parsing
- `rig-core`: AI framework abstraction
- `reqwest`: HTTP client for AI requests
- `serde`: Serialization/deserialization
- `tokio`: Async runtime
- `toml`: Configuration file handling

### Development Dependencies
- `thiserror`: Error handling
- `colored`: Terminal output formatting
- `tracing`: Logging and observability

## Testing Strategy

### Unit Tests
- Individual component testing
- Mock AI responses
- Configuration validation

### Integration Tests
- End-to-end command testing
- File system operations
- AI service integration

### Error Testing
- Error condition simulation
- Recovery mechanism validation
- User experience testing

## Deployment Considerations

### Build Requirements
- Rust toolchain (latest stable)
- Ollama service running
- Gemma3n model available

### Runtime Requirements
- Network access for AI requests
- File system write permissions
- Sufficient disk space for generated content

## Future Enhancements

### Planned Features
- Multiple AI provider support
- Advanced template system
- Collaborative editing support
- Version control integration
- Web-based interface

### Scalability Considerations
- Batch processing capabilities
- Distributed processing support
- Caching mechanisms
- Performance monitoring
