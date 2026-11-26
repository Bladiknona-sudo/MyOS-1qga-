# MyOS

MyOS is an experimental x86_64 operating-system project written in Rust.

## Current target

The first milestone is a bootable BIOS/UEFI hybrid ISO using the Limine boot protocol. The initial kernel is freestanding (no_std) and runs directly on the machine; it is not a Linux distribution or a desktop environment.

## Build

A reproducible build will be provided by GitHub Actions. Successful builds will publish the ISO and SHA-256 checksum as workflow artifacts.

## Status

Early development. A successful compilation is not considered proof of a working OS: the ISO must also pass boot-image checks and a QEMU boot smoke test. Hardware support and a complete desktop environment are not yet implemented.

## Architecture

- x86_64 kernel in Rust
- Limine boot protocol
- BIOS and UEFI boot paths
- GitHub Actions CI and ISO packaging

## License

MIT
