# Wormhole Name Service (WNS) - Solana Implementation

## Overview

The Wormhole Name Service (WNS) is a cross-chain naming service allowing users to manage decentralized identities across multiple blockchain ecosystems. This implementation on Solana includes:
- **Cross-chain messaging** using Wormhole to synchronize names across chains.
- **SNS integration** allowing `.sol` domain holders to receive a corresponding `.wns` domain.
- **Automatic renewal** and domain registration fees set to `0.2 SOL` per year.

## Features
- **Domain Registration**: Register `.wns` domains.
- **Automatic Registration for SNS `.sol` Holders**: Owners of `.sol` domains from SNS receive free `.wns` domains.
- **Cross-Chain Messaging**: Send domain data across chains using Wormhole.
- **Fees and Renewal**: Domains can be renewed at `0.2 SOL` per year.

## Project Structure

```bash
wns-solana/
├── programs/
│   └── wns/
│       ├── Cargo.toml                # Program dependencies
│       └── src/
│           ├── lib.rs                # Program entry point
│           ├── instruction.rs        # Instruction logic
│           ├── state.rs              # State definitions
│           ├── error.rs              # Error handling
│           ├── sns_integration.rs    # SNS integration logic
│           └── cross_chain.rs        # Cross-chain messaging with Wormhole
├── migrations/
│   └── deploy.ts                     # Deployment script
├── tests/
│   └── wns.ts                        # Integration tests
├── Anchor.toml                       # Anchor configuration file
├── Cargo.toml                        # Rust dependencies
└── README.md                         # Project documentation
```

## Installation and Setup

### Prerequisites
- **Rust and Anchor CLI**: Install Rust and Anchor CLI.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
    ```
- **Solana CLI**: Install Solana CLI.
    ```bash
    sh -c "$(curl -sSfL https://release.solana.com/v1.8.0/install)"
    ```

### Build and Deploy
1. **Build the Project**:
    ```bash
    anchor build
    ```
2. **Deploy to Local Validator**:
    ```bash
    solana-test-validator &
    anchor deploy
    ```

3. **Set up Environment for Testnet Deployment**:
    ```bash
    solana config set --url https://api.devnet.solana.com
    anchor deploy
    ```

### Running Tests
Run the tests using:
```bash
anchor test
```

## Usage Examples
1. **Register a New WNS Domain**:
   ```typescript
   await program.methods.registerName("alice.wns", ownerPubkey).rpc();
   ```

2. **Automatically Register a WNS Domain from an Existing SNS Domain**:
   ```typescript
   await program.methods.registerWnsFromSns("alice.sol").rpc();
   ```

---

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support
For questions or issues, please reach out via our [Telegram](https://t.me/Tom_Tom29).
```
