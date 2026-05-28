use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all tray items
    List,

    /// Get details of a specific tray item by ID
    Get(GetArgs),
}

#[derive(Args)]
pub struct GetArgs {
    /// ID of the tray item to retrieve
    pub id: String,

    /// Display the icon using the specified method
    #[arg(long, value_enum, value_name = "METHOD")]
    pub icon: Option<IconDisplayMode>,
}

#[derive(ValueEnum, Clone)]
pub enum IconDisplayMode {
    Rgba,
    Kitty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_debug_test() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
