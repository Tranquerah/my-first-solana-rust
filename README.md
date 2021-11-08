# My First Solana + Rust Program

## Devnet config
```
solana config set --url devnet

// Make sure you're on devnet.
solana config get

anchor build

// Get the new program id.
solana address -k target/deploy/myepicproject-keypair.json

// Update Anchor.toml and lib.rs w/ new program id.
// Make sure Anchor.toml is on devnet.

// Build again.
anchor build
```

## READ!

**[Solana-Web3.js](https://solana-labs.github.io/solana-web3.js/modules.html#Commitment)**

**[Solana Docs](https://docs.solana.com/developing/programming-model/overview)**

**[Rust-Lang Docs](https://doc.rust-lang.org/book/ch01-01-installation.html)**

**[Anchor Docs](https://project-serum.github.io/anchor/getting-started/projects.html)**