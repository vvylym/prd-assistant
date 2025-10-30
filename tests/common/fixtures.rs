// Duplicated minimal fixtures needed by integration/e2e tests

pub const DEFAULT_TEMPLATE: &str = r#"# {PROJECT_NAME} - {FEATURE_NAME}

## Overview
{OVERVIEW}

## Problem Statement
{PROBLEM_STATEMENT}

## Requirements
{REQUIREMENTS}

## Success Metrics
{METRICS}
"#;

pub const TECHNICAL_TEMPLATE: &str = r#"# {PROJECT_NAME} - {FEATURE_NAME} (Technical)

## Technical Overview
{OVERVIEW}

## Architecture Requirements
{REQUIREMENTS}

## API Specifications
{API_SPECS}

## Performance Metrics
{METRICS}
"#;

pub const DEFAULT_AUDIT_RULES: &str = r#"# Audit Rules

## Required Sections
- Overview
- Requirements  
- Success Metrics

## Common Issues to Flag
- Vague language like "fast" or "user-friendly"
- Missing error handling
- No success criteria
"#;

pub const DEFAULT_GENERATION_RULES: &str = r#"# Generation Rules

## Always Include
- Clear problem statement
- Specific user stories
- Measurable success metrics
"#;

pub const SAMPLE_PRD_CONTENT: &str = r#"# Test Project - User Authentication System

## Introduction/Overview
This feature provides user authentication capabilities for the test project.

## Goals
- Enable secure user login
- Provide user registration
- Implement password reset functionality

## User Stories
- As a user, I want to log in so that I can access my account
- As a user, I want to register so that I can create an account
- As a user, I want to reset my password so that I can regain access

## Functional Requirements
1. The system must allow users to log in with email and password
2. The system must allow users to register with email and password
3. The system must allow users to reset their password
4. The system must validate email format
5. The system must hash passwords securely

## Non-Goals (Out of Scope)
- Social media login
- Two-factor authentication
- Password complexity requirements

## Design Considerations
- Use a clean, simple login form
- Provide clear error messages
- Ensure mobile responsiveness

## Technical Considerations
- Use bcrypt for password hashing
- Implement JWT tokens for sessions
- Store user data in PostgreSQL

## Success Metrics
- 95% successful login rate
- Less than 2% registration abandonment
- User satisfaction score above 4.0/5.0

## Open Questions
- Should we implement remember me functionality?
- What password requirements should we enforce?
"#;


