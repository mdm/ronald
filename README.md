**Try a hosted demo of the WebAssembly version at https://ronald.migge.io


Ronald - An Amstrad CPC Emulator
================================

Ronald is an emulator for the Amstrad CPC written in Rust. It was written as a
way to teach myself more about both emulators and Rust. It runs a couple of
games that I remember fondly from my childhood. However, it has quite a few
limitations and is not a feature complete as some of the other CPC emulators out
there. For example:

* It only supports the CPC 464 model
* Basic reading from floppy disk is supported, but writing to disk is not
* Reading from tape is not supported at all
* Audio support is very basic, i.e. it lacks noise channels and volume envelopes
* It has only been tested on Linux (though the dependencies are cross-platform)

Running The Emulator
--------------------

If you have Rust installed you can easily run the emulator from the command line
using

`cargo run --release`

There are a few optional arguments. To see their documentation run

`cargo run --release -- --help`

***Note:*** You need a valid keyboard configuration, to run the desktop version of the
emulator. To get started renaming the included `keyconfig.yml.example` to `keyconfig.yml`
should be sufficient.

Loading DSK Files
-----------------

The `F5` key opens a dialog to load DSK files. These files are images of floppy
disks. Both standard and extended versions of the DSK format are supported.

Browser Version
---------------

The repo contains a WASM version as well. To run it you need Node installed and
execute the following commands:

1. `cd ronald-web`
2. `npm install`
3. `npm run dev`

You can drag and drop DSK files onto the emulator screen to load them.

