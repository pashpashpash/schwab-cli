# Schwab Market Data REST API

Source: `market-data-production-specs.html`

Endpoint count: `10`

## GET `/quotes`

Operation: `Quotes.getQuotes`

Get Quotes by list of symbols.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbols` | `query` | `string` | `false` | Comma separated list of symbol(s) to look up a quote Example : MRAD,EATOF,EBIZ,AAPL,BAC,AAAHX,AAAIX,$DJI,$SPX,MVEN,SOBS,TOITF,CNSWF,AMZN 230317C01360000,DJX 231215C00290000,/ESH23,./ADUF23C0.55,AUD/CAD |
| `fields` | `query` | `string` | `false` | Request for subset of data by passing coma separated list of root nodes, possible root nodes are quote, fundamental, extended, reference, regular. Sending quote, fundamental in request will return quote and fundamental data in response. Dont send this attribute for full response. Default value : all |
| `indicative` | `query` | `boolean` | `false` | Include indicative symbol quotes for all ETF symbols in request. If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV Available values : true, false Example : false |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | Quote Response | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/{symbol_id}/quotes`

Operation: `Quotes.getQuote`

Get Quote by single symbol.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol_id` | `path` | `string` | `true` | Symbol of instrument Example : TSLA |
| `fields` | `query` | `string` | `false` | Request for subset of data by passing coma separated list of root nodes, possible root nodes are quote, fundamental, extended, reference, regular. Sending quote, fundamental in request will return quote and fundamental data in response. Dont send this attribute for full response. Default value : all |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | Quote Response | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/chains`

Operation: `Option_Chains.getChain`

Get option chain for an optionable Symbol

Get Option Chain including information on options contracts associated with each expiration.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol` | `query` | `string` | `true` | Enter one symbol Example : AAPL |
| `contractType` | `query` | `string` | `false` | Contract Type Available values : CALL, PUT, ALL |
| `strikeCount` | `query` | `integer` | `false` | The Number of strikes to return above or below the at-the-money price |
| `includeUnderlyingQuote` | `query` | `boolean` | `false` | Underlying quotes to be included |
| `strategy` | `query` | `string` | `false` | OptionChain strategy. Default is SINGLE. ANALYTICAL allows the use of volatility, underlyingPrice, interestRate, and daysToExpiration params to calculate theoretical values. Available values : SINGLE, ANALYTICAL, COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, BUTTERFLY, CONDOR, DIAGONAL, COLLAR, ROLL |
| `interval` | `query` | `number ($double)` | `false` | Strike interval for spread strategy chains (see strategy param) |
| `strike` | `query` | `number ($double)` | `false` | Strike Price |
| `range` | `query` | `string` | `false` | Range(ITM/NTM/OTM etc.) |
| `fromDate` | `query` | `string ($date)` | `false` | From date(pattern: yyyy-MM-dd) |
| `toDate` | `query` | `string ($date)` | `false` | To date (pattern: yyyy-MM-dd) |
| `volatility` | `query` | `number ($double)` | `false` | Volatility to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param) |
| `underlyingPrice` | `query` | `number ($double)` | `false` | Underlying price to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param) |
| `interestRate` | `query` | `number ($double)` | `false` | Interest rate to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param) |
| `daysToExpiration` | `query` | `integer ($int32)` | `false` | Days to expiration to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param) |
| `expMonth` | `query` | `string` | `false` | Expiration month Available values : JAN, FEB, MAR, APR, MAY, JUN, JUL, AUG, SEP, OCT, NOV, DEC, ALL |
| `optionType` | `query` | `string` | `false` | Option Type |
| `entitlement` | `query` | `string` | `false` | Applicable only if its retail token, entitlement of client PP-PayingPro, NP-NonPro and PN-NonPayingPro Available values : PN, NP, PP |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | The Chain for the symbol was returned successfully. | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/expirationchain`

Operation: `Option_Expiration_Chain.getExpirationChain`

Get option expiration chain for an optionable symbol

Get Option Expiration (Series) information for an optionable symbol. Does not include individual options contracts for the underlying.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol` | `query` | `string` | `true` | Enter one symbol Example : AAPL |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | The Expiration Chain for the symbol was returned successfully. | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/pricehistory`

Operation: `PriceHistory.getPriceHistory`

Get PriceHistory for a single symbol and date ranges.

Get historical Open, High, Low, Close, and Volume for a given frequency (i.e. aggregation). Frequency available is dependent on periodType selected. The datetime format is in EPOCH milliseconds.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol` | `query` | `string` | `true` | The Equity symbol used to look up price history Example : AAPL |
| `periodType` | `query` | `string` | `false` | The chart period being requested. Available values : day, month, year, ytd |
| `period` | `query` | `integer ($int32)` | `false` | The number of chart period types.; ; If the periodType is ; • day - valid values are 1, 2, 3, 4, 5, 10; • month - valid values are 1, 2, 3, 6; • year - valid values are 1, 2, 3, 5, 10, 15, 20; • ytd - valid values are 1; ; If the period is not specified and the periodType is; • day - default period is 10.; • month - default period is 1.; • year - default period is 1.; • ytd - default period is 1.; |
| `frequencyType` | `query` | `string` | `false` | The time frequencyType; ; If the periodType is ; • day - valid value is minute; • month - valid values are daily, weekly; • year - valid values are daily, weekly, monthly; • ytd - valid values are daily, weekly; ; If frequencyType is not specified, default value depends on the periodType; • day - defaulted to minute.; • month - defaulted to weekly.; • year - defaulted to monthly.; • ytd - defaulted to weekly.; Available values : minute, daily, weekly, monthly |
| `frequency` | `query` | `integer ($int32)` | `false` | The time frequency duration; ; If the frequencyType is ; • minute - valid values are 1, 5, 10, 15, 30; • daily - valid value is 1; • weekly - valid value is 1; • monthly - valid value is 1; ; If frequency is not specified, default value is 1 ; |
| `startDate` | `query` | `integer ($int64)` | `false` | The start date, Time in milliseconds since the UNIX epoch eg 1451624400000; If not specified startDate will be (endDate - period) excluding weekends and holidays. |
| `endDate` | `query` | `integer ($int64)` | `false` | The end date, Time in milliseconds since the UNIX epoch eg 1451624400000; If not specified, the endDate will default to the market close of previous business day. |
| `needExtendedHoursData` | `query` | `boolean` | `false` | Need extended hours data |
| `needPreviousClose` | `query` | `boolean` | `false` | Need previous close price/date |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | Get all candles for given date range |  |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/movers/{symbol_id}`

Operation: `Movers.getMovers`

Get Movers for a specific index.

Get a list of top 10 securities movement for a specific index.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol_id` | `path` | `string` | `true` | Index Symbol Available values : $DJI, $COMPX, $SPX, NYSE, NASDAQ, OTCBB, INDEX_ALL, EQUITY_ALL, OPTION_ALL, OPTION_PUT, OPTION_CALL Example : $DJI |
| `sort` | `query` | `string` | `false` | Sort by a particular attribute Available values : VOLUME, TRADES, PERCENT_CHANGE_UP, PERCENT_CHANGE_DOWN Example : VOLUME |
| `frequency` | `query` | `integer ($int32)` | `false` | To return movers with the specified directions of up or down Available values : 0, 1, 5, 10, 30, 60 Default value : 0 |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | Analytics for the symbol was returned successfully. | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/markets`

Operation: `MarketHours.getMarketHours`

Get Market Hours for different markets.

Get Market Hours for dates in the future across different markets.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `markets` | `query` | `array[string]` | `true` | List of markets Available values : equity, option, bond, future, forex |
| `date` | `query` | `string ($date)` | `false` | Valid date range is from currentdate to 1 year from today. It will default to current day if not entered. Date format:YYYY-MM-DD |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | OK | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/markets/{market_id}`

Operation: `MarketHours.getMarketHour`

Get Market Hours for a single market.

Get Market Hours for dates in the future for a single market.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `market_id` | `path` | `string` | `true` | market id Available values : equity, option, bond, future, forex |
| `date` | `query` | `string ($date)` | `false` | Valid date range is from currentdate to 1 year from today. It will default to current day if not entered. Date format:YYYY-MM-DD |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | OK | `Schwab-Client-CorrelId` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/instruments`

Operation: `Instruments.getInstruments`

Get Instruments by symbols and projections.

Get Instruments details by using different projections. Get more specific fundamental instrument data by using fundamental as the projection.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `symbol` | `query` | `string` | `true` | symbol of a security |
| `projection` | `query` | `string` | `true` | search by Available values : symbol-search, symbol-regex, desc-search, desc-regex, search, fundamental |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | OK | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |

## GET `/instruments/{cusip_id}`

Operation: `Instruments.getInstrumentsByCusip`

Get Instrument by specific cusip

Get basic instrument details by cusip

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `cusip_id` | `path` | `string` | `true` | cusip of a security |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | OK | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `400` | Error response for generic client error 400 | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `401` | Error response for 401 Unauthorized | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `404` | Error response for 404 Not Found | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
| `500` | Error response for 500 Internal Server Error | `Schwab-Client-CorrelId`, `Schwab-Resource-Version` |
