# cosmwasm-oracle

## Bot
To run the service
```
cd bot
python main.py
```

- Currently support 2 open-API sources, which can be found in `bot/data_source`
- The service config can be edited on `bot/utils/config.py`
- When the service is running, the latest queried price can be accessed through the REST API http://localhost:8888 which is update at every interval provided config, named `INTERVAL`
- New prices will be updated on the contract is the new prices deviate from the prices on the contract more than the given config `DEVIATION` 

Unimplemented Features
- Add source APIs to the config as currently the API is hardcoded
- Support multiple endpoints for each source
- Add `GasAdjustment` and `GasPrice` for `TerraLCD`
- Find propoer way to update price to the web server
- Change to Websocket server

## Oracle Contract
### `InstantiateMsg`
- `symbols: Vec<String>` - a list of supported symbols on the contract
- `feeder: String` - an address allowed to update price data on the contract
### `ExecuteMsg`
- `SetPrice` updates the price of a given symbol on the contract
    - `symbol: String` - a symbol to be updated
    - `price: u64` - a new price with a multiplier
- `UpdateAdmin` updates the contract's owner
    - `addr: Option<String>` - a new owner addr, if `None` then there will be no owner onward
- `UpdateFeeder` updates the feeder addr allowed to feed prices
    - `addr: String` - a new feeder addr allowed to update price data
### `QueryMsg`
- `GetPrice` returns the price of a given symbol stored on the contract
    - `symbol: String` - a symbol to qeury
- `GetAdmin` returns the owner addr
- `GetFeeder` returns the allowed feeder addr


Unimplemented Features
- Updatable supported list of symbols

# Setup with LocalTerra

## Initialize

1. Create optimizer alias
    ```
    export OPTIMIZER_VERSION="0.12.4"

    alias rust-optimizer='docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:${OPTIMIZER_VERSION}'
    ```
2. Set up LocalTerra. Please refer to [here](https://github.com/terra-money/LocalTerra).

## Compile and Deploy Contract

1. Compile the oracle contract
    ```
    cd oracle-contract
    rust-optimizer
    ```
    After this, you should see a directory `artifacts` containing `oracle_contract.wasm`.
2. Upload Compile Code
    ```
    terrad tx wasm store artifacts/oracle_contract.wasm --from test1 --chain-id=localterra --gas=2000000 --fees=100000uluna --broadcast-mode=block
    ```
3. Initialize the contract with the wallet `test1` as the owner and feeder
    ```
    terrad tx wasm instantiate 1 '{"feeder":"terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v", "symbols": ["BTC", "ETH", "LUNA"]}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
    ```
## Start the service
Go to the main directory
```
python main.py
```
Then the service will start fetching the price data. Yo can check if the service is running properly via REST api at http://localhost:8888/latest.
