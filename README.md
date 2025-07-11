# ErcNftMetadataAnalyzer

## Description



## Features

- Retrieves and parses ERC-721 and ERC-1155 metadata from on-chain storage or IPFS.
- Validates the integrity of NFT metadata against a user-defined schema using JSON Schema validation.
- Generates statistical reports on NFT collections, including attribute rarity and distribution.
- Detects and flags potential metadata anomalies, such as broken image links or inconsistent attribute types.
- Exports analysis results in CSV, JSON, and human-readable formats for further processing.
- Integrates with decentralized storage solutions like Arweave for permanent metadata archiving.
- Calculates a rarity score for each NFT based on its attributes and their prevalence within the collection.
## Installation

```bash
cargo add ercnftmetadataanalyzer
```

## Usage

```rust
use ercnftmetadataanalyzer::run;

fn main() {
    run(false).expect("Execution failed");
}
```

## Contributing

We welcome contributions! Here's how to get started:

1. Fork this repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some awesome feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
