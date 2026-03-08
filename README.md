# Solana Vanity Miner - Open Source Standalone

A high-performance, multi-threaded CPU miner for generating Solana vanity addresses using native `ed25519-dalek` cryptography.

This repository is provided as a **Zero-Trust** alternative to our cloud service. If you prefer to generate your vanity keys on your own hardware without relying on the cloud, you can compile and run this CLI tool yourself.

## Features
- **Stateless & Zero-Trust**: No databases, no API calls, no network requests.
- **Lightning Fast**: Multi-threaded architecture utilizing 100% of your CPU cores.
- **Pure Rust**: Built using Solana's underlying cryptographic standard (`ed25519-dalek`).
- **Flexible Search**: Support for exact case targeting and prefix/suffix placement.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Ensure `cargo` is installed)

## Quick Start (Installation)
Clone the repository and compile the highly-optimized release binary.

```bash
git clone https://github.com/bavtika/solana-vanity-miner.git
cd solana-vanity-miner
cargo build --release
```

## Usage

After compiling, the binary will be located at `./target/release/solana-vanity-miner`.

### Basic Search (Prefix)
Finds any address starting with `pump` (case-insensitive by default).
```bash
cargo run --release -- --pattern pump --strict false
```

### Exact Case (Strict Mode)
Finds an address starting with exactly `PUMP`.
```bash
cargo run --release -- --pattern PUMP --strict true
```

### Suffix Search
Finds an address ending with `mint`.
```bash
cargo run --release -- --pattern mint --prefix false
```

### Advanced: Thread Control
By default, the miner will use all available logical CPU cores. You can throttle this by specifying a thread count.
```bash
cargo run --release -- --pattern PUMP --threads 4
```

## Understanding the Output
As the miner runs, it will print a live Hashrate (Keys checked per second).

When a match is found, the miner **stops immediately** and prints the unencrypted private key directly to your terminal screen:

```
🚀 Solana Vanity Miner Standalone
=============================
Pattern: PUMP
Position: Prefix
Case Sensitive: true
Threads: 16
=============================

Mining started. Press Ctrl+C to abort...

Hashrate: 245000 H/s

🎉 MATCH FOUND!
=============================
Public Key (Address): PUMPSqXRZwq5RxyQYx8j2TzG1K9B4pNX3M6Y
Private Key (Base58): 4qk...[Your Private Key Here]...
=============================

⚠️ KEEP YOUR PRIVATE KEY SAFE. NEVER SHARE IT WITH ANYONE.
```

## Security Disclosure
When running this tool, your generated private key never leaves your machine. It is printed only to `localhost` standard output. We strongly recommend running this directly on an air-gapped machine if you are generating high-value institutional addresses.
