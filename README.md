# Identity Smart Contract

## Overview

This smart contract allows users to create, update, and query identity metadata on the blockchain. It is built using CosmWasm and provides functionalities for managing digital identities in a decentralized manner.

## Features

- **Mint Identity**: Users can create a new identity by minting an identity metadata entry.
- **Update Metadata**: Users can update their existing identity metadata.
- **Query Identity**: Anyone can query the identity metadata of a specific user by their address.
- **Query All Identities**: Retrieve all identity metadata entries stored in the contract.

## Execute Messages

- Mint Identity:

```json
{
  "mint": {
    "identity_data": {
      "name": "Alice",
      "pic": "ipfs://pic",
      "address": "cosmos1...",
      "about": "About Alice",
      "avatar": "ipfs://avatar"
    }
  }
}
```

- Update Metadata:

```json
{
  "update_metadata": {
    "identity_data": {
      "name": "Alice Updated",
      "pic": "ipfs://newpic",
      "address": "cosmos1...",
      "about": "Updated About Alice",
      "avatar": "ipfs://newavatar"
    }
  }
}
```

## Query Messages

- User Info:

```json
{
  "user_info": {
    "address": "cosmos1..."
  }
}
```

- User Info All:

```json
{
  "user_info_all": {}
}
```

## Contract Structure

- lib.rs: Main entry point of the contract.
- exec.rs: Handles execution of contract functions such as minting and updating identities.
- models.rs: Defines the data structures used in the contract.
- msg.rs: Contains the message types for executing and querying the contract.
- que.rs: Handles query functions for the contract.
- states.rs: Manages the state storage for the contract.

## License

This project is licensed under the MIT License
