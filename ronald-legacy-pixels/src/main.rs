#![allow(dead_code, unused_variables)]

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

use ronald_core::{system, Driver};
use winit::event_loop::{ControlFlow, EventLoop};

use gui::RonaldApp;

mod gui;
mod keybindings;
mod keyboard_configurator;

fn main() {
    env_logger::Builder::from_default_env()
        // .filter_level(log::LevelFilter::Info)
        .init();

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("an Amstrad CPC emulator")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .value_name("DEBUG")
                .help("Runs the emulator in debug mode (not available for zexdoc and keyconfig)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("system")
                .short('s')
                .long("system")
                .value_name("SYSTEM")
                .help("Selects the system to run")
                .default_value("cpc464"),
        )
        .arg(
            Arg::new("floppy")
                .short('f')
                .long("floppy")
                .value_name("DSK_FILE")
                .help("Loads a DSK file"),
        )
        .get_matches();

    let system = matches.get_one::<String>("system").unwrap();

    match system.as_str() {
        "cpc464" => {
            let debug = matches.get_flag("debug");
            let mut driver = Driver::<system::AmstradCpc464>::new();
            if debug {
                driver.activate_debugger();
            }

            if let Some(dsk_filename) = matches.get_one::<String>("floppy") {
                match std::fs::read(dsk_filename) {
                    Ok(rom) => driver.load_disk(0, rom, PathBuf::from(dsk_filename)),
                    Err(err) => {
                        println!("Floppy load error: {err}");
                        return;
                    }
                }
            }

            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            let mut app = RonaldApp::new(driver);
            event_loop.run_app(&mut app);
        }
        "zexdoc" => {
            let mut zex_harness = system::ZexHarness::new("rom/zexdoc.rom");
            zex_harness.emulate();
        }
        "keyconfig" => {
            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            let mut app = keyboard_configurator::KeyboardConfigurator::new();
            event_loop.run_app(&mut app);
        }
        unknown_system => {
            println!(
                "Unknown system \"{unknown_system}\". Valid systems are:\n\n\tcpc464\n\tzexdoc\n\tkeyconfig\n"
            );
        }
    }
}
