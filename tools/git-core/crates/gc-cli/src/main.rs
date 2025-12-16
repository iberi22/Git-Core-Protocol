use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gc", version, about = "Git-Core Protocol CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

mod commands;
use commands::{InitArgs, ContextCmd, ReportCmd, ValidateCmd, TelemetryArgs, CiDetectArgs};

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init(InitArgs),
    /// Manage Agent Context
    Context {
        #[command(subcommand)]
        subcmd: ContextCmd,
    },
    /// Generate AI Reports
    #[command(subcommand)]
    Report(ReportCmd),
    /// Collect and Send Telemetry
    Telemetry(TelemetryArgs),
    /// Detect CI Environment
    CiDetect(CiDetectArgs),
    /// Validate Workflows
    #[command(subcommand)]
    Validate(ValidateCmd),
    /// Execute Workflows
    Workflow {
        /// Workflow to validate
        #[arg(long)]
        name: Option<String>,
    },
}



#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            commands::init::execute(args, &fs, &system).await?;
        }
        Commands::Context { subcmd } => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let github = gc_adapter_github::OctocrabGitHub::new();
            commands::context::execute(subcmd, &fs, &github).await?;
        }
        Commands::Report(args) => {
            let _fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            let github = gc_adapter_github::OctocrabGitHub::new();
            // TODO: Refactor adapter instantiation to be shared or dependency injection container
            commands::report::execute(args, &system, &github).await?;
        }
        Commands::Telemetry(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::telemetry::execute(args, &system).await?;
        }
        Commands::CiDetect(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::ci_detect::execute(args, &system).await?;
        }
        Commands::Validate(args) => {
            commands::validate::execute(args).await?;
        }
        Commands::Workflow { name } => {
            println!("Validating workflow: {:?}", name);
        }
    }

    Ok(())
}
