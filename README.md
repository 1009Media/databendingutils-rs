# DatabendingUtils-RS
Converts common image codecs into formats more suitable for databending.

This is a Rust rewrite of a program I originally wrote in Python, from now on the old Python version is deprecated, and this is the one I'll update. The old Python code is available [here](https://github.com/sykesgabri/databendingutils).

## How to install:

Precompiled binaries for 64 bit Windows and 64 bit Linux are available in this repo's [releases](https://github.com/sykesgabri/databendingutils-rs/releases). Here's how to run them on each OS:

Windows (64 bit):

- Download `databendingutils-rs-X_X_X-windows.exe` from the releases page, and double click it.
- If Windows Defender gives you a warning prompt, click `More Info` and `Run Anyway`.

Linux (64 bit):

- Download `databendingutils-rs-X_X_X-linux` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x databendingutils-rs-X_X_X-linux` to make the program executable.
- Type `./databendingutils-rs-X_X_X-linux` to run the program.

## Building from source:

If you need the program on a platform not in the releases, such as 32 bit Windows or MacOS, follow these instructions:

- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Open a terminal or command prompt and type `git clone https://github.com/sykesgabri/databendingutils-rs`.
- When that finishes, type `cd databendingutils-rs`.
- Now type `cargo build --release`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". The compiled binary will be in there, and it will be named "databendingutils-rs" or "databendingutils-rs.exe" depending on the platform you compiled it on. You can move this binary wherever you want, and can run it the same way you would run the precompiled releases.

This project is licensed under the GNU GPL V3.