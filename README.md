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

Loading DSK Files
-----------------

The `F5` key opens a dialog to load DSK files. These files are images of floppy
disks. Both standard and extended versions of the DSK format are supported.

Using The Debugger
------------------

`F12` starts a simple debugger in the terminal used to run the emulator. The
debugger shell expects one of the following commands:

* `disassemble` shows the next 10 instructions in human readable form.
* `registers` shows the current contents of the CPU registers.
* `breakpoint <address>` toggles a breakpoint at the given address (adresses
  can be decimal or hexadecimal, e.g. `0xc000`).
* `step <skip>` executes `<skip> + 1` instructions before dropping back into the
  debugger shell. If the argument is ommited it defaults to `0`, i.e. `step`
  executes a single instruction. Breakpoints might cause the debugger shell to
  be entered earlier.
* `continue` continues executing instructions until a breakpoint is reached or
  indefinitely if there are no breakpoints.
