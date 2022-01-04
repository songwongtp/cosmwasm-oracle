import requests
from .data_source import DataSource

from typing import Dict, List


# Coingecko is a class implementing DataSource interface for requesting Coingecko API
class Coingecko(DataSource):
    def __init__(self, api="https://api.coingecko.com"):
        self.api = api
        self.slugs = None

    def request(self, symbols: List[str]) -> Dict[str, float]:
        if self.slugs == None:
            self.slugs = self._init_slugs(symbols)

        res = requests.get(
            self.api + "/api/v3/simple/price",
            params={"ids": ",".join([slug for slug in self.slugs.values()]), "vs_currencies": "usd"},
        )
        res.raise_for_status()

        res = res.json()
        result = {symbol: res[slug]["usd"] for symbol, slug in self.slugs.items()}

        assert len(result) == len(symbols)
        return result

    def _init_slugs(self, symbols: List[str]) -> Dict[str, str]:
        res = requests.get(self.api + "/api/v3/coins/list")
        res.raise_for_status()

        return {token["symbol"].upper(): token["id"] for token in res.json() if token["symbol"].upper() in symbols}
