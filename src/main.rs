use std::{env, fs, io};

use clap::{error::ErrorKind, Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use clap_mangen::Man;
use havsalt_clap_styles::STYLES;

#[derive(Parser, Debug)]
#[command(
    name = "mk",
    version,
    about = "Command for creating empty files, like mkdir - but for file",
    long_about = None,
    styles = STYLES
)]
struct Cli {
    #[arg(help = "Filename to create")]
    filename: Option<String>,
    #[arg(
        long = "completion",
        value_name = "SHELL",
        help = "Generate completions for shell"
    )]
    generate_completions: Option<Shell>,
    #[arg(long = "markdown", help = "Generate markdown help page")]
    generate_markdown_page: bool,
    #[arg(long = "man", help = "Generate man page for program")]
    generate_man_page: bool,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn print_man_page(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    let man = Man::new(cmd);
    man.render(&mut io::stdout())?;
    Ok(())
}

fn handle_argument_conflict(cli: &Cli, cmd: &mut Command) {
    if cli.filename.is_some()
        && (cli.generate_completions.is_some()
            || cli.generate_markdown_page
            || cli.generate_man_page)
    {
        cmd.error(
            clap::error::ErrorKind::ArgumentConflict,
            format!(
                "{}: {} / {} / {}",
                "positional argument [FILENAME] cannot be used in combination with one of",
                "`--completion`",
                "`--markdown`",
                "`--man`"
            ),
        )
        .exit();
    } else {
        let render_flags = [
            ("completions", cli.generate_completions.is_some()),
            ("markdown", cli.generate_markdown_page),
            ("man", cli.generate_man_page),
        ];
        let render_flags_present = render_flags.iter().filter(|pair| pair.1);
        let count = render_flags_present.clone().count();
        if count > 1 {
            let flags = render_flags_present
                .map(|pair| format!("`--{}`", pair.0))
                .collect::<Vec<String>>()
                .join(", ");
            cmd.error(
                ErrorKind::ArgumentConflict,
                format!("cannot render more than 1 option, found {count}: {flags}"),
            )
            .exit();
        } else if cli.filename.is_none() {
            cmd.error(
                ErrorKind::MissingRequiredArgument,
                "missing positional argument [FILENAME]",
            )
            .exit();
        }
    }
}

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
