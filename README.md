# WBA Solana Q2 2024

TypeScript utilities for working with the Solana blockchain.

## Components

1. **keygen.ts**: Generates a new Solana wallet.
2. **airdrop.ts**: Requests an airdrop of SOL tokens to a specified wallet on the Solana Devnet.
3. **transfer.ts**: Transfers SOL between two wallets.
4. **enroll.ts**: Handles enrollment operations via a smart contract using the Anchor framework.
5. **main.rs**: Rust program for Base58 encoding/decoding.
6. **wba_prereq.ts**: Defines types and interfaces for a specific smart contract using Anchor.

### How to Use

#### Setup

Install all required Node.js packages by running:

```bash
npm install
```

#### Keygen.ts

- **Purpose**: Generates a new keypair for a Solana wallet.
- **Command**:

```bash
npm run keygen
```

#### Airdrop.ts

- **Purpose**: Obtains 2 SOL tokens from the Devnet's faucet to the specified wallet.
- **Command**:

```bash
npm run airdrop
```

#### Transfer.ts

- **Purpose**: Transfers a fraction of SOL to another wallet.
- **Command**:

```bash
npm run transfer
```

#### Enroll.ts

- **Purpose**: Performs an enrollment operation via a smart contract(program) interaction.
- **Command**:

```bash
npm run enroll
```

#### Main.rs (base58_converter/src/main.rs)

- **Purpose**: Demonstrates Base58 encoding and decoding.
- **Usage**: Replace `"YOUR_PK_HERE"` with your actual private key string.

```bash
cargo run
```

#### WBA Prereq (programs/wba_prereq.ts)

- **File**: `wba_prereq.ts`
- **Description**: Contains the interface and types for the `wba_prereq` smart contract.
- **Usage**: Utilized by the `enroll.ts` for contract interaction.

### Configuration

Ensure your `dev-wallet.json` (new wallet created using keygen.ts) and `wba-wallet.json` (exported wallet) contain the correct private keys for interaction with the blockchain.

### Running the Scripts

Scripts can be executed through npm scripts defined in `package.json`. Navigate to the directory containing your scripts and run:

```bash
npm run <script-name>
```

### Security

- **DO NOT** share your private keys (`dev-wallet.json`, `wba-wallet.json`) with anyone.
