# Getting Started

## Build

```bash
# Build
cargo wasm
cargo run-script optimize

cosmwasm-check ./target/wasm32-unknown-unknown/release/cw20_balance_contract.wasm
```

## Deployment
```bash
terrad keys add dev-wallet --recover

terrad tx wasm store artifacts/cw20_balance_contract.wasm \
  --from dev-wallet \
  --chain-id=phoenix-1 \
  --gas=auto \
  --gas-adjustment=1.3 \
  --fees=21500uluna \
  --node=https://terra-rpc.publicnode.com:443

# 9609DB0F1DD8530DE17AFDCE87B0F0EC4F13F65A25CA3AE20A1CF68203694F4B
# code 1780

terrad tx wasm init 1781 '{}' \
  --from dev-wallet \
  --label=cw20-balances-contract \
  --admin=terra1g0gtulu06u8uht8wsam0flfqg0drp69pm2s3km \
  --chain-id=phoenix-1 \
  --gas=auto \
  --gas-adjustment=1.3 \
  --fees=10000uluna \
  --node=https://terra-rpc.publicnode.com:443

# 8F149CAB30364E91139D48FDDFD0ACC0F720A07AE376EA3A07E85107B785BAFA

terrad query wasm contract terra167r6f7aeuknffkrq6t3cm8r5l8ngke77dpgzhclgjhj25mgxdafqgrg6u3 \
  --node=https://terra-rpc.publicnode.com:443

terrad query wasm contract-state smart terra167r6f7aeuknffkrq6t3cm8r5l8ngke77dpgzhclgjhj25mgxdafqgrg6u3 '{"balances":{"address":"terra1djkwwpgevx42h8j7zwtgtvy2zcd2z345n5kuhn","tokens":["terra19p20mfnvwh9yvyr7aus3a6z6g6uk28fv4jhx9kmnc2m7krg27q2qkfenjw","terra1lxx40s29qvkrcj8fsa3yzyehy7w50umdvvnls2r830rys6lu2zns63eelv","terra1t4p3u8khpd7f8qzurwyafxt648dya6mp6vur3vaapswt6m24gkuqrfdhar","terra10aa3zdkrc7jwuf8ekl3zq7e7m42vmzqehcmu74e4egc7xkm5kr2s0muyst","terra14xsm2wzvu7xaf567r693vgfkhmvfs08l68h4tjj5wjgyn5ky8e2qvzyanh","ibc/36A02FFC4E74DF4F64305130C3DFA1B06BEAC775648927AA44467C76A77AB8DB","ibc/36A02FFC4E74DF4F64305130C3DFA1B06BEAC775648927AA44467C76A77AB8DB","ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"]}}' \
  --node=https://terra-rpc.publicnode.com:443
```

### Update

```bash
terrad tx wasm migrate terra167r6f7aeuknffkrq6t3cm8r5l8ngke77dpgzhclgjhj25mgxdafqgrg6u3 1781 '{}' \
  --from dev-wallet \
  --chain-id=phoenix-1 \
  --gas=auto \
  --gas-adjustment=1.3 \
  --fees=10000uluna \
  --node=https://terra-rpc.publicnode.com:443
```

## Others
```bash
cargo schema
cargo test
```