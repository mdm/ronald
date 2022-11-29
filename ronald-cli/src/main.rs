#![allow(dead_code, unused_variables)]

use clap::{App, Arg};

use ronald_core::{Driver, system};

mod gui;
mod keybindings;
mod keyboard_configurator;

fn main() {
    env_logger::Builder::from_default_env()
        // .filter_level(log::LevelFilter::Info)
        .init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("an Amstrad CPC emulator")
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .value_name("DEBUG")
                .help("Runs the emulator in debug mode (not available for zexdoc and keyconfig)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("system")
                .short('s')
                .long("system")
                .value_name("SYSTEM")
                .help("Selects the system to run")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("floppy")
                .short('f')
                .long("floppy")
                .value_name("DSK_FILE")
                .help("Loads a DSK file")
                .takes_value(true),
        )
        .get_matches();

    let system = matches.value_of("system").unwrap_or("cpc464");

    match system {
        "cpc464" => {
            let debug = matches.is_present("debug");
            let mut driver = Driver::<system::CPC464>::new();
            if debug {
                driver.activate_debugger();
            }

            if let Some(dsk_filename) = matches.value_of("floppy") {
                match std::fs::read(dsk_filename) {
                    Ok(rom) => driver.load_disk(0, rom),
                    Err(err) => {
                        println!("Floppy load error: {err}");
                        return;
                    }
                }
                
            }

            gui::run(driver);
        }
        "zexdoc" => {
            let mut zex_harness = system::ZexHarness::new("rom/zexdoc.rom");
            zex_harness.emulate();
        }
        "keyconfig" => {
            let keyboard_configurator = keyboard_configurator::KeyboardConfigurator::new();
            keyboard_configurator.run();
        }
        unknown_system => {
            println!(
                "Unknown system \"{}\". Valid systems are:\n\n\tcpc464\n\tzexdoc\n\tkeyconfig\n",
                unknown_system
            );
        }
    }
}
