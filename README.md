# DatabendingUtils-RS
Converts common image codecs into formats more suitable for databending.

This is a Rust rewrite of a program Gabriel Sykes originally wrote in Python, from now on the old Python version is deprecated, and this is the one that will be updated. The old Python code is available [here](https://github.com/sykesgabri/databendingutils).

## How to install:

Precompiled portable binaries of the program exist for Windows, Linux, and MacOS. Download them from the links below:

Windows (x86_64):

- Download [`DatabendingUtils-RS_X-X-X_x64_PORTABLE.exe`](https://github.com/1009Media/databendingutils-rs/releases/download/2.0.0/DatabendingUtils-RS_2-0-0_x64_PORTABLE.exe), and double click it.
- If Windows Defender gives you a warning prompt, click `More Info` and `Run Anyway`.

Linux (x86_64):

- Download [`DatabendingUtils-RS_X-X-X_x64_PORTABLE`](https://github.com/1009Media/databendingutils-rs/releases/download/2.0.0/DatabendingUtils-RS_2-0-0_x64_PORTABLE).
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x DatabendingUtils-RS_X-X-X_x64_PORTABLE` to make the program executable.
- Type `./DatabendingUtils-RS_X-X-X_x64_PORTABLE` to run the program.

MacOS (aarch64):

- Download [`DatabendingUtils-RS_X-X-X_aarch64_PORTABLE.app.zip`](https://github.com/1009Media/databendingutils-rs/releases/download/2.0.0/DatabendingUtils-RS_2-0-0_aarch64_PORTABLE.app.zip), extract it, and double click it.
- You'll probably get a warning about an untrusted developer, forcing you to go into privacy settings to allow the program to run, [this is Apple trying to extort developers by making them pay to remove this message](https://developer.apple.com/support/compare-memberships/), fuck the cunts.

## How to use:

Refer to this repo's [wiki](https://github.com/1009Media/databendingutils-rs/wiki) for detailed instructions on usage of the program itself, as well as how to databend.

## Building from source:

If you need the program on a platform not in the releases, such as 32 bit Windows or MacOS, follow these instructions:

- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Install the [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/).
- Open a terminal or command prompt and type `cargo install tauri-cli`.
- After that finishes, type `git clone https://github.com/1009Media/databendingutils-rs`.
- When that finishes, type `cd databendingutils-rs`.
- Now type `cargo tauri build`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". Inside that folder (on Windows and Linux) will be a file called "databendingutils-rs" or "databendingutils-rs.exe", this is the portable version of the program.
- Inside the "bundle" folder you will find the various non-portable installers for Windows, MacOS, and Linux. Note for Mac users, the .app file you find here is your portable file, the .dmg is your permanent installer.

## CONTRIBUTORS:

- Gabriel Sykes

---
This project is licensed under the GNU GPL V3.
