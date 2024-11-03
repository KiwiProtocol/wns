# Wormhole Name Service (WNS) - Sui Implementation

## Overview

The Wormhole Name Service (WNS) on Sui is a decentralized naming system that allows users to register, renew, and manage domain names across multiple blockchains using the Wormhole cross-chain messaging protocol. This implementation is designed for the Sui blockchain and includes:
- **Cross-chain interoperability** with Wormhole, enabling communication with other blockchain networks.
- **Automatic .wns domain assignment** for users with existing .sui domains on SuiNS, allowing `.sui` domain holders to receive a free, corresponding `.wns` domain.
- **Domain registration and renewal fees** set at 0.2 SUI per year.

## Features

- **Domain Registration**: Users can register `.wns` domains with ownership on the Sui blockchain.
- **Automatic .sui to .wns Mapping**: If a user has a `.sui` domain in SuiNS, they automatically receive a corresponding `.wns` domain.
- **Cross-Chain Messaging**: Use Wormhole to pass WNS data across blockchains.
- **Registration and Renewal Fees**: Fixed annual fee for `.wns` domains.

## Project Structure

```plaintext
wns-sui/
├── Move.toml                     # Project metadata and dependencies
├── sources/
│   ├── WNS.move                  # Main contract defining WNS logic
│   ├── Registry.move             # Manages domain registration and ownership
│   ├── CrossChain.move           # Wormhole cross-chain messaging functions
│   ├── Fees.move                 # Domain registration and renewal fees logic
│   ├── SuiNSIntegration.move     # SuiNS integration for .sui to .wns mapping
├── scripts/
│   └── deploy.sh                 # Deployment script for the Sui network
└── tests/
    ├── wns.move                  # Test script for WNS functionality
    └── CrossChainTests.move      # Test script for cross-chain interactions
```

### Contract Descriptions

- **`WNS.move`**: Core module implementing WNS functionality, including domain registration, renewal, and resolution.
- **`Registry.move`**: Manages domain ownership, expiration, and registry lookup.
- **`CrossChain.move`**: Implements cross-chain message passing using Wormhole.
- **`Fees.move`**: Sets and collects fees for domain registration and renewal.
- **`SuiNSIntegration.move`**: Integrates with SuiNS to check for `.sui` domains and assign free `.wns` domains if an equivalent `.sui` domain exists.

---

## Installation and Setup

### Prerequisites

- **Sui CLI and Sui Move**: Install the Sui CLI to interact with the Sui blockchain and build the Move package.
    ```bash
    curl -fsSL https://get.sui.io | sh
    ```

### Clone the Repository

```bash
git clone https://github.com/your-org/wns-sui.git
cd wns-sui
```

### Install Dependencies

Make sure to specify dependencies in `Move.toml`:
```toml
[dependencies]
Wormhole = { git = "https://github.com/wormhole-foundation/wormhole-sui.git", rev = "latest" }
Sui = { git = "https://github.com/MystenLabs/sui.git", rev = "latest" }
SuiNS = { git = "https://github.com/MystenLabs/suins.git", rev = "latest" }
```

---

## Usage

### 1. Build the Move Package

To build the package, use:
```bash
sui move build
```

### 2. Deploy to Sui Network

Use the provided deployment script to deploy the WNS package to the Sui network:

```bash
./scripts/deploy.sh
```

Alternatively, you can deploy manually with:
```bash
sui move publish --gas-budget 10000000
```

### 3. Register a Domain

After deploying, you can interact with the WNS to register a new domain:

```typescript
await program.methods.register_domain("example.wns", owner, 365).rpc();
```

### 4. Assign a `.wns` Domain Based on Existing `.sui` Domain

If a user has a `.sui` domain in SuiNS, they can receive a free `.wns` domain:
```typescript
await program.methods.assign_free_wns_if_sui_exists(suins, "example.sui").rpc();
```

This will check for the `.sui` domain in SuiNS, and if it exists, a corresponding `.wns` domain will be assigned automatically.

---

## Testing

### Run Tests

You can run the provided test scripts in `tests/wns.move` and `tests/CrossChainTests.move` to validate the functionality.

1. **Compile and Run Tests**
    ```bash
    sui move test
    ```

2. **Example Test Cases**
   - **Register a New `.wns` Domain**: Validates domain ownership and fee deduction.
   - **Renew Domain**: Extends the expiration for an existing `.wns` domain.
   - **Assign Free `.wns` Domain for `.sui` Name**: Checks that `.wns` domains are automatically created for users with existing `.sui` domains.
   - **Cross-Chain Messaging**: Ensures Wormhole cross-chain messages can be sent with WNS data.

### Testing Notes

- **Environment Setup**: Ensure `Move.toml` and dependency modules are correctly configured.
- **Gas Requirements**: Set an appropriate gas budget in the deployment script or test environment.

---

## Example Code Snippets

### Registering and Resolving Domains

```move
module WNS::WNS {
    public fun register_domain(
        wns: &mut WNS,
        domain_name: vector<u8>,
        owner: address,
        duration: u64
    ): bool {
        Fees::charge_registration_fee(owner);
        Registry::register(wns, domain_name, owner, duration);
        true
    }

    public fun resolve_domain(
        wns: &WNS,
        domain_name: vector<u8>
    ): address {
        Registry::resolve(wns, domain_name)
    }
}
```

### Cross-Chain Messaging Example

Using the Wormhole integration for cross-chain communication:

```move
module WNS::CrossChain {
    public fun post_cross_chain_message(domain_name: vector<u8>, recipient_chain: u8, recipient_address: address): bool {
        Wormhole::core::post_message(domain_name, recipient_chain, recipient_address);
        true
    }
}
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For support, feel free to contact us on our official [Telegram](https://t.me/Tom_Tom29).

---

This README provides a full overview of the Wormhole Name Service (WNS) on Sui, including setup instructions, usage examples, and testing guidelines. By integrating SuiNS, we ensure that users with `.sui` domains receive corresponding `.wns` domains, creating a unified and cross-chain compatible naming system.