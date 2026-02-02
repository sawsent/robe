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
use registry::{Registry, ToolRegistry};
use settings::Settings;

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    _main().unwrap_or_else(|err| {
        println!("{}", err);
    });
}

fn _main() -> Result<(), RobeError> {
    let full_args: Vec<String> = std::env::args().skip(1).collect();

    let command = domain::parse_cmd(&full_args)?;

    let settings_fp = utils::settings_file_path();

    let settings = utils::get_settings(&settings_fp);

    let registry = get_registry(&settings)?;

    match command {
        Command::Add(add) => add::add(&add, &registry)?,
        Command::Use(usecmd) => usecmd::usecmd(&usecmd, &registry)?,
        Command::Rm(rm) => rm::rm(&rm, &registry)?,
        Command::List(ls) => list::list(&ls, &registry)?,
        Command::Help(_cmd) => println!(
            "{}",
            help::help_with_storage_and_config(&settings.data_location, &settings_fp)
        ),
        Command::Version => println!("{}", help::VERSION),
    };

    Ok(())
}

fn get_registry(settings: &Settings) -> Result<Registry, RobeError> {
    let fp: PathBuf = PathBuf::from(&settings.data_location);

    fs::create_dir_all(&fp)?;

    let mut registry = Registry::default();
    registry.base_path = fp.clone();

    for tool in utils::get_subdirs(&fp)? {
        if let Ok(str) = fs::read_to_string(Path::join(&tool, "meta.toml")) {
            if let Ok(meta) = toml::from_str(&str) {
                let profiles = utils::get_files_in_dir_except(&tool, "meta.toml")?;
                if let Some(tool_name_os) = tool.file_name() {
                    let tool_name = tool_name_os.to_string_lossy().to_string();
                    let tool_registry = ToolRegistry::new(&tool_name, &meta, &profiles);
                    registry.tools.insert(tool_name, tool_registry);
                }
            }
        }
    }

    Ok(registry)
}
