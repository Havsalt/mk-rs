use std::io;

use clap::Command;
use clap_complete::{generate, Generator};
use clap_mangen::Man;

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn print_man_page(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    let man = Man::new(cmd);
    man.render(&mut io::stdout())?;
    Ok(())
}
