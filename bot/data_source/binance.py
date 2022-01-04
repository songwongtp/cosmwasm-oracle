import requests
from .data_source import DataSource

from typing import Dict, List


# Binance is a class implementing DataSource interface for requesting Binance API
class Binance(DataSource):
    def __init__(self, api="https://api.binance.com"):
        self.api = api

    def request(self, symbols: List[str]) -> Dict[str, float]:
        result = {}
        for symbol in symbols:
            res = requests.get(self.api + "/api/v3/ticker/price", params={"symbol": symbol + "USDT"})
            res.raise_for_status()

            result[symbol] = float(res.json()["price"])

        assert len(result) == len(symbols)
        return result
