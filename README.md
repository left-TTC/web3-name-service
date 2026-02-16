# Web3-name-service


## Build
```bash
cargo build-sbf
```

## Check Pubky
```bash
solana-keygen pubkey target/deploy/web3_domain_name_service-keypair.json
```

## Deploy
```bash
solana program deploy --program-id target/deploy/web3_domain_name_service-keypair.json target/sbpf-solana-solana/release/web3_domain_name_service.so  --use-rpc
```