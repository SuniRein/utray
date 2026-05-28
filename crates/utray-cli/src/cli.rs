use clap::{Parser, Subcommand};

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
