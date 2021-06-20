mod bus;
mod cpu;
mod crtc;
mod fdc;
mod gate_array;
mod gui;
mod instruction;
mod keyboard;
mod memory;
mod ppi;
mod screen;
mod system;

use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("an Amstrad CPC emulator")
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
            let cpc = Box::new(system::CPC464::new());
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
