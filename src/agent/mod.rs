use crate::error::{Error, Result};
use rig::completion::{
    AssistantContent, CompletionModel, CompletionRequest, Message as CompletionMessage,
};
use rig::message::{Text, UserContent};
use rig::one_or_many::OneOrMany;
use rig::prelude::*;
use rig::providers::ollama;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PrdAgent {
    project_name: String,
    client: ollama::Client<reqwest::Client>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrdGenerationRequest {
    pub project_name: String,
    pub user_input: String,
    pub template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrdAuditRequest {
    pub prd_content: String,
    pub audit_rules: String,
}

impl PrdAgent {
    pub fn new(project_name: String) -> Result<Self> {
        let client = ollama::Client::new();
        Ok(Self {
            project_name,
            client,
        })
    }

    pub async fn generate_prd_content(&self, user_input: String) -> Result<String> {
        let prompt = format!(
            "You are a Product Requirements Document (PRD) expert. \
            Generate a comprehensive PRD for the project '{}' based on the following input: {}\n\n\
            Please structure the PRD with the following sections:\n\
            1. Overview\n\
            2. Problem Statement\n\
            3. Requirements\n\
            4. Success Metrics\n\
            5. User Stories\n\
            6. Technical Considerations\n\n\
            Make sure the content is specific, measurable, and actionable.",
            self.project_name, user_input
        );

        let model = self.client.completion_model("gemma3n:latest");

        let request = CompletionRequest {
            preamble: Some(
                "You are an expert product manager who creates detailed, actionable PRDs."
                    .to_string(),
            ),
            chat_history: OneOrMany::one(CompletionMessage::User {
                content: OneOrMany::one(UserContent::Text(Text { text: prompt })),
            }),
            documents: vec![],
            tools: vec![],
            temperature: Some(0.7),
            max_tokens: None,
            tool_choice: None,
            additional_params: None,
        };

        let response = model
            .completion(request)
            .await
            .map_err(|e| Error::AgentError(format!("Failed to generate PRD: {}", e)))?;

        // Extract text content from the response
        let content = match response.choice.first() {
            AssistantContent::Text(text) => text.text,
            _ => {
                return Err(Error::AgentError(
                    "No text content in AI response".to_string(),
                ));
            }
        };

        Ok(content)
    }

    pub async fn audit_prd_content(&self, prd_content: String) -> Result<String> {
        self.audit_prd_content_with_rules(prd_content, "").await
    }

    pub async fn audit_prd_content_with_rules(
        &self,
        prd_content: String,
        audit_rules: &str,
    ) -> Result<String> {
        let rules_context = if audit_rules.is_empty() {
            "Please evaluate the PRD based on these criteria:\n\
            1. Completeness - Are all required sections present?\n\
            2. Clarity - Is the language clear and unambiguous?\n\
            3. Specificity - Are requirements specific and measurable?\n\
            4. User Focus - Are user needs clearly addressed?\n\
            5. Technical Feasibility - Are technical requirements realistic?\n\
            6. Success Metrics - Are success criteria well-defined?\n\n\
            Provide specific feedback and suggestions for improvement."
                .to_string()
        } else {
            format!(
                "Please evaluate the PRD based on these specific audit rules:\n\n{}\n\nProvide specific feedback and suggestions for improvement.",
                audit_rules
            )
        };

        let prompt = format!(
            "You are a PRD quality auditor. Please review the following PRD and provide a detailed audit report:\n\n\
            {}\n\n\
            {}",
            prd_content, rules_context
        );

        let model = self.client.completion_model("gemma3n:latest");

        let request = CompletionRequest {
            preamble: Some(
                "You are an expert PRD auditor who provides detailed, constructive feedback."
                    .to_string(),
            ),
            chat_history: OneOrMany::one(CompletionMessage::User {
                content: OneOrMany::one(UserContent::Text(Text { text: prompt })),
            }),
            documents: vec![],
            tools: vec![],
            temperature: Some(0.7),
            max_tokens: None,
            tool_choice: None,
            additional_params: None,
        };

        let response = model
            .completion(request)
            .await
            .map_err(|e| Error::AgentError(format!("Failed to audit PRD: {}", e)))?;

        // Extract text content from the response
        let content = match response.choice.first() {
            AssistantContent::Text(text) => text.text,
            _ => {
                return Err(Error::AgentError(
                    "No text content in AI response".to_string(),
                ));
            }
        };

        Ok(content)
    }

    pub async fn validate_prd_content(&self, prd_content: String) -> Result<String> {
        let prompt = format!(
            "You are a PRD validator. Please validate the following PRD and provide a validation report:\n\n\
            {}\n\n\
            Check for:\n\
            1. Required sections are present\n\
            2. Requirements are testable\n\
            3. Success metrics are measurable\n\
            4. User stories follow proper format\n\
            5. Technical requirements are realistic\n\n\
            Provide a pass/fail status with specific issues if any.",
            prd_content
        );

        let model = self.client.completion_model("gemma3n:latest");

        let request = CompletionRequest {
            preamble: Some("You are a PRD validator who provides clear pass/fail assessments with specific feedback.".to_string()),
            chat_history: OneOrMany::one(CompletionMessage::User {
                content: OneOrMany::one(UserContent::Text(Text { text: prompt })),
            }),
            documents: vec![],
            tools: vec![],
            temperature: Some(0.7),
            max_tokens: None,
            tool_choice: None,
            additional_params: None,
        };

        let response = model
            .completion(request)
            .await
            .map_err(|e| Error::AgentError(format!("Failed to validate PRD: {}", e)))?;

        // Extract text content from the response
        let content = match response.choice.first() {
            AssistantContent::Text(text) => text.text,
            _ => {
                return Err(Error::AgentError(
                    "No text content in AI response".to_string(),
                ));
            }
        };

        Ok(content)
    }
}
