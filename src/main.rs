use std::{env, fs};

use clap::{error::ErrorKind, CommandFactory, Parser};

mod cli;
use cli::Cli;

mod conflict;
use conflict::handle_argument_conflict;

mod generate_page;
use generate_page::{print_completions, print_man_page};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    handle_argument_conflict(&cli, &mut cmd);

    if let Some(shell) = cli.generate_completions {
        print_completions(shell, &mut Cli::command());
    }
    if cli.generate_markdown_page {
        clap_markdown::print_help_markdown::<Cli>();
    }
    if cli.generate_man_page {
        print_man_page(Cli::command())?;
    }

    if let Some(filename) = &cli.filename {
        let path = env::current_dir()?.join(filename);
        if fs::exists(&path)? {
            cmd.error(
                ErrorKind::Io,
                format!(
                    "file already exists: \"{}\"",
                    cli.filename
                        .expect("filename is not `None` after validation")
                ),
            )
            .exit();
        } else {
            println!(
                "Creating: \"{}\"",
                cli.filename
                    .expect("filename is not `None` after validation")
            );
            let _ = fs::File::create_new(&path)?;
        }
    }

    Ok(())
}
