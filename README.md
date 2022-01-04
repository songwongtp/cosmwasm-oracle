# cosmwasm-oracle

## Initialize

```
export OPTIMIZER_VERSION="0.12.4"
alias rust-optimizer='docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:${OPTIMIZER_VERSION}'
```

## 