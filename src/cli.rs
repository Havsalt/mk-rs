use clap::Parser;
use clap_complete::Shell;
use havsalt_clap_styles::STYLES;

#[derive(Parser, Debug)]
#[command(
    name = "mk",
    version,
    about = "Command for creating empty files, like mkdir - but for file",
    long_about = None,
    styles = STYLES
)]
pub struct Cli {
    #[arg(help = "Filename to create")]
    pub filename: Option<String>,
    #[arg(
        long = "completion",
        value_name = "SHELL",
        help = "Generate completions for shell"
    )]
    pub generate_completions: Option<Shell>,
    #[arg(long = "markdown", help = "Generate markdown help page")]
    pub generate_markdown_page: bool,
    #[arg(long = "man", help = "Generate man page for program")]
    pub generate_man_page: bool,
}
