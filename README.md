# AMOCA Climate - Solana Anchor Program

AMOCA Climate is a Solana smart contract (program) built using the Anchor framework. It is designed to support on-chain climate analytics, data tracking, and decentralized applications related to climate initiatives.

## Features

- On-chain storage and management of climate-related data
- Anchor-based program structure for safety and productivity
- Extensible for various climate analytics and incentive mechanisms

## Prerequisites

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/chapter_2/installation.html)
- Node.js (for JavaScript/TypeScript client development)
- Rust (for Solana program development)

## Getting Started

1. **Clone the repository:**

   ```sh
   git clone <repo-url>
   cd amoca-climate-solana-program
   ```

2. **Install dependencies:**

   ```sh
   anchor install
   ```

3. **Build the program:**

   ```sh
   anchor build
   ```

4. **Run tests:**

   ```sh
   anchor test
   ```

## Directory Structure

- `programs/` - Rust source code for the Solana program
- `tests/` - Integration tests using Anchor's Mocha framework
- `migrations/` - Anchor deployment scripts

## Deployment

To deploy the program to a localnet or devnet:

```sh
anchor deploy
```

Update your client with the new program ID as needed.

## Contributing

Pull requests and issues are welcome. Please open an issue to discuss your ideas or report bugs.

## License

MIT License
