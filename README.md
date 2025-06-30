# Solana HTTP Server

This project is a Rust-based HTTP server that exposes various Solana-related endpoints. It provides functionality to generate keypairs, handle SPL tokens, sign and verify messages, and construct valid on-chain instructions.

## Features

- Generate new Solana keypairs
- Create and mint SPL tokens
- Sign and verify messages using Ed25519
- Send SOL and SPL tokens

## Endpoints

### 1. Generate Keypair
- **Endpoint:** `POST /keypair`
- **Description:** Generates a new Solana keypair.

### 2. Create Token
- **Endpoint:** `POST /token/create`
- **Description:** Creates a new SPL token and initializes the mint.

### 3. Mint Token
- **Endpoint:** `POST /token/mint`
- **Description:** Mints SPL tokens to a specified destination.

### 4. Sign Message
- **Endpoint:** `POST /message/sign`
- **Description:** Signs a message using a private key.

### 5. Verify Message
- **Endpoint:** `POST /message/verify`
- **Description:** Verifies a signed message.

### 6. Send SOL
- **Endpoint:** `POST /send/sol`
- **Description:** Creates a SOL transfer instruction.

### 7. Send Token
- **Endpoint:** `POST /send/token`
- **Description:** Creates an SPL token transfer instruction.

## Setup

1. Clone the repository:
   ```
   git clone https://your-repo-url.git
   cd solana-http-server
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the server:
   ```
   cargo run
   ```

## Usage

You can interact with the server using tools like `curl` or Postman. Make sure to send requests to the appropriate endpoints as specified above.

## Dependencies

This project uses the following dependencies:
- `tokio` for asynchronous runtime
- `warp` for building the HTTP server
- `solana-sdk` for Solana-related functionalities

## License

This project is licensed under the MIT License. See the LICENSE file for more details.