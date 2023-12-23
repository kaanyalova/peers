use clap::Command;
use clap::{builder::Str, Args, CommandFactory, Parser, Subcommand};
use peers_core;

fn main() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand, Debug)]
    enum Commands {
        Add(AddArgs),
    }

    #[derive(Args, Debug)]
    struct AddArgs {
        path: String,
    }

    let cli = Cli::parse();

    let mut c = Cli::command();
    match &cli.command {
        Some(Commands::Add(t)) => add_torrent(&t.path),
        // if
        None => no_subcommands(&mut c),
    };
}

fn add_torrent(path: &str) {
    println!("added torrent with path {}", path)
}

#[cfg(feature = "gui")]
fn no_subcommands(c: Command) {
    println!("run gui");
}

#[cfg(not(feature = "gui"))]
fn no_subcommands(c: &mut Command) {
    c.print_help().unwrap();
}
