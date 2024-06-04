# DatabendingUtils-RS
Converts common image codecs into formats more suitable for databending.

This is a Rust rewrite of a program Gabriel Sykes originally wrote in Python, from now on the old Python version is deprecated, and this is the one that will be updated. The old Python code is available [here](https://github.com/sykesgabri/databendingutils).

## How to install:

Precompiled binaries and installers for x86_64 Windows and Linux, and Apple Silicon (aarch64) MacOS are available in this repo's [releases](https://github.com/1009Media/databendingutils-rs/releases).

**IMPORTANT:**
I strongly recommend using the versions of the program with "_PORTABLE" at the end of the filename. The ones not marked as portable will fully install the program to your machine, while the portable ones just run straight off the file, and you can easily move, delete, and replace them.
As the dev I personally don't feel like this program warrants a full install to the user's OS, which is why I recommend the portable. If you still want a full install, they're there in the releases: choose either exe or msi for Windows, dmg for MacOS, and deb or appimage for Linux.

Windows (x86_64):

- Download `DatabendingUtils-RS_X-X-X_x64_PORTABLE.exe` from the releases page, and double click it.
- If Windows Defender gives you a warning prompt, click `More Info` and `Run Anyway`.

Linux (x86_64):

- Download `DatabendingUtils-RS_X-X-X_x64_PORTABLE` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x DatabendingUtils-RS_X-X-X_x64_PORTABLE` to make the program executable.
- Type `./DatabendingUtils-RS_X-X-X_x64_PORTABLE` to run the program.

MacOS (aarch64):

- Download `DatabendingUtils-RS_X-X-X_aarch64_PORTABLE.app` from the releases page, and double click it.
- You'll probably get a warning about an untrusted developer, forcing you to go into privacy settings to allow the program to run, [this is Apple trying to extort developers by making them pay to remove this message](https://developer.apple.com/support/compare-memberships/), fuck the cunts.

## How to use:

Refer to this repo's [wiki](https://github.com/1009Media/databendingutils-rs/wiki) for detailed instructions on usage of the program itself, as well as how to databend.

## Building from source:

If you need the program on a platform not in the releases, such as 32 bit Windows or MacOS, follow these instructions:

- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Open a terminal or command prompt and type `git clone https://github.com/1009Media/databendingutils-rs`.
- When that finishes, type `cd databendingutils-rs`.
- Now type `cargo build --release`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". The compiled binary will be in there, and it will be named "databendingutils-rs" or "databendingutils-rs.exe" depending on the platform you compiled it on. You can move this binary wherever you want, and can run it the same way you would run the precompiled releases.

## CONTRIBUTORS:

- Gabriel Sykes

---
This project is licensed under the GNU GPL V3.
