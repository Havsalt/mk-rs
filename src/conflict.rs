use clap::{error::ErrorKind, Command};

use crate::cli::Cli;

pub fn handle_argument_conflict(cli: &Cli, cmd: &mut Command) {
    if cli.filename.is_some()
        && (cli.generate_completions.is_some()
            || cli.generate_markdown_page
            || cli.generate_man_page)
    {
        cmd.error(
            ErrorKind::ArgumentConflict,
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
