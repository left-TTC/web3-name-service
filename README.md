# Web3-name-service


## Build
```bash
cargo build-sbf
```

## Deploy
```bash
solana program deploy --program-id target/deploy/web3_domain_name_service-keypair.json target
/sbf-solana-solana/release/web3_domain_name_service.so  --use-rpc
```