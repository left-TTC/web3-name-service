# Web3-name-service

## Overview
The core contract used to create the actual effective domain name

## About Domain

### create
All the domain can only be created by [registrar](https://github.com/left-TTC/web3-domain-registrar). 
You can create a domain name by initiating an auction. During the bidding period, if you are the highest bidder, you may preconfigure the content of the domain. Anyone can access the content of your domain through [Kilo](https://github.com/left-TTC/kilo-browser). Once the bidding period has ended, the domain can be settled through a permissionless settle transaction.

### set
You can create a DNS record through the [Record](https://github.com/left-TTC/web3-records) contract. It supports storing IPFS and IPNS CIDs, and Tor content will be supported soon.

### freeze
While you hold the domain, you may execute a freeze instruction. This instruction will permanently freeze the current record and is completely irreversible.

## Build
```bash
cargo build-sbf
```

```bash
solana-keygen pubkey target/deploy/web3_domain_name_service-keypair.json
```

```bash
solana program deploy --program-id target/deploy/web3_domain_name_service-keypair.json target/sbpf-solana-solana/release/web3_domain_name_service.so  --use-rpc
```