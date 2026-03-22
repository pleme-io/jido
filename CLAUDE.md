# jido

Execute multi-step Android workflows from YAML or natural language. Install, grant, navigate, screenshot, analyze. Consumes AdbTransport, ArchiveReader traits.

## Build

```bash
nix build
nix run .#jido
cargo build
```

## Architecture

- Binary: `jido`
- Language: Rust (edition 2024, rust-version 1.91.0)
- License: MIT
- Nix: substrate `rust-tool-release-flake.nix` pattern
