# Schwab API Reference

Generated at `2026-05-04T23:55:21-07:00` from locally saved Schwab Developer Portal HTML.

This directory is generated reference material for building the Schwab CLI. It intentionally contains no app client ID, no app secret, no access token, and no refresh token.

These files are not official Schwab documentation and may be stale relative to the live Schwab Developer Portal. Review Schwab's developer terms before redistributing this directory publicly.

## Sources

| Key | Source | Size | Modified | SHA-256 |
| --- | --- | ---: | --- | --- |
| `trading_specs` | `trading-production-specs.html` | 1659974 | `2026-05-04T23:47:49-07:00` | `1cd11f2999a087267c1b10d5995cc50eb76b87b31f790067517238f92d6f0765` |
| `market_specs` | `market-data-production-specs.html` | 1710498 | `2026-05-04T23:49:02-07:00` | `6a677de034cf140190715aefd69c189944521807ec69a361d113d086ba1a7ef1` |
| `trading_docs` | `account-trading-production-docs.html` | 396995 | `2026-05-04T23:13:58-07:00` | `fc2bb0b0f85918a060f95cf85d472128d2dd7ef001ef29038c4130c97f2b0fd3` |
| `market_docs` | `market-data-production-docs.html` | 459969 | `2026-05-04T23:14:25-07:00` | `fa0b8fc3f2dfa980ff1c9125e7f719177a9076b1555e9d5210e8236af0a8f76d` |


## Extracted Surface

| Area | Count |
| --- | ---: |
| Accounts and Trading REST endpoints | 13 |
| Market Data REST endpoints | 10 |
| Accounts and Trading models | 84 |
| Market Data models | 57 |

## Files

- `oauth.md`: OAuth authorization-code flow, token exchange, refresh behavior, token lifetimes, and callback rules.
- `trading-rest.md`: Accounts, orders, transactions, and user preference REST endpoints.
- `market-data-rest.md`: Quotes, option chains, price history, movers, market hours, and instruments REST endpoints.
- `streamer.md`: Schwab Streamer WebSocket protocol and field maps.
- `order-examples.md`: Order JSON examples from Schwab's documentation.
- `endpoint-catalog.json`: Machine-readable endpoint catalog extracted from the expanded Swagger DOM.
- `schemas/trading-models.md` and `schemas/trading-models.json`: Accounts and Trading schema summaries.
- `schemas/market-data-models.md` and `schemas/market-data-models.json`: Market Data schema summaries.
