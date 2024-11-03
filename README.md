# Wormhole Name Service (WNS)

## Overview

The **Wormhole Name Service (WNS)** is a cross-chain, decentralized naming system that allows users to register, renew, and manage blockchain-based domain names that can be used across multiple networks. WNS provides a unified identity layer that leverages the **Wormhole interoperability protocol** for cross-chain messaging, enabling seamless domain management and resolution across **EVM-compatible chains**, **Solana**, and **Sui**.

WNS aims to simplify blockchain interactions by providing a user-friendly alternative to complex blockchain addresses, supporting interoperability and enhancing the usability of Web3 identities.

## Features

- **Cross-Chain Domain Management**: Register and manage domains across multiple blockchains with the Wormhole protocol.
- **Automatic Domain Mapping**: Provides `.wns` domains for users with existing blockchain-specific domains (e.g., `.sol` for Solana, `.sui` for Sui).
- **Standardized Fees and Renewals**: Domain registration and renewal fees are standardized across networks, improving user experience.
- **Cross-Chain Messaging with Wormhole**: Synchronize domain data across different blockchains.

---

## Project Structure

```plaintext
wns/
├── evm/                             # EVM implementation
│   ├── contracts/                   # EVM smart contracts (Solidity)
│   ├── scripts/                     # EVM deployment and migration scripts
│   ├── tests/                       # EVM test scripts (Mocha/Chai)
│   ├── README.md                    # EVM-specific README
│   ├── hardhat.config.js            # Hardhat configuration for EVM
│   └── package.json                 # NPM dependencies
├── solana/                          # Solana implementation
│   ├── programs/                    # Solana Move programs
│   ├── tests/                       # Solana integration tests (Anchor)
│   ├── migrations/                  # Solana deployment scripts
│   ├── Anchor.toml                  # Anchor configuration file
│   └── README.md                    # Solana-specific README
├── sui/                             # Sui implementation
│   ├── sources/                     # Sui Move sources
│   ├── tests/                       # Sui test scripts
│   ├── scripts/                     # Sui deployment scripts
│   ├── Move.toml                    # Sui Move package configuration
│   └── README.md                    # Sui-specific README
└── README.md                        # Main project README
```

---

## Testing

Each implementation includes unit and integration tests:
- **EVM**: Run with Hardhat using `npx hardhat test`.
- **Solana**: Run with Anchor using `anchor test`.
- **Sui**: Run with Sui Move using `sui move test`.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Resources

- **Wormhole Documentation**: [Wormhole Protocol](https://wormhole.com/)
- **Solana Documentation**: [Solana Developer Docs](https://docs.solana.com/)
- **Sui Documentation**: [Sui Developer Portal](https://docs.sui.io/)
- **EVM Chains**: [Ethereum Developer Portal](https://ethereum.org/en/developers/)

For additional support, contact us on our [Telegram](https://t.me/Tom_Tom29).
