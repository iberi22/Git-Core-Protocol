use clap::Args;
use gc_core::ports::SystemPort;
use console::style;

#[derive(Args, Debug)]
pub struct InfoArgs {}

pub async fn execute(
    _args: InfoArgs,
    system: &impl SystemPort,
) -> color_eyre::Result<()> {
    println!("{}", style("ℹ️ Project Info").bold());

    // Detect if solo or team
    // Simple heuristic: check number of contributors in git log
    let output = system.run_command_output("git", &["shortlog", "-s", "-n", "HEAD"].map(|s| s.to_string())).await?;
    let contributors = output.lines().count();

    let dev_type = if contributors > 1 { "Team" } else { "Solo" };

    println!("Development Type: {}", style(dev_type).cyan());
    println!("Contributors: {}", style(contributors).yellow());

    Ok(())
}
