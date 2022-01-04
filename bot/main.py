from flask import Flask, jsonify
from flask_apscheduler import APScheduler

from utils import config
from data_source import NewDataSource
from service.feeder import Feeder
from service.fetcher import Fetcher

app = Flask(__name__)
scheduler = APScheduler()
symbols = config.SYMBOLS.split(",")
prices = {symbol: 0 for symbol in symbols}


@app.route("/latest")
def latest():
    # Return only symbols specified in the config
    return jsonify({"prices": {symbol: prices[symbol] for symbol in symbols}})


if __name__ == "__main__":
    # Initialize services from the config
    feeder = Feeder(
        config.CHAIN_ID, config.NODE_URL, config.MNEMONIC, config.CONTRACT_ADDR, config.DEVIATION, config.MULTIPLIER
    )
    data_sources = [NewDataSource(ds) for ds in config.DATA_SOURCES.split(",")]
    fetcher = Fetcher(feeder, symbols, prices, data_sources)

    # Start a cron job to fetch price data regularly
    scheduler.add_job(id="Fetcher", func=fetcher.fetch, trigger="interval", seconds=config.INTERVAL)
    scheduler.start()

    app.run(host="0.0.0.0", port="8888")
