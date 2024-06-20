# Identity Smart Contract

## Overview

This smart contract allows users to create, update, and query identity metadata on the blockchain. It is built using CosmWasm and provides functionalities for managing digital identities in a decentralized manner.

A minimalistic web interface was developed to test the smart contract API. You can find it in [this repository](https://github.com/wotori/cw-dev-interface).

## Features

- **Mint/Update Identity**: Users can create a new identity by minting an identity metadata with one multipurpose UpdateMetadata method.
- **Query Identity**: Anyone can query the identity metadata of a specific user by their address.
- **Query All Identities**: Retrieve all identity metadata entries stored in the contract.

## Execute Messages

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

`archway contracts query smart identity --args-file './queryMsg.json'`

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

## Deploy

- `archway contracts build`
- `archway contracts store identity`
- `archway contracts instantiate --code 3001 --args '{}' --label wotori-identity`

## License

This project is licensed under the MIT License
