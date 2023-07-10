# Oxide - A simple OS for Miyoo Mini (Plus) consoles

## Development

This project uses a streamlined docker based workflow, making it portable to any environment with docker and it requires just one command to build the whole project.

### Requirements

- Deno
- Docker
- Rust

### How To

You just need to have deno in your path and run `./tools/build.ts` in a terminal.

After this completes you can move the contents of `./build/PAYLOAD` to a fat32 formatted sd card and insert into the miyoo mini (plus).
