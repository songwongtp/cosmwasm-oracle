from statistics import median

from .feeder import Feeder
from data_source import DataSource

from typing import Dict, List


# Fetcher is a service that does the followings
# - request the givens symbols on all given data sources and aggregate the results
# - update the price on the main app flask through `flask_output`
# - send the price to Feeder service
class Fetcher:
    def __init__(
        self,
        feeder: Feeder,
        symbols: List[str],
        flask_output: Dict[str, float],
        data_sources: List[DataSource],
        aggregate_fn=median,
    ):
        self.feeder = feeder
        self.symbols = symbols
        self.flask_output = flask_output
        self.data_sources = data_sources
        self.aggregate_fn = aggregate_fn

        self.fetch()

    # fetch() gets the new prices and updates the main flask and feeder
    def fetch(self):
        prices = self.request()
        self.flask_output.update(prices)

        self.feeder.feed_if_deviate(self.flask_output)

    # request() makes requests to all data sources and aggregates the results
    def request(self):
        res = [ds.request(self.symbols) for ds in self.data_sources]
        return {symbol: self.aggregate_fn([r[symbol] for r in res]) for symbol in self.symbols}
