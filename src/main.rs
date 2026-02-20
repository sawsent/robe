mod dispatch;
mod domain;
mod errors;
mod help;
mod registry;
mod settings;
mod utils;

use dispatch::*;
use domain::Command;
use errors::RobeError;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = _main(&args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn _main(args: &[String]) -> Result<(), RobeError> {
    let command = domain::parse_cmd(args)?;

    let settings_fp = utils::settings_file_path();

    let settings = utils::get_settings(&settings_fp);

    let registry = utils::get_registry(&settings)?;

    match command {
        Command::Add(add) => add::add(&add, &registry)?,
        Command::Register(reg) => register::register(&reg, &registry)?,
        Command::Edit(edit) => edit::edit(&edit, &registry)?,
        Command::Use(usecmd) => usecmd::usecmd(&usecmd, &registry)?,
        Command::Rm(rm) => rm::rm(&rm, &registry)?,
        Command::List(ls) => list::list(&ls, &registry)?,
        Command::View(view) => view::view(&view, &registry)?,
        Command::Help(_cmd) => println!(
            "{}",
            help::help_with_storage_and_config(&settings.wardrobe, &settings_fp)
        ),
        Command::Version => println!("{}", help::VERSION),
    };

    Ok(())
}
