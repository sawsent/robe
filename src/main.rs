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
use registry::{Registry, TargetRegistry};
use settings::Settings;

use std::collections::HashMap;
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

fn get_registry(settings: &Settings) -> Result<Registry, RobeError> {
    let fp: PathBuf = PathBuf::from(&settings.wardrobe);

    fs::create_dir_all(&fp)?;

    let mut registered: HashMap<String, TargetRegistry> = HashMap::new();

    for target in utils::get_subdirs(&fp)? {
        if let Ok(str) = fs::read_to_string(Path::join(&target, "meta.toml"))
            && let Ok(meta) = toml::from_str(&str)
        {
            let profiles = utils::get_profiles_from_dir(&target, "meta.toml")?;
            if let Some(target_name_os) = target.file_name() {
                let target_name = target_name_os.to_string_lossy().to_string();
                let target_registry = TargetRegistry::new(&target_name, &meta, &profiles);
                registered.insert(target_name, target_registry);
            }
        }
    }

    Ok(Registry {
        base_path: fp.clone(),
        targets: registered,
    })
}
