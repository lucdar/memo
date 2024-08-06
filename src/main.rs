use anyhow::{Result, Context};
use clap::{CommandFactory, Parser, Subcommand};
use directories::UserDirs;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;

/// A CLI for jotting down your thoughts.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long, env)]
    memo_path: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// create a new memo in your text editor
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file
    Compose {
        /// Optionally, a title for the memo
        ///
        /// Used as the filename for the memo.
        /// If no title is supplied, the filename
        /// will be set to the current time.
        #[clap(short, long)]
        title: Option<String>,
    },
    /// pick a memo to edit from a list
    ///
    /// Use arrow keys to navigate and enter to select
    /// the memo to open in your $EDITOR
    Edit {
        /// Include to open a random memo
        #[clap(short, long)]
        random: bool,
    },
}

/// Get the user's default memos directory
/// Placed in their home directory by default
fn get_default_memos_dir() -> Option<PathBuf> {
    UserDirs::new().map(|dirs| dirs.home_dir().join(".memos"))
}

/// Prompt the user for an answer to a yes or no question
/// Returns true for "y" or "Y", false for anything else.
fn ask_confirm(question: &str) -> bool {
    println!("{} (Y/n)", question);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => return (input == "y\n") || (input == "Y\n"),
        _ => return false,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    // dbg!(&args);

    // Validate memos dir
    let Some(memos_dir) = args.memo_path.or_else(get_default_memos_dir) else {
        let mut cmd = Args::command();
        cmd.error(
            clap::error::ErrorKind::ValueValidation,
            format!("memo directory not provided and home directory not available for memo path creation"),
        )
        .exit()
    };
    if !memos_dir.exists() {
        println!("memos directory `{}` does not exist.", memos_dir.display());
        if !ask_confirm("Would you like to create a new directory there?") {
            exit(1);
        }
        if let Err(e) = create_dir(&memos_dir) {
            eprintln!("Failed to create memos directory: {e}");
            exit(1);
        }
    }
    // dbg!(&memos_dir);

    // Execute Command
    match args.cmd {
        Commands::Compose { title } => {
            // dbg!(&title);
            memo::compose(memos_dir, title)
        }
        Commands::Edit { random } => memo::edit(memos_dir, random),
    }
}
