from terra_sdk.client.lcd import LCDClient
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.core.auth import StdFee
from terra_sdk.core.wasm import MsgExecuteContract

from typing import Dict


# Feeder is a service for updating prices on contract if deviating too much
class Feeder:
    def __init__(
        self, chain_id: str, node_url: str, mnemonic: str, contract_addr: str, deviation: float, multiplier: int
    ):
        self.client = LCDClient(chain_id=chain_id, url=node_url)
        self.wallet = self.client.wallet(MnemonicKey(mnemonic))
        self.contract_addr = contract_addr
        self.deviation = deviation
        self.multiplier = multiplier

    # feed_if_deviate() checks if the new prices deviate from the prices on contract
    # If so, update the prices on the contract
    def feed_if_deviate(self, prices: Dict[str, float]):
        for symbol, price in prices.items():
            new_price = int(price * self.multiplier)
            old_price = self._get_price(symbol)
            if old_price == 0 or self._deviate(new_price, old_price):
                self._set_price(symbol, new_price)

    # _deviate() checks if new_price deviates from old_price more than the threshold
    def _deviate(self, new_price: int, old_price: int) -> bool:
        dev = abs(new_price - old_price) / float(old_price)
        return dev >= self.deviation

    # _get_price() gets the price of the given symbol from the contract
    def _get_price(self, symbol: str) -> int:
        res = self.client.wasm.contract_query(self.contract_addr, {"get_price": {"symbol": symbol}})
        return res["price"]

    # _set_price() sets the given price of the given symbol onto the contract
    def _set_price(self, symbol: str, price: int):
        tx = self.wallet.create_and_sign_tx(
            msgs=[
                MsgExecuteContract(
                    self.wallet.key.acc_address,
                    self.contract_addr,
                    {"set_price": {"symbol": symbol, "price": price}},
                    {"uluna": 100000},
                )
            ],
            fee=StdFee(200000, "120000uluna"),
        )
        result = self.client.tx.broadcast(tx)
        assert result.code != 0
