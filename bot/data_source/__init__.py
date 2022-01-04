from .data_source import DataSource
from .binance import Binance
from .coingecko import Coingecko


# NewDataSource is a helper function for selecting DataSource classes based on the input
def NewDataSource(source: str) -> DataSource:
    if source == "binance":
        return Binance()
    elif source == "coingecko":
        return Coingecko()
    else:
        raise NotImplementedError
