# solana-pfp-program

## Description

This repository contains the Rust implementation of the Solana PFP protocol.

The Solana PFP protocol allows Solana users to set a single Metaplex-standard NFT as a universal PFP for the Solana blockchain.

## Instructions

### SetProfileNFT
**Instruction: 0** - Allows users to set a single Metaplex-standard NFT as their PFP

#### Account keys
0. `[signer]` The account of the person setting the NFT profile picture
1. `[writable]` NFT profile picture account (PDA)
2. `[]` NFT mint account
3. `[]` NFT token account
4. `[]` NFT Metaplex metadata account
5. `[]` Clock sysvar
6. `[]` System program

### UnsetProfileNFT
**Instruction: 1** - Allows users to remove their PFP

#### Account keys
0. `[signer]` The account of the person removing the NFT profile picture
1. `[writable]` NFT profile picture account (PDA)

## Development
If you want to add new features to the protocol or improve existing ones, please follow the steps below.

### Environment Setup
- Install the latest Rust stable from https://rustup.rs/
- Install Solana v1.8.11 or later from https://docs.solana.com/cli/install-solana-cli-tools
- Install the `libudev` development package for your distribution (`libudev-dev` on Debian-derived distros, `libudev-devel` on Redhat-derived).

### Build
To compile the PFP rust code to the wasm binary, use:
```
cargo build-bpf
```

### Deployment
To deploy the compiled wasm binary, use:
```
solana program deploy ./target/deploy/solana_pfp_program.so
```
