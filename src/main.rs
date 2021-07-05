#![allow(dead_code, unused_variables)]

mod bus;
mod cpu;
mod crtc;
mod debugger;
mod fdc;
mod gate_array;
mod gui;
mod instruction;
mod keyboard;
mod memory;
mod ppi;
mod psg;
mod screen;
mod system;
mod tape;

use clap::{App, Arg};

use crate::system::System;

fn main() {
    env_logger::init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("an Amstrad CPC emulator")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .value_name("DEBUG")
                .help("Runs the emulator in debug mode (not available for zexdoc)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("system")
                .short("s")
                .long("system")
                .value_name("SYSTEM")
                .help("Selects the system to run")
                .takes_value(true),
        )
        .get_matches();

    let system = matches.value_of("system").unwrap_or("cpc464");

    match system {
        "cpc464" => {
            let debug = matches.is_present("debug");
            let mut cpc = Box::new(system::CPC464::new());
            if debug {
                cpc.activate_debugger();
            }

            let mut gui = gui::GUI::new(cpc);
            gui.run();
        }
        "zexdoc" => {
            let mut zex_harness = system::ZexHarness::new("rom/zexdoc.rom");
            zex_harness.emulate();
        }
        unknown_system => {
            println!(
                "Unknown system \"{}\". Valid systems are:\n\n\tcpc464\n\tzexdoc\n",
                unknown_system
            );
        }
    }
}
