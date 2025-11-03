# Stellink 🔗

## Project Title
**Stellink: Decentralized URL Shortener on Stellar Blockchain**

---

## Project Description

Stellink is a fully decentralized URL shortening service built on the Stellar blockchain using Soroban smart contracts. Unlike traditional URL shorteners (like bit.ly or TinyURL), Stellink stores link mappings directly on-chain, making them permanent, censorship-resistant, and user-owned.

The smart contract enables users to:
- Create short, memorable links with custom aliases or auto-generated keys
- Own and control their links through their Stellar wallet
- Update or delete links they own
- Query link destinations without any centralized intermediary

All link data is stored immutably on the Stellar ledger, ensuring that once created, links cannot be hijacked, censored, or deleted by any third party. Only the wallet that created a link has the authority to modify or remove it.

---

## Project Vision

Our vision is to create a **censorship-resistant infrastructure for the decentralized web** where:

1. **No Single Point of Failure**: Links persist as long as the Stellar network exists, eliminating "link rot" caused by centralized services shutting down.

2. **User Sovereignty**: Users maintain complete ownership and control over their links through cryptographic authentication.

3. **Transparency & Trust**: All link mappings are publicly verifiable on-chain, preventing malicious redirects and building trust.

4. **Permissionless Innovation**: Anyone can build their own resolver front-end or integrate StellarLink into their applications without requiring permission.

5. **Global Accessibility**: Leverage Stellar's fast, low-cost network to make decentralized link shortening accessible to everyone worldwide.

Stellink aims to become the standard for permanent, trustless link shortening in the Web3 ecosystem, serving activists, content creators, developers, and anyone who values digital permanence and freedom from censorship.

---

## Key Features

### Core Functionality
- **🔐 Wallet-Based Authentication**: Links are owned by Stellar addresses and protected by cryptographic signatures
- **✨ Custom Aliases**: Choose your own memorable short keys or let the system auto-generate them
- **📝 Full CRUD Operations**: Create, read, update, and delete links with proper ownership validation
- **🌐 Public Resolution**: Anyone can resolve a short key to its destination URL without authentication
- **⚡ Fast & Efficient**: Leverages Stellar's 5-second finality for quick link creation
- **💰 Low Cost**: Minimal transaction fees make it economical to create and manage links

### Security & Ownership
- **Owner Verification**: Only the wallet that created a link can modify or delete it
- **Duplicate Prevention**: System prevents key collisions and ensures unique short keys
- **On-Chain Storage**: All data stored in Soroban persistent storage with extended TTL (1 year+)
- **Input Validation**: Comprehensive checks for empty URLs, invalid keys, and malformed inputs

### Technical Features
- **Pure Rust Implementation**: Built with Soroban SDK for optimal performance and security
- **No External Dependencies**: Self-contained smart contract with no oracle or off-chain requirements
- **Map-Based Storage**: Efficient key-value storage using Soroban's Map data structure
- **Comprehensive Testing**: Full test suite covering all operations and edge cases
- **Gas Optimized**: Efficient storage patterns to minimize transaction costs

---

## Future Scope

### Phase 1: Enhanced Link Management
- **Link Analytics** (Optional, Off-Chain): Privacy-preserving click tracking for users who opt-in
- **Link Expiration**: Time-locked links that automatically expire after a set period
- **Link Categories/Tags**: Organize links with on-chain metadata
- **Batch Operations**: Create or update multiple links in a single transaction

### Phase 2: Anti-Spam & Economic Mechanisms
- **Deposit System**: Require a refundable XLM deposit to create links, disincentivizing spam
- **Reputation System**: On-chain reputation scores for link creators based on usage and longevity
- **Premium Keys**: Auction system for highly desirable short keys (1-3 characters)
- **Storage Cleanup Incentives**: Reward users for deleting unused links to free up storage

### Phase 3: Advanced Features
- **IPFS Integration**: Store destination content on IPFS and link to CIDs for fully decentralized content hosting
- **NFT-Based Links**: Create links as Stellar assets/NFTs that can be traded or transferred
- **Multi-Sig Links**: Links owned by multiple addresses requiring consensus for updates
- **Domain Integration**: Support for custom domains pointing to the contract (e.g., `mysite.link/abc123`)

### Phase 4: Governance & Ecosystem
- **DAO Governance**: Decentralized governance for contract upgrades and parameter adjustments
- **Developer SDK**: JavaScript/TypeScript SDK for easy integration into dApps
- **Resolver Network**: Incentivize running resolver nodes that cache and serve links
- **Cross-Chain Bridges**: Enable link creation from other blockchain networks

### Phase 5: Social & Collaborative Features
- **Link Sharing Permissions**: Allow multiple users to manage the same link
- **Link Collections**: Create curated collections of links as on-chain portfolios
- **QR Code Generation**: On-chain QR code metadata for easy mobile sharing
- **Link Monetization**: Optional tipping or pay-per-click mechanisms for content creators

---

## Contract Information

### Deployment Details

| Parameter | Value |
|-----------|-------|
| **Contract ID** | CDK3WKSU2GADDFKCVTYEAOI6GYAJZELJ27WGVJW3AZLXETSUOMYUIUYW|
| **Network** | Stellar Testnet  |
| **Admin Address** |GC424DQWKSPANHBI7E5IG2MFLMSFPRKOMFLHV3AA3YKDJUHU5TWD4G6C |
<img width="2877" height="1439" alt="image" src="https://github.com/user-attachments/assets/238a7cf3-8fa8-473d-a834-b2c0e3a9c459" />




## Technical Architecture

### Smart Contract Structure

```rust
pub struct LinkRecord {
    pub destination_url: String,   // The full URL to redirect to
    pub created_ledger: u32,        // Ledger sequence when created
    pub owner: Address,             // Stellar address of the owner
}
```

### Storage Design

The contract uses two primary storage maps:
- **LINKS**: `Map<String, LinkRecord>` - Maps short keys to full link records
- **OWNERS**: `Map<String, Address>` - Maps short keys to owner addresses for quick ownership lookups

### Key Generation Algorithm

Auto-generated keys use a base62 encoding (a-z, A-Z, 0-9) of the combined ledger sequence and timestamp, producing 7-character unique identifiers with ~3.5 trillion possible combinations.

---

## Getting Started

### Prerequisites

- Rust 1.70+
- Soroban CLI
- Stellar account with XLM for transaction fees

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/stellink
   cd stellink
   ```

2. **Build the contract:**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

### Deployment

1. **Deploy to Stellar Testnet:**
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/url_shortener.wasm \
     --source YOUR_SECRET_KEY \
     --rpc-url https://soroban-testnet.stellar.org \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

2. **Save the Contract ID** returned from the deployment command.

### Usage Examples

#### Create a Link with Custom Key

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_link \
  --owner YOUR_ADDRESS \
  --long_url "https://example.com/very/long/url/path" \
  --custom_key "mylink"
```

#### Create a Link with Auto-Generated Key

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_link \
  --owner YOUR_ADDRESS \
  --long_url "https://example.com/another/url"
```

#### Resolve a Link (Read-Only)

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_url \
  --short_key "mylink"
```

#### Update a Link

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- update_link \
  --owner YOUR_ADDRESS \
  --short_key "mylink" \
  --new_long_url "https://example.com/updated/url"
```

#### Delete a Link

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- delete_link \
  --owner YOUR_ADDRESS \
  --short_key "mylink"
```

---

## API Reference

### Contract Functions

#### `create_link(owner: Address, long_url: String, custom_key: Option<String>) -> String`
Creates a new short link and returns the short key.

**Parameters:**
- `owner`: The Stellar address that will own the link
- `long_url`: The destination URL (must not be empty)
- `custom_key`: Optional custom alias (1-64 characters, or None for auto-generation)

**Returns:** The short key (either custom or generated)

**Panics:** 
- "URL cannot be empty"
- "Custom key must be between 1 and 64 characters"
- "Link key already exists"

---

#### `update_link(owner: Address, short_key: String, new_long_url: String)`
Updates the destination URL of an existing link.

**Parameters:**
- `owner`: The Stellar address that owns the link
- `short_key`: The short key to update
- `new_long_url`: The new destination URL

**Panics:**
- "URL cannot be empty"
- "Link not found"
- "Not the owner of this link"

---

#### `delete_link(owner: Address, short_key: String)`
Permanently deletes a link.

**Parameters:**
- `owner`: The Stellar address that owns the link
- `short_key`: The short key to delete

**Panics:**
- "Link not found"
- "Not the owner of this link"

---

#### `get_url(short_key: String) -> String`
Retrieves the destination URL for a short key (public, no authentication required).

**Parameters:**
- `short_key`: The short key to resolve

**Returns:** The destination URL

**Panics:** "Link not found"

---

#### `get_link(short_key: String) -> LinkRecord`
Retrieves the full link record including metadata.

**Parameters:**
- `short_key`: The short key to query

**Returns:** Complete LinkRecord struct

**Panics:** "Link not found"

---

#### `get_owner(short_key: String) -> Address`
Retrieves the owner address of a link.

**Parameters:**
- `short_key`: The short key to query

**Returns:** Owner's Stellar address

**Panics:** "Link not found"

---

## Development

### Project Structure

```
stellink/
├── src/
│   ├── lib.rs           # Main contract implementation
│   └── test.rs          # Comprehensive test suite
├── Cargo.toml           # Dependencies and metadata
└── README.md            # This file
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_create_link_with_custom_key
```

### Building for Production

```bash
# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM (optional)
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/url_shortener.wasm
```

---

## Security Considerations

### Auditing Status
⚠️ **This contract has not been formally audited.** Use at your own risk, especially on mainnet.

### Known Limitations
1. **Key Generation**: The pseudo-random key generation uses ledger sequence and timestamp, which are predictable. For high-security applications, consider requiring custom keys only.
2. **Storage Costs**: Each link consumes persistent storage. Consider implementing cleanup mechanisms for production.
3. **No Link Validation**: The contract does not validate URL formats or check for malicious destinations.
4. **Rate Limiting**: No built-in rate limiting; spam prevention must be handled at the application layer or through deposit mechanisms.

### Best Practices
- Always test on testnet before mainnet deployment
- Monitor storage usage and extend TTL as needed
- Implement front-end validation before contract calls
- Use hardware wallets for mainnet deployments
- Keep backup records of important links off-chain

---

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Write tests for all new features
- Follow Rust naming conventions
- Document public functions
- Keep gas costs optimized
- Update README for breaking changes

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- **Stellar Development Foundation** for the Soroban smart contract platform
- **Soroban Community** for documentation and support
- **Open Source Contributors** who inspire decentralized innovation

---

## Contact & Support

- **Project Repository**: [GitHub Repository URL]
- **Documentation**: [Documentation URL]
- **Discord**: [Discord Server URL]
- **Twitter**: [@Stellink]
- **Email**: support@stellink.xyz

---

## Disclaimer

Stellink is experimental software. While we strive for security and reliability, we make no guarantees. Users are responsible for:
- Securing their private keys
- Validating link destinations before clicking
- Understanding transaction costs
- Complying with local laws regarding content linking

**Use at your own risk.**

---

*Built with ❤️ on Stellar*
