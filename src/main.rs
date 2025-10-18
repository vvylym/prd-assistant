use clap::{Parser, Subcommand};
use prd_assistant::{
    commands::{audit_prd, generate_prd, init_project},
    error::Result,
};
use tracing::info;

#[derive(Parser)]
#[command(name = "prd-assistant", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new PRD project
    InitProject {
        /// Project name
        project: String,
    },
    /// Generate a new PRD
    GeneratePRD {
        /// Project name
        project: String,
        /// User input
        feature: String,
        /// PRD Template to use
        template: Option<String>,
    },
    /// Audit an existing PRD
    AuditPRD {
        /// Project name
        project: String,
        /// Feature name
        feature: String,
        /// Rules to use
        rules: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    tracing_subscriber::fmt::init();
    info!("Starting PRD Assistant");

    let cli = Cli::parse();

    match cli.command {
        Commands::InitProject { project } => init_project(&project).await,
        Commands::GeneratePRD {
            project,
            feature,
            template,
        } => generate_prd(&project, &feature, template.as_deref()).await,
        Commands::AuditPRD {
            project,
            feature,
            rules,
        } => audit_prd(&project, &feature, rules.as_deref()).await,
    }
}
