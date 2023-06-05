# Rusty-Win-Driver
The Rusty-Win-Driver is a simple windows driver that written on the Rust lang.

## Build
1. Install the [Rust](https://www.rust-lang.org/tools/install)
2. Install the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/)
3. Install the [WDK](https://docs.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
4. Install cargo-make: `cargo install cargo-make`
5. Set the environment variable ```WIN_SDK_TOOLS``` to the path to the Windows SDK tools. For example: ```C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64```
6. Run the ```cargo make deploy``` command
7. Move the ```Rusty-Win-Driver.sys``` to virtual machine and run.
