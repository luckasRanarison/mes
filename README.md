<div align="center">
  <img width="150" src="./web/src/assets/nes-controller.svg" alt="logo"/>
</div>

# Mes

A decent multiplatform NES emulator built using Rust. Try it [now](https://luckasranarison.github.io/mes/) in your browser.

## Contents
- [Supported platforms](#supported-platforms)
- [Features](#features)
- [Mappers](#mappers)
- [Build](#build)
- [Resources](#resources)

## Supported platforms

- [x] Web
- [x] Android
- [ ] Desktop
- [ ] Embedded (ESP32)

## Features

- Supports [iNES 1.0](https://www.nesdev.org/wiki/INES) file format
- Supports basic [mappers](#mappers)
- Fairly decent audio quality
- Implements some of the original hardware quirks

## Mappers

- [NROM](https://nesdir.github.io/mapper0.html) (0)
- [SXROM](https://nesdir.github.io/mapper1.html) (1)
- [UXROM](https://nesdir.github.io/mapper2.html) (2)
- [CNROM](https://nesdir.github.io/mapper2.html) (3)

## Build

> [!IMPORTANT]
> The Rust [toolchain](https://rustup.rs/) is required to build the main library.

### Web

![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/vite-%23646CFF.svg?style=for-the-badge&logo=vite&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=fff&style=for-the-badge)

**Requirements**:

- [NodeJS](https://nodejs.org/en)
- [wasmpack](https://rustwasm.github.io/wasm-pack/)

**Scripts**:

```bash
npm run wasm # build the WASM artifacts using wasmpack
npm run dev # run the dev server
npm run build # build the website
```

### Android

![Kotlin](https://img.shields.io/badge/kotlin-%237F52FF.svg?style=for-the-badge&logo=kotlin&logoColor=white)

**Requirements**:

- [Android studio](https://developer.android.com/studio)
- [NDK](https://developer.android.com/ndk)
- `aarch64-linux-android` and `x86_64-linux-android` Rust targets

**Setup**:

Edit your global cargo config in `~/.cargo/cargo.toml` and use linkers from NDK:

```toml
[target.aarch64-linux-android]
linker = "your-ndk-pah/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

[target.x86_64-linux-android]
linker = "your-ndk-pah/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android34-clang"
```

**Gradle scripts**:

- `buildRustArm64`: Build the shared library for arm64
- `buildRustx86_64`: Build the shared library for x86_64
- `buildRs`: Runs both

## Resources

This project wouldn't have been possible without the help of the following ressources:

- [nesdev.org](https://www.nesdev.org/): Covers everything needed to build a NES emulator.
- [6502 instruction set reference](https://www.masswerk.at/6502/6502_instruction_set.html): A detailed reference for the MOS6502 CPU.
- [Displaced Gamers](https://www.youtube.com/@DisplacedGamers): Has a lot of interesting technical video about the NES.
- [javidx9's NES emulator series](https://www.youtube.com/playlist?list=PLrOv9FMX8xJHqMvSGB_9G9nZZ_4IgteYf): Guided me at the beginning of my journey.
