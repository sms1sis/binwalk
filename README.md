# Binwalk v3 (aarch64 Android Compatible)

This is an updated version of the Binwalk firmware analysis tool, re-written in Rust for speed and accuracy. This specific fork has been patched to compile successfully on `aarch64` Android environments.

![binwalk v3](images/binwalk_animated.svg)

## What does it do?

Binwalk can identify, and optionally extract, files and data that have been embedded inside of other files.

While its primary focus is firmware analysis, it supports a [wide variety](https://github.com/ReFirmLabs/binwalk/wiki/Supported-Signatures) of file and data types.

**Note on Entropy Analysis:** The original Binwalk v3 uses `plotly` for graphical entropy analysis. To enable compilation on `aarch64` Android, the `plotly` dependency and all related entropy graph generation code have been removed. Therefore, this patched version **does not support generating entropy graphs**. The underlying entropy calculation logic is also removed as it was only used for plotting.

Binwalk can be customized and [integrated](https://github.com/ReFirmLabs/binwalk/wiki/Using-the-Rust-Library) into your own Rust projects.

## How do I get it?

The easiest way to install Binwalk and all dependencies is to [build a Docker image](https://github.com/ReFirmLabs/binwalk/wiki/Building-A-Binwalk-Docker-Image).

Binwalk can also be [installed](https://github.com/ReFirmLabs/binwalk/wiki/Cargo-Installation) via the Rust package manager.

Or, you can [compile from source](https://github.com/ReFirmLabs/binwalk/wiki/Compile-From-Source)!

### Compiling for aarch64 Android (with this patch)

To compile this patched version for `aarch64` Android, ensure you have Rust and Cargo installed for your target. Then, navigate to the `binwalk` directory and run:

```bash
cargo build --release
```

This patched version addresses compilation issues related to the `plotly` dependency on `aarch64` Android by removing the entropy plotting feature.

## How do I use it?

Usage is _**simple**_, analysis is _**fast**_, and results are _**detailed**_:

```bash
binwalk DIR-890L_AxFW110b07.bin
```
![example output](images/output.png)

Use `--help`, or check out the [Wiki](https://github.com/ReFirmLabs/binwalk/wiki#usage) for more advanced options!