# ICP Token Wallet

A Rust-based token wallet implementation for the Internet Computer Protocol (ICP) blockchain that supports basic IRCRC2 token operations.

## Features
- Send tokens to other addresses
- Receive tokens and update balances
- Query wallet balance
- Basic security implementation using caller verification

## Prerequisites
- Rust (latest stable)
- DFX SDK version 0.8.4
- Internet Computer CLI

## Setup
1. Install the Internet Computer SDK:
```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

2. Install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Add WebAssembly target:
rustup target add wasm32-unknown-unknown

Structure
icp_token_wallet/
├── Cargo.toml          # Rust dependencies and project configuration
├── dfx.json            # DFX configuration
└── src/
    ├── lib.rs          # Smart contract implementation
    └── icp_token_wallet.did  # Candid interface


    Testing
       cargo test

Usage Examples
Check Balance: dfx canister call icp_token_wallet get_balance
get_balance
Send Tokens: dfx canister call icp_token_wallet send_tokens '("<recipient_address>", <amount>)'
Receive Tokens: dfx canister call icp_token_wallet receive_tokens '("<sender_address>", <amount>)'
