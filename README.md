# Oxide - A simple OS for Miyoo Mini (Plus) consoles

## Development

This project uses a streamlined docker based workflow, making it portable to any environment with docker and it requires just one command to build the whole project.

### Requirements

- Deno
- Rust
- Docker (If using Docker for compilation)


OR

Build with native cross-compilation tools.

### Building

You just need to have deno in your path and run `./tools/build.ts` in a terminal.

After this completes you can move the contents of `./build/PAYLOAD` to a fat32 formatted sd card and insert into the miyoo mini (plus).

#### Cross-compiling Natively

By default, the build script uses Docker. To use a native cross-compilation toolchain, add `-N` or `--native` to `./tools/build.ts`.

Ex: `./tools/build.ts -N`

#### Release

To build for release, add `--release` to the build command.

Ex: `./tools/build.ts --release`

#### Extra Arguments

Any extra arguments for cargo, if it is used by the build method, can be passed after `--`.

Ex: `./tools/build.ts --release -- -Z build-std`
