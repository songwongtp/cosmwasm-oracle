from typing import Dict, List


# DataSource is an interface specifying the minimum necessities for data source classes
class DataSource:
    api: str
    symbols: List[str]

    def request(self, symbols: List[str]) -> Dict[str, float]:
        pass
