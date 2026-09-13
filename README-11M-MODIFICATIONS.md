# Nexus 1.10.3 — 11 m / DX cluster modification

This source tree contains the 11 m integration requested for the DX/CB workflow.

## What was changed

- 11 m is recognized throughout the existing band engine as `11m` for 26.000–27.999 MHz.
- The main Nexus band selector now includes a receive-only **11 m · DX 27.555** channel.
- The 11 m channel is explicitly marked `tx=false` so the application does not present 27.555 MHz as an amateur transmit authorization.
- The DX-cluster/spot pipeline already had 11 m frequency classification in the supplied source; the Spots panel and Needed panel now expose **11m** as a normal band filter instead of only showing it when a spot happened to be present.
- Propagation band ordering includes 11 m so existing cluster/propagation data can retain the `11m` identity.
- A Windows build script is included: `BUILD-WINDOWS-11M.bat`.

## Important France note

27.555 MHz is commonly used internationally for 11 m DX, but it is outside the French CB allocation. This build therefore treats the 11 m DX channel as **receive/cluster monitoring only**. Local regulations must be followed for any transmission.

## Building the Windows installer

On Windows, install:

1. Node.js LTS.
2. Rust via rustup with the MSVC toolchain.
3. Microsoft Visual Studio Build Tools / C++ build tools required by Tauri.

Then double-click `BUILD-WINDOWS-11M.bat`.

The script installs the UI dependencies, installs the Tauri CLI if needed, and runs:

`cargo tauri build --manifest-path src-tauri/Cargo.toml --features radio`

The NSIS installer is produced in:

`src-tauri\\target\\release\\bundle\\nsis\\`

## Why there is no precompiled EXE in this archive

The current build environment used to prepare this archive does not contain the Rust/Node Windows toolchain and cannot produce a Windows Tauri executable. The source modifications are complete, but an EXE/installer cannot honestly be claimed to have been compiled here.
