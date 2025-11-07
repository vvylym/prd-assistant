mod agent;
pub use agent::*;

use std::{collections::HashMap, path::PathBuf};

pub struct PrdAssistantConfig {
    pub project_name: String,
    pub project_root: PathBuf,
    pub template_root: PathBuf,
    pub rule_root: PathBuf,
    pub strict_audit: bool,
}

pub struct PrdAssistant {
    pub orchestrator: Option<Orchestrator>,
    pub retriever: SmartRetriever,
    pub generator: Option<Generator>,
    pub auditor: Option<Auditor>,
    pub researcher: Option<Researcher>,
    pub memory_manager: Option<MemoryManager>,
    pub template_store: Option<TemplateStore>,
    pub rule_store: Option<RuleStore>,
}



pub struct Orchestrator {}

pub struct SmartRetriever {}

pub struct Generator {}

pub struct Auditor {}

pub struct Researcher {}

pub struct MemoryManager {}

pub struct TemplateStore {
    pub rules: HashMap<TemplateType, String>,
}

pub struct RuleStore {
    pub rules: HashMap<RuleType, String>,
}

impl RuleStore {
    fn new(rule_root: PathBuf) -> Result<Self> {
        let mut rules = HashMap::new();
        for entry in std::fs::read_dir(rule_root)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let rule_type = RuleType::parse(file_name)?;
            let rule = std::fs::read_to_string(path)?;
            rules.insert(rule_type, rule);
        }
        Ok(Self { rules })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TemplateType {
    Default,
    Requirements,
    Tasks,
}



#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum RuleType {
    ProjectContext,
    RequirementsGeneration,
    RequirementsAudit,
    TasksGeneration,
    TasksAudit,
}

impl RuleType {
    fn parse(file_name: &str) -> Result<Self> {
        match file_name {
            "project_context.md" => Ok(Self::ProjectContext),
            "requirements_generation.md" => Ok(Self::RequirementsGeneration),
            "requirements_audit.md" => Ok(Self::RequirementsAudit),
            "tasks_generation.md" => Ok(Self::TasksGeneration),
            "tasks_audit.md" => Ok(Self::TasksAudit),
            _ => Err(Error::InvalidRuleFileName(file_name.to_string())),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid rule file name: {0}")]
    InvalidRuleFileName(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}