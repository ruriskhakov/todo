mod database;
pub mod task;
use clap::{Parser, Subcommand};
use database::Database;

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all tasks
    List,
    /// Add a task
    Add {
        name: String,
    },
    // Finish a task
    Finish {
        id: u32,
    },
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => Database::list_tasks(),
        Some(Commands::Add { name }) => Database::add_task(name),
        Some(Commands::Finish { id }) => Database::end_task(id),
        None => {
            println!("Run with --help to see instructions.");
            std::process::exit(0);
        }
    }
}
