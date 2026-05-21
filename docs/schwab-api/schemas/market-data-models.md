# Schwab Market Data Models

Source: `market-data-production-specs.html`

Model count: `57`

## `Bond`

| Field | Details |
| --- | --- |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `exchange` | string |
| `assetType` | string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |
| `bondFactor` | string |
| `bondMultiplier` | string |
| `bondPrice` | number |
| `type` | string writeOnly: true Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |

## `FundamentalInst`

| Field | Details |
| --- | --- |
| `symbol` | string |
| `high52` | number ($double) |
| `low52` | number ($double) |
| `dividendAmount` | number ($double) |
| `dividendYield` | number ($double) |
| `dividendDate` | string |
| `peRatio` | number ($double) |
| `pegRatio` | number ($double) |
| `pbRatio` | number ($double) |
| `prRatio` | number ($double) |
| `pcfRatio` | number ($double) |
| `grossMarginTTM` | number ($double) |
| `grossMarginMRQ` | number ($double) |
| `netProfitMarginTTM` | number ($double) |
| `netProfitMarginMRQ` | number ($double) |
| `operatingMarginTTM` | number ($double) |
| `operatingMarginMRQ` | number ($double) |
| `returnOnEquity` | number ($double) |
| `returnOnAssets` | number ($double) |
| `returnOnInvestment` | number ($double) |
| `quickRatio` | number ($double) |
| `currentRatio` | number ($double) |
| `interestCoverage` | number ($double) |
| `totalDebtToCapital` | number ($double) |
| `ltDebtToEquity` | number ($double) |
| `totalDebtToEquity` | number ($double) |
| `epsTTM` | number ($double) |
| `epsChangePercentTTM` | number ($double) |
| `epsChangeYear` | number ($double) |
| `epsChange` | number ($double) |
| `revChangeYear` | number ($double) |
| `revChangeTTM` | number ($double) |
| `revChangeIn` | number ($double) |
| `sharesOutstanding` | number ($double) |
| `marketCapFloat` | number ($double) |
| `marketCap` | number ($double) |
| `bookValuePerShare` | number ($double) |
| `shortIntToFloat` | number ($double) |
| `shortIntDayToCover` | number ($double) |
| `divGrowthRate3Year` | number ($double) |
| `dividendPayAmount` | number ($double) |
| `dividendPayDate` | string |
| `beta` | number ($double) |
| `vol1DayAvg` | number ($double) |
| `vol10DayAvg` | number ($double) |
| `vol3MonthAvg` | number ($double) |
| `avg10DaysVolume` | integer ($int64) |
| `avg1DayVolume` | integer ($int64) |
| `avg3MonthVolume` | integer ($int64) |
| `declarationDate` | string |
| `dividendFreq` | integer ($int32) |
| `eps` | number ($double) |
| `corpactionDate` | string |
| `dtnVolume` | integer ($int64) |
| `nextDividendPayDate` | string |
| `nextDividendDate` | string |
| `fundLeverageFactor` | number ($double) |
| `fundStrategy` | string |

## `Instrument`

| Field | Details |
| --- | --- |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `exchange` | string |
| `assetType` | string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |
| `type` | string writeOnly: true Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |

## `InstrumentResponse`

| Field | Details |
| --- | --- |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `exchange` | string |
| `assetType` | string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |
| `bondFactor` | string |
| `bondMultiplier` | string |
| `bondPrice` | number |
| `fundamental` | #/components/schemas/FundamentalInst FundamentalInst { symbol string high52 number ($double) low52 number ($double) dividendAmount number ($double) dividendYield number ($double) dividendDate string peRatio number ($double) pegRatio number ($double) pbRatio number ($double) prRatio number ($double) pcfRatio number ($double) grossMarginTTM number ($double) grossMarginMRQ number ($double) netProfitMarginTTM number ($double) netProfitMarginMRQ number ($double) operatingMarginTTM number ($double) operatingMarginMRQ number ($double) returnOnEquity number ($double) returnOnAssets number ($double) returnOnInvestment number ($double) quickRatio number ($double) currentRatio number ($double) interestCoverage number ($double) totalDebtToCapital number ($double) ltDebtToEquity number ($double) totalDebtToEquity number ($double) epsTTM number ($double) epsChangePercentTTM number ($double) epsChangeYear number ($double) epsChange number ($double) revChangeYear number ($double) revChangeTTM number ($double) revChangeIn number ($double) sharesOutstanding number ($double) marketCapFloat number ($double) marketCap number ($double) bookValuePerShare number ($double) shortIntToFloat number ($double) shortIntDayToCover number ($double) divGrowthRate3Year number ($double) dividendPayAmount number ($double) dividendPayDate string beta number ($double) vol1DayAvg number ($double) vol10DayAvg number ($double) vol3MonthAvg number ($double) avg10DaysVolume integer ($int64) avg1DayVolume integer ($int64) avg3MonthVolume integer ($int64) declarationDate string dividendFreq integer ($int32) eps number ($double) corpactionDate string dtnVolume integer ($int64) nextDividendPayDate string nextDividendDate string fundLeverageFactor number ($double) fundStrategy string } |
| `instrumentInfo` | #/components/schemas/Instrument Instrument { cusip string symbol string description string exchange string assetType string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] type string writeOnly: true Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] } |
| `bondInstrumentInfo` | #/components/schemas/Bond Bond { cusip string symbol string description string exchange string assetType string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] bondFactor string bondMultiplier string bondPrice number type string writeOnly: true Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] } |
| `type` | string writeOnly: true Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |

## `Hours`

| Field | Details |
| --- | --- |
| `date` | string |
| `marketType` | string Enum: [ BOND, EQUITY, ETF, EXTENDED, FOREX, FUTURE, FUTURE_OPTION, FUNDAMENTAL, INDEX, INDICATOR, MUTUAL_FUND, OPTION, UNKNOWN ] |
| `exchange` | string |
| `category` | string |
| `product` | string |
| `productName` | string |
| `isOpen` | boolean |
| `sessionHours` | { < * >: [] #/components/schemas/Interval Interval { start string end string } } |

## `Interval`

| Field | Details |
| --- | --- |
| `start` | string |
| `end` | string |

## `Screener`

| Field | Details |
| --- | --- |
| `description:` | Security info of most moved with in an index |
| `change` | number ($double) percent or value changed, by default its percent changed |
| `description` | string Name of security |
| `direction` | string Enum: [ up, down ] |
| `last` | number ($double) what was last quoted price |
| `symbol` | string schwab security symbol |
| `totalVolume` | integer ($int64) |

## `Candle`

| Field | Details |
| --- | --- |
| `close` | number ($double) |
| `datetime` | integer ($int64) |
| `datetimeISO8601` | string ($yyyy-MM-dd) |
| `high` | number ($double) |
| `low` | number ($double) |
| `open` | number ($double) |
| `volume` | integer ($int64) |

## `CandleList`

| Field | Details |
| --- | --- |
| `candles` | [] #/components/schemas/Candle Candle { close number ($double) datetime integer ($int64) datetimeISO8601 string ($yyyy-MM-dd) high number ($double) low number ($double) open number ($double) volume integer ($int64) } |
| `empty` | boolean |
| `previousClose` | number ($double) |
| `previousCloseDate` | integer ($int64) |
| `previousCloseDateISO8601` | string ($yyyy-MM-dd) |
| `symbol` | string |

## `EquityResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Equity security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `assetSubType` | EquityAssetSubType string nullable: true Asset Sub Type (only there if applicable) Enum: [ COE, PRF, ADR, GDR, CEF, ETF, ETN, UIT, WAR, RGT, ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quoteType` | QuoteType string nullable: true NBBO - realtime, NFL - Non-fee liable quote. Enum: [ NBBO, NFL, ] |
| `extended` | #/components/schemas/ExtendedMarket ExtendedMarket { description: Quote data for extended hours askPrice number ($double) example: 124.85 Extended market ask price askSize integer ($int32) example: 51771 Extended market ask size bidPrice number ($double) example: 124.85 Extended market bid price bidSize integer ($int32) example: 51771 Extended market bid size lastPrice number ($double) example: 124.85 Extended market last price lastSize integer ($int32) example: 51771 Regular market last size mark number ($double) example: 1.1246 mark price quoteTime integer ($int64) example: 1621368000400 Extended market quote time in milliseconds since Epoch totalVolume number ($int64) example: 12345 Total volume tradeTime integer ($int64) example: 1621368000400 Extended market trade time in milliseconds since Epoch } |
| `fundamental` | #/components/schemas/Fundamental Fundamental { description: Fundamentals of a security avg10DaysVolume number ($double) Average 10 day volume avg1YearVolume number ($double) Average 1 day volume declarationDate string ($date-time) example: 2021-04-28T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Declaration date in yyyy-mm-ddThh:mm:ssZ divAmount number ($double) example: 0.88 Dividend Amount divExDate string ($yyyy-MM-dd'T'HH:mm:ssZ) example: 2021-05-07T00:00:00Z Dividend date in yyyy-mm-ddThh:mm:ssZ divFreq DivFreq integer nullable: true Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly Enum: [ 1, 2, 3, 4, 6, 11, 12, ] divPayAmount number ($double) example: 0.22 Dividend Pay Amount divPayDate string ($date-time) example: 2021-05-13T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Dividend pay date in yyyy-mm-ddThh:mm:ssZ divYield number ($double) example: 0.7 Dividend yield eps number ($double) example: 4.45645 Earnings per Share fundLeverageFactor number ($double) example: -1 Fund Leverage Factor + > 0 <- fundStrategy FundStrategy string nullable: true FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" - Quantitative "S" - Short Enum: [ A, L, P, Q, S, ] nextDivExDate string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend date nextDivPayDate string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend pay date peRatio number ($double) example: 28.599 P/E Ratio } |
| `quote` | #/components/schemas/QuoteEquity QuoteEquity { description: Quote data of Equity security 52WeekHigh number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks 52WeekLow number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks askMICId string example: XNYS ask MIC code askPrice number ($double) example: 124.63 Current Best Ask Price askSize integer ($int32) example: 700 Number of shares for ask askTime integer ($int64) example: 1621376892336 Last ask time in milliseconds since Epoch bidMICId string example: XNYS bid MIC code bidPrice number ($double) example: 124.6 Current Best Bid Price bidSize integer ($int32) example: 300 Number of shares for bid bidTime integer ($int64) example: 1621376892336 Last bid time in milliseconds since Epoch closePrice number ($double) example: 126.27 Previous day's closing price highPrice number ($double) example: 126.99 Day's high trade price lastMICId string example: XNYS Last MIC Code lastPrice number ($double) example: 122.3 lastSize integer ($int32) example: 100 Number of shares traded with last trade lowPrice number ($double) Day's low trade price mark number ($double) example: 52.93 Mark price markChange number ($double) example: -0.01 Mark Price change markPercentChange number ($double) example: -0.0189 Mark Price percent change netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change openPrice number ($double) example: 52.8 Price at market open quoteTime integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch securityStatus string example: Normal Status of security totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch volatility number ($double) example: 0.0094 Option Risk/Volatility Measurement } |
| `reference` | #/components/schemas/ReferenceEquity ReferenceEquity { description: Reference data of Equity security cusip string example: A23456789 CUSIP of Instrument description string example: Apple Inc. - Common Stock Description of Instrument exchange string example: q Exchange Code exchangeName string Exchange Name fsiDesc string maxLength: 50 FSI Desc htbQuantity integer ($int32) example: 100 Hard to borrow quantity. htbRate number ($double) example: 4.5 Hard to borrow rate. isHardToBorrow boolean example: false is Hard to borrow security. isShortable boolean example: false is shortable security. otcMarketTier string maxLength: 10 OTC Market Tier } |
| `regular` | #/components/schemas/RegularMarket RegularMarket { description: Market info of security regularMarketLastPrice number ($double) example: 124.85 Regular market last price regularMarketLastSize integer ($int32) example: 51771 Regular market last size regularMarketNetChange number ($double) example: -1.42 Regular market net change regularMarketPercentChange number ($double) example: -1.1246 Regular market percent change regularMarketTradeTime integer ($int64) example: 1621368000400 Regular market trade time in milliseconds since Epoch } |

## `QuoteError`

| Field | Details |
| --- | --- |
| `description:` | Partial or Custom errors per request |
| `invalidCusips` | [] list of invalid cusips from request string |
| `invalidSSIDs` | [] list of invalid SSIDs from request integer ($int64) |
| `invalidSymbols` | [] list of invalid symbols from request string |

## `ExtendedMarket`

| Field | Details |
| --- | --- |
| `description:` | Quote data for extended hours |
| `askPrice` | number ($double) example: 124.85 Extended market ask price |
| `askSize` | integer ($int32) example: 51771 Extended market ask size |
| `bidPrice` | number ($double) example: 124.85 Extended market bid price |
| `bidSize` | integer ($int32) example: 51771 Extended market bid size |
| `lastPrice` | number ($double) example: 124.85 Extended market last price |
| `lastSize` | integer ($int32) example: 51771 Regular market last size |
| `mark` | number ($double) example: 1.1246 mark price |
| `quoteTime` | integer ($int64) example: 1621368000400 Extended market quote time in milliseconds since Epoch |
| `totalVolume` | number ($int64) example: 12345 Total volume |
| `tradeTime` | integer ($int64) example: 1621368000400 Extended market trade time in milliseconds since Epoch |

## `ForexResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Forex security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quote` | #/components/schemas/QuoteForex QuoteForex { description: Quote data of Forex security 52WeekHigh number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks 52WeekLow number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks askPrice number ($double) example: 124.63 Current Best Ask Price askSize integer ($int32) example: 700 Number of shares for ask bidPrice number ($double) example: 124.6 Current Best Bid Price bidSize integer ($int32) example: 300 Number of shares for bid closePrice number ($double) example: 126.27 Previous day's closing price highPrice number ($double) example: 126.99 Day's high trade price lastPrice number ($double) example: 122.3 lastSize integer ($int32) example: 100 Number of shares traded with last trade lowPrice number ($double) example: 52.74 Day's low trade price mark number ($double) example: 52.93 Mark price netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change openPrice number ($double) example: 52.8 Price at market open quoteTime integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch securityStatus string example: Normal Status of security tick number ($double) example: 0 Tick Price tickAmount number ($double) example: 0 Tick Amount totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch } |
| `reference` | #/components/schemas/ReferenceForex ReferenceForex { description: Reference data of Forex security description string example: Euro/USDollar Spot Description of Instrument exchange string example: q Exchange Code exchangeName string Exchange Name isTradable boolean example: true is FOREX tradable marketMaker string Market marker product string example: null Product name tradingHours string Trading hours } |

## `Fundamental`

| Field | Details |
| --- | --- |
| `description:` | Fundamentals of a security |
| `avg10DaysVolume` | number ($double) Average 10 day volume |
| `avg1YearVolume` | number ($double) Average 1 day volume |
| `declarationDate` | string ($date-time) example: 2021-04-28T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Declaration date in yyyy-mm-ddThh:mm:ssZ |
| `divAmount` | number ($double) example: 0.88 Dividend Amount |
| `divExDate` | string ($yyyy-MM-dd'T'HH:mm:ssZ) example: 2021-05-07T00:00:00Z Dividend date in yyyy-mm-ddThh:mm:ssZ |
| `divFreq` | DivFreq integer nullable: true Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly Enum: [ 1, 2, 3, 4, 6, 11, 12, ] |
| `divPayAmount` | number ($double) example: 0.22 Dividend Pay Amount |
| `divPayDate` | string ($date-time) example: 2021-05-13T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Dividend pay date in yyyy-mm-ddThh:mm:ssZ |
| `divYield` | number ($double) example: 0.7 Dividend yield |
| `eps` | number ($double) example: 4.45645 Earnings per Share |
| `fundLeverageFactor` | number ($double) example: -1 Fund Leverage Factor + > 0 <- |
| `fundStrategy` | FundStrategy string nullable: true FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" - Quantitative "S" - Short Enum: [ A, L, P, Q, S, ] |
| `nextDivExDate` | string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend date |
| `nextDivPayDate` | string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend pay date |
| `peRatio` | number ($double) example: 28.599 P/E Ratio |

## `FutureOptionResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Future Option security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quote` | #/components/schemas/QuoteFutureOption QuoteFutureOption { description: Quote data of Option security askMICId string example: XNYS ask MIC code askPrice number ($double) example: 124.63 Current Best Ask Price askSize integer ($int32) example: 700 Number of shares for ask bidMICId string example: XNYS bid MIC code bidPrice number ($double) example: 124.6 Current Best Bid Price bidSize integer ($int32) example: 300 Number of shares for bid closePrice number ($double) example: 126.27 Previous day's closing price highPrice number ($double) example: 126.99 Day's high trade price lastMICId string example: XNYS Last MIC Code lastPrice number ($double) example: 122.3 lastSize integer ($int32) example: 100 Number of shares traded with last trade lowPrice number ($double) example: 52.74 Day's low trade price mark number ($double) example: 52.93 Mark price markChange number ($double) example: -0.04 Mark Price change netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change openInterest integer ($int32) example: 317 Open Interest openPrice number ($double) example: 52.8 Price at market open quoteTime integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch securityStatus string example: Normal Status of security settlemetPrice number ($double) example: 52.8 Price at market open tick number ($double) example: 0 Tick Price tickAmount number ($double) example: 0 Tick Amount totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch } |
| `reference` | #/components/schemas/ReferenceFutureOption ReferenceFutureOption { description: Reference data of Future Option security contractType ContractType string Indicates call or put Enum: [ P, C ] description string example: AMZN Aug 20 2021 2300 Put Description of Instrument exchange string example: q Exchange Code exchangeName string Exchange Name multiplier number ($double) example: 100 Option multiplier expirationDate integer ($int64) date of expiration in long expirationStyle string Style of expiration strikePrice number ($double) example: 2300 Strike Price underlying string example: AMZN Aug 20 2021 2300 Put A company, index or fund name } |

## `FutureResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Future security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quote` | #/components/schemas/QuoteFuture QuoteFuture { description: Quote data of Future security askMICId string example: XNYS ask MIC code askPrice number ($double) example: 4083.25 Current Best Ask Price askSize integer ($int32) example: 36 Number of shares for ask askTime integer ($int64) example: 1621376892336 Last ask time in milliseconds since Epoch bidMICId string example: XNYS bid MIC code bidPrice number ($double) example: 4083 Current Best Bid Price bidSize integer ($int32) example: 18 Number of shares for bid bidTime integer ($int64) example: 1621376892336 Last bid time in milliseconds since Epoch closePrice number ($double) example: 4123 Previous day's closing price futurePercentChange number ($double) example: -0.0756 Net Percentage Change highPrice number ($double) example: 4123 Day's high trade price lastMICId string example: XNYS Last MIC Code lastPrice number ($double) example: 4083 lastSize integer ($int32) example: 7 Number of shares traded with last trade lowPrice number ($double) example: 4075.5 Day's low trade price mark number ($double) example: 4083 Mark price netChange number ($double) example: -40 Current Last-Prev Close openInterest integer ($int32) example: 2517139 Open interest openPrice number ($double) example: 4114 Price at market open quoteTime integer ($int64) example: 1621427004585 Last quote time in milliseconds since Epoch quotedInSession boolean example: false quoted during trading session securityStatus string example: Normal Status of security settleTime integer ($int64) example: 1621376892336 settlement time in milliseconds since Epoch tick number ($double) example: 0.25 Tick Price tickAmount number ($double) example: 12.5 Tick Amount totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch } |
| `reference` | #/components/schemas/ReferenceFuture ReferenceFuture { description: Reference data of Future security description string example: E-mini S&P 500 Index Futures,Jun-2021,ETH Description of Instrument exchange string example: q Exchange Code exchangeName string Exchange Name futureActiveSymbol string example: /ESM21 Active symbol futureExpirationDate number ($int64) example: 1623988800000 Future expiration date in milliseconds since epoch futureIsActive boolean example: true Future is active futureMultiplier number ($double) example: 50 Future multiplier futurePriceFormat string example: D,D Price format futureSettlementPrice number ($double) example: 4123 Future Settlement Price futureTradingHours string example: GLBX(de=1640;0=-1700151515301600;1=r-17001515r15301600d-15551640;7=d-16401555) Trading Hours product string example: /ES Futures product symbol } |

## `IndexResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Index security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quote` | #/components/schemas/QuoteIndex QuoteIndex { description: Quote data of Index security 52WeekHigh number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks 52WeekLow number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks closePrice number ($double) example: 126.27 Previous day's closing price highPrice number ($double) example: 126.99 Day's high trade price lastPrice number ($double) example: 122.3 lowPrice number ($double) example: 52.74 Day's low trade price netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change openPrice number ($double) example: 52.8 Price at market open securityStatus string example: Normal Status of security totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch } |
| `reference` | #/components/schemas/ReferenceIndex ReferenceIndex { description: Reference data of Index security description string example: DOW JONES 30 INDUSTRIALS Description of Instrument exchange string example: q Exchange Code exchangeName string Exchange Name } |

## `MutualFundResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of MutualFund security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `assetSubType` | MutualFundAssetSubType string nullable: true Asset Sub Type (only there if applicable) Enum: [ OEF, CEF, MMF, ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `fundamental` | #/components/schemas/Fundamental Fundamental { description: Fundamentals of a security avg10DaysVolume number ($double) Average 10 day volume avg1YearVolume number ($double) Average 1 day volume declarationDate string ($date-time) example: 2021-04-28T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Declaration date in yyyy-mm-ddThh:mm:ssZ divAmount number ($double) example: 0.88 Dividend Amount divExDate string ($yyyy-MM-dd'T'HH:mm:ssZ) example: 2021-05-07T00:00:00Z Dividend date in yyyy-mm-ddThh:mm:ssZ divFreq DivFreq integer nullable: true Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly Enum: [ 1, 2, 3, 4, 6, 11, 12, ] divPayAmount number ($double) example: 0.22 Dividend Pay Amount divPayDate string ($date-time) example: 2021-05-13T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Dividend pay date in yyyy-mm-ddThh:mm:ssZ divYield number ($double) example: 0.7 Dividend yield eps number ($double) example: 4.45645 Earnings per Share fundLeverageFactor number ($double) example: -1 Fund Leverage Factor + > 0 <- fundStrategy FundStrategy string nullable: true FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" - Quantitative "S" - Short Enum: [ A, L, P, Q, S, ] nextDivExDate string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend date nextDivPayDate string ($date-time) example: 2021-02-12T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Next Dividend pay date peRatio number ($double) example: 28.599 P/E Ratio } |
| `quote` | #/components/schemas/QuoteMutualFund QuoteMutualFund { description: Quote data of Mutual Fund security 52WeekHigh number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks 52WeekLow number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks closePrice number ($double) example: 126.27 Previous day's closing price nAV number ($double) example: 126.99 Net Asset Value netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change securityStatus string example: Normal Status of security totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch } |
| `reference` | #/components/schemas/ReferenceMutualFund ReferenceMutualFund { description: Reference data of MutualFund security cusip string example: A23456789 CUSIP of Instrument description string example: Apple Inc. - Common Stock Description of Instrument exchange string default: m Exchange Code exchangeName string default: MUTUAL_FUND Exchange Name } |

## `OptionResponse`

| Field | Details |
| --- | --- |
| `description:` | Quote info of Option security |
| `assetMainType` | AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] |
| `ssid` | integer ($int64) example: 1234567890 SSID of instrument |
| `symbol` | string example: AAPL Symbol of instrument |
| `realtime` | boolean example: true is quote realtime |
| `quote` | #/components/schemas/QuoteOption QuoteOption { description: Quote data of Option security 52WeekHigh number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks 52WeekLow number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks askPrice number ($double) example: 124.63 Current Best Ask Price askSize integer ($int32) example: 700 Number of shares for ask bidPrice number ($double) example: 124.6 Current Best Bid Price bidSize integer ($int32) example: 300 Number of shares for bid closePrice number ($double) example: 126.27 Previous day's closing price delta number ($double) example: -0.0407 Delta Value gamma number ($double) example: 0.0001 Gamma Value highPrice number ($double) example: 126.99 Day's high trade price indAskPrice number ($double) example: 126.99 Indicative Ask Price applicable only for Indicative Option Symbols indBidPrice number ($double) example: 126.99 Indicative Bid Price applicable only for Indicative Option Symbols indQuoteTime integer ($int64) example: 126.99 Indicative Quote Time in milliseconds since Epoch applicable only for Indicative Option Symbols impliedYield number ($double) example: -0.0067 Implied Yield lastPrice number ($double) example: 122.3 lastSize integer ($int32) example: 100 Number of shares traded with last trade lowPrice number ($double) example: 52.74 Day's low trade price mark number ($double) example: 52.93 Mark price markChange number ($double) example: -0.01 Mark Price change markPercentChange number ($double) example: -0.0189 Mark Price percent change moneyIntrinsicValue number ($double) example: -947.96 Money Intrinsic Value netChange number ($double) example: -0.04 Current Last-Prev Close netPercentChange number ($double) example: -0.0756 Net Percentage Change openInterest number ($double) example: 317 Open Interest openPrice number ($double) example: 52.8 Price at market open quoteTime integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch rho number ($double) example: -0.3732 Rho Value securityStatus string example: Normal Status of security theoreticalOptionValue number ($double) example: 12.275 Theoretical option Value theta number ($double) example: -0.315 Theta Value timeValue number ($double) example: 12.22 Time Value totalVolume integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. tradeTime integer ($int64) example: 1621376731304 Last trade time in milli… |
| `reference` | #/components/schemas/ReferenceOption ReferenceOption { description: Reference data of Option security contractType ContractType string Indicates call or put Enum: [ P, C ] cusip string example: 0AMZN.TK12300000 CUSIP of Instrument daysToExpiration integer ($int32) example: 94 Days to Expiration deliverables string example: $6024.37 cash in lieu of shares, 212 shares of AZN Unit of trade description string example: AMZN Aug 20 2021 2300 Put Description of Instrument exchange string default: o Exchange Code exchangeName string Exchange Name exerciseType ExerciseType string option contract exercise type America or European Enum: [ A, E ] expirationDay integer ($int32) example: 20 maximum: 31 minimum: 1 Expiration Day expirationMonth integer ($int32) example: 8 maximum: 12 minimum: 1 Expiration Month expirationType ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] expirationYear integer ($int32) example: 2021 Expiration Year isPennyPilot boolean example: true Is this contract part of the Penny Pilot program lastTradingDay integer ($int64) example: 1629504000000 milliseconds since epoch multiplier number ($double) example: 100 Option multiplier settlementType SettlementType string option contract settlement type AM or PM Enum: [ A, P ] strikePrice number ($double) example: 2300 Strike Price underlying string example: AMZN Aug 20 2021 2300 Put A company, index or fund name } |

## `QuoteEquity`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Equity security |
| `52WeekHigh` | number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks |
| `52WeekLow` | number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks |
| `askMICId` | string example: XNYS ask MIC code |
| `askPrice` | number ($double) example: 124.63 Current Best Ask Price |
| `askSize` | integer ($int32) example: 700 Number of shares for ask |
| `askTime` | integer ($int64) example: 1621376892336 Last ask time in milliseconds since Epoch |
| `bidMICId` | string example: XNYS bid MIC code |
| `bidPrice` | number ($double) example: 124.6 Current Best Bid Price |
| `bidSize` | integer ($int32) example: 300 Number of shares for bid |
| `bidTime` | integer ($int64) example: 1621376892336 Last bid time in milliseconds since Epoch |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `highPrice` | number ($double) example: 126.99 Day's high trade price |
| `lastMICId` | string example: XNYS Last MIC Code |
| `lastPrice` | number ($double) example: 122.3 |
| `lastSize` | integer ($int32) example: 100 Number of shares traded with last trade |
| `lowPrice` | number ($double) Day's low trade price |
| `mark` | number ($double) example: 52.93 Mark price |
| `markChange` | number ($double) example: -0.01 Mark Price change |
| `markPercentChange` | number ($double) example: -0.0189 Mark Price percent change |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `openPrice` | number ($double) example: 52.8 Price at market open |
| `quoteTime` | integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch |
| `securityStatus` | string example: Normal Status of security |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |
| `volatility` | number ($double) example: 0.0094 Option Risk/Volatility Measurement |

## `QuoteForex`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Forex security |
| `52WeekHigh` | number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks |
| `52WeekLow` | number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks |
| `askPrice` | number ($double) example: 124.63 Current Best Ask Price |
| `askSize` | integer ($int32) example: 700 Number of shares for ask |
| `bidPrice` | number ($double) example: 124.6 Current Best Bid Price |
| `bidSize` | integer ($int32) example: 300 Number of shares for bid |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `highPrice` | number ($double) example: 126.99 Day's high trade price |
| `lastPrice` | number ($double) example: 122.3 |
| `lastSize` | integer ($int32) example: 100 Number of shares traded with last trade |
| `lowPrice` | number ($double) example: 52.74 Day's low trade price |
| `mark` | number ($double) example: 52.93 Mark price |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `openPrice` | number ($double) example: 52.8 Price at market open |
| `quoteTime` | integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch |
| `securityStatus` | string example: Normal Status of security |
| `tick` | number ($double) example: 0 Tick Price |
| `tickAmount` | number ($double) example: 0 Tick Amount |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |

## `QuoteFuture`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Future security |
| `askMICId` | string example: XNYS ask MIC code |
| `askPrice` | number ($double) example: 4083.25 Current Best Ask Price |
| `askSize` | integer ($int32) example: 36 Number of shares for ask |
| `askTime` | integer ($int64) example: 1621376892336 Last ask time in milliseconds since Epoch |
| `bidMICId` | string example: XNYS bid MIC code |
| `bidPrice` | number ($double) example: 4083 Current Best Bid Price |
| `bidSize` | integer ($int32) example: 18 Number of shares for bid |
| `bidTime` | integer ($int64) example: 1621376892336 Last bid time in milliseconds since Epoch |
| `closePrice` | number ($double) example: 4123 Previous day's closing price |
| `futurePercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `highPrice` | number ($double) example: 4123 Day's high trade price |
| `lastMICId` | string example: XNYS Last MIC Code |
| `lastPrice` | number ($double) example: 4083 |
| `lastSize` | integer ($int32) example: 7 Number of shares traded with last trade |
| `lowPrice` | number ($double) example: 4075.5 Day's low trade price |
| `mark` | number ($double) example: 4083 Mark price |
| `netChange` | number ($double) example: -40 Current Last-Prev Close |
| `openInterest` | integer ($int32) example: 2517139 Open interest |
| `openPrice` | number ($double) example: 4114 Price at market open |
| `quoteTime` | integer ($int64) example: 1621427004585 Last quote time in milliseconds since Epoch |
| `quotedInSession` | boolean example: false quoted during trading session |
| `securityStatus` | string example: Normal Status of security |
| `settleTime` | integer ($int64) example: 1621376892336 settlement time in milliseconds since Epoch |
| `tick` | number ($double) example: 0.25 Tick Price |
| `tickAmount` | number ($double) example: 12.5 Tick Amount |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |

## `QuoteFutureOption`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Option security |
| `askMICId` | string example: XNYS ask MIC code |
| `askPrice` | number ($double) example: 124.63 Current Best Ask Price |
| `askSize` | integer ($int32) example: 700 Number of shares for ask |
| `bidMICId` | string example: XNYS bid MIC code |
| `bidPrice` | number ($double) example: 124.6 Current Best Bid Price |
| `bidSize` | integer ($int32) example: 300 Number of shares for bid |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `highPrice` | number ($double) example: 126.99 Day's high trade price |
| `lastMICId` | string example: XNYS Last MIC Code |
| `lastPrice` | number ($double) example: 122.3 |
| `lastSize` | integer ($int32) example: 100 Number of shares traded with last trade |
| `lowPrice` | number ($double) example: 52.74 Day's low trade price |
| `mark` | number ($double) example: 52.93 Mark price |
| `markChange` | number ($double) example: -0.04 Mark Price change |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `openInterest` | integer ($int32) example: 317 Open Interest |
| `openPrice` | number ($double) example: 52.8 Price at market open |
| `quoteTime` | integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch |
| `securityStatus` | string example: Normal Status of security |
| `settlemetPrice` | number ($double) example: 52.8 Price at market open |
| `tick` | number ($double) example: 0 Tick Price |
| `tickAmount` | number ($double) example: 0 Tick Amount |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |

## `QuoteIndex`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Index security |
| `52WeekHigh` | number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks |
| `52WeekLow` | number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `highPrice` | number ($double) example: 126.99 Day's high trade price |
| `lastPrice` | number ($double) example: 122.3 |
| `lowPrice` | number ($double) example: 52.74 Day's low trade price |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `openPrice` | number ($double) example: 52.8 Price at market open |
| `securityStatus` | string example: Normal Status of security |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |

## `QuoteMutualFund`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Mutual Fund security |
| `52WeekHigh` | number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks |
| `52WeekLow` | number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `nAV` | number ($double) example: 126.99 Net Asset Value |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `securityStatus` | string example: Normal Status of security |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |

## `QuoteOption`

| Field | Details |
| --- | --- |
| `description:` | Quote data of Option security |
| `52WeekHigh` | number ($double) example: 145.09 Higest price traded in the past 12 months, or 52 weeks |
| `52WeekLow` | number ($double) example: 77.581 Lowest price traded in the past 12 months, or 52 weeks |
| `askPrice` | number ($double) example: 124.63 Current Best Ask Price |
| `askSize` | integer ($int32) example: 700 Number of shares for ask |
| `bidPrice` | number ($double) example: 124.6 Current Best Bid Price |
| `bidSize` | integer ($int32) example: 300 Number of shares for bid |
| `closePrice` | number ($double) example: 126.27 Previous day's closing price |
| `delta` | number ($double) example: -0.0407 Delta Value |
| `gamma` | number ($double) example: 0.0001 Gamma Value |
| `highPrice` | number ($double) example: 126.99 Day's high trade price |
| `indAskPrice` | number ($double) example: 126.99 Indicative Ask Price applicable only for Indicative Option Symbols |
| `indBidPrice` | number ($double) example: 126.99 Indicative Bid Price applicable only for Indicative Option Symbols |
| `indQuoteTime` | integer ($int64) example: 126.99 Indicative Quote Time in milliseconds since Epoch applicable only for Indicative Option Symbols |
| `impliedYield` | number ($double) example: -0.0067 Implied Yield |
| `lastPrice` | number ($double) example: 122.3 |
| `lastSize` | integer ($int32) example: 100 Number of shares traded with last trade |
| `lowPrice` | number ($double) example: 52.74 Day's low trade price |
| `mark` | number ($double) example: 52.93 Mark price |
| `markChange` | number ($double) example: -0.01 Mark Price change |
| `markPercentChange` | number ($double) example: -0.0189 Mark Price percent change |
| `moneyIntrinsicValue` | number ($double) example: -947.96 Money Intrinsic Value |
| `netChange` | number ($double) example: -0.04 Current Last-Prev Close |
| `netPercentChange` | number ($double) example: -0.0756 Net Percentage Change |
| `openInterest` | number ($double) example: 317 Open Interest |
| `openPrice` | number ($double) example: 52.8 Price at market open |
| `quoteTime` | integer ($int64) example: 1621376892336 Last quote time in milliseconds since Epoch |
| `rho` | number ($double) example: -0.3732 Rho Value |
| `securityStatus` | string example: Normal Status of security |
| `theoreticalOptionValue` | number ($double) example: 12.275 Theoretical option Value |
| `theta` | number ($double) example: -0.315 Theta Value |
| `timeValue` | number ($double) example: 12.22 Time Value |
| `totalVolume` | integer ($int64) example: 20171188 Aggregated shares traded throughout the day, including pre/post market hours. |
| `tradeTime` | integer ($int64) example: 1621376731304 Last trade time in milliseconds since Epoch |
| `underlyingPrice` | number ($double) example: 3247.96 Underlying Price |
| `vega` | number ($double) example: 1.4455 Vega Value |
| `volatility` | number ($double) example: 0.0094 Option Risk/Volatility Measurement |

## `QuoteRequest`

| Field | Details |
| --- | --- |
| `description:` | Request one or more quote data in POST body |
| `cusips` | [] example: List [ 808524680, 594918104 ] List of cusip, max of 500 of symbols+cusip+ssids string |
| `fields` | string example: quote,reference comma separated list of nodes in each quote possible values are quote,fundamental,reference,extended,regular. Dont send this attribute for full response. |
| `ssids` | [] example: List [ 1516105793, 34621523 ] List of Schwab securityid[SSID], max of 500 of symbols+cusip+ssids integer ($int64) maximum: 9999999999 minimum: 1 |
| `symbols` | [] example: List [ "MRAD", "EATOF", "EBIZ", "AAPL", "BAC", "AAAHX", "AAAIX", "$DJI", "$SPX", "MVEN", "SOBS", "TOITF", "CNSWF", "AMZN 230317C01360000", "DJX 231215C00290000", "/ESH23", "./ADUF23C0.55", "AUD/CAD" ] List of symbols, max of 500 of symbols+cusip+ssids string |
| `realtime` | boolean example: true Get realtime quotes and skip entitlement check Enum: [ true, false ] |
| `indicative` | boolean example: true Include indicative symbol quotes for all ETF symbols in request. If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV Enum: [ true, false ] |

## `QuoteResponse`

| Field | Details |
| --- | --- |
| `description:` | a (symbol, QuoteResponse) map. is an example key SCHW |
| `< * >:` | #/components/schemas/QuoteResponseObject QuoteResponseObject { oneOf -> #/components/schemas/EquityResponse EquityResponse { description: Quote info of Equity security assetMainType AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] assetSubType EquityAssetSubType string nullable: true Asset Sub Type (only there if applicable) Enum: [ COE, PRF, ADR, GDR, CEF, ETF, ETN, UIT, WAR, RGT, ] ssid integer ($int64) example: 1234567890 SSID of instrument symbol string example: AAPL Symbol of instrument realtime boolean example: true is quote realtime quoteType QuoteType string nullable: true NBBO - realtime, NFL - Non-fee liable quote. Enum: [ NBBO, NFL, ] extended #/components/schemas/ExtendedMarket ExtendedMarket { description: Quote data for extended hours askPrice number ($double) example: 124.85 Extended market ask price askSize integer ($int32) example: 51771 Extended market ask size bidPrice number ($double) example: 124.85 Extended market bid price bidSize integer ($int32) example: 51771 Extended market bid size lastPrice number ($double) example: 124.85 Extended market last price lastSize integer ($int32) example: 51771 Regular market last size mark number ($double) example: 1.1246 mark price quoteTime integer ($int64) example: 1621368000400 Extended market quote time in milliseconds since Epoch totalVolume number ($int64) example: 12345 Total volume tradeTime integer ($int64) example: 1621368000400 Extended market trade time in milliseconds since Epoch } fundamental #/components/schemas/Fundamental Fundamental { description: Fundamentals of a security avg10DaysVolume number ($double) Average 10 day volume avg1YearVolume number ($double) Average 1 day volume declarationDate string ($date-time) example: 2021-04-28T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Declaration date in yyyy-mm-ddThh:mm:ssZ divAmount number ($double) example: 0.88 Dividend Amount divExDate string ($yyyy-MM-dd'T'HH:mm:ssZ) example: 2021-05-07T00:00:00Z Dividend date in yyyy-mm-ddThh:mm:ssZ divFreq DivFreq integer nullable: true Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly Enum: [ 1, 2, 3, 4, 6, 11, 12, ] divPayAmount number ($double) example: 0.22 Dividend Pay Amount divPayDate string ($date-time) example: 2021-05-13T00:00:00Z pa… |

## `QuoteResponseObject`

| Field | Details |
| --- | --- |
| `oneOf ->` | #/components/schemas/EquityResponse EquityResponse { description: Quote info of Equity security assetMainType AssetMainType string Instrument's asset type Enum: [ BOND, EQUITY, FOREX, FUTURE, FUTURE_OPTION, INDEX, MUTUAL_FUND, OPTION ] assetSubType EquityAssetSubType string nullable: true Asset Sub Type (only there if applicable) Enum: [ COE, PRF, ADR, GDR, CEF, ETF, ETN, UIT, WAR, RGT, ] ssid integer ($int64) example: 1234567890 SSID of instrument symbol string example: AAPL Symbol of instrument realtime boolean example: true is quote realtime quoteType QuoteType string nullable: true NBBO - realtime, NFL - Non-fee liable quote. Enum: [ NBBO, NFL, ] extended #/components/schemas/ExtendedMarket ExtendedMarket { description: Quote data for extended hours askPrice number ($double) example: 124.85 Extended market ask price askSize integer ($int32) example: 51771 Extended market ask size bidPrice number ($double) example: 124.85 Extended market bid price bidSize integer ($int32) example: 51771 Extended market bid size lastPrice number ($double) example: 124.85 Extended market last price lastSize integer ($int32) example: 51771 Regular market last size mark number ($double) example: 1.1246 mark price quoteTime integer ($int64) example: 1621368000400 Extended market quote time in milliseconds since Epoch totalVolume number ($int64) example: 12345 Total volume tradeTime integer ($int64) example: 1621368000400 Extended market trade time in milliseconds since Epoch } fundamental #/components/schemas/Fundamental Fundamental { description: Fundamentals of a security avg10DaysVolume number ($double) Average 10 day volume avg1YearVolume number ($double) Average 1 day volume declarationDate string ($date-time) example: 2021-04-28T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Declaration date in yyyy-mm-ddThh:mm:ssZ divAmount number ($double) example: 0.88 Dividend Amount divExDate string ($yyyy-MM-dd'T'HH:mm:ssZ) example: 2021-05-07T00:00:00Z Dividend date in yyyy-mm-ddThh:mm:ssZ divFreq DivFreq integer nullable: true Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly Enum: [ 1, 2, 3, 4, 6, 11, 12, ] divPayAmount number ($double) example: 0.22 Dividend Pay Amount divPayDate string ($date-time) example: 2021-05-13T00:00:00Z pattern: yyyy-MM-dd'T'HH:mm:ssZ Dividend pay date in yyyy-mm-ddThh:mm:ssZ… |

## `ReferenceEquity`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Equity security |
| `cusip` | string example: A23456789 CUSIP of Instrument |
| `description` | string example: Apple Inc. - Common Stock Description of Instrument |
| `exchange` | string example: q Exchange Code |
| `exchangeName` | string Exchange Name |
| `fsiDesc` | string maxLength: 50 FSI Desc |
| `htbQuantity` | integer ($int32) example: 100 Hard to borrow quantity. |
| `htbRate` | number ($double) example: 4.5 Hard to borrow rate. |
| `isHardToBorrow` | boolean example: false is Hard to borrow security. |
| `isShortable` | boolean example: false is shortable security. |
| `otcMarketTier` | string maxLength: 10 OTC Market Tier |

## `ReferenceForex`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Forex security |
| `description` | string example: Euro/USDollar Spot Description of Instrument |
| `exchange` | string example: q Exchange Code |
| `exchangeName` | string Exchange Name |
| `isTradable` | boolean example: true is FOREX tradable |
| `marketMaker` | string Market marker |
| `product` | string example: null Product name |
| `tradingHours` | string Trading hours |

## `ReferenceFuture`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Future security |
| `description` | string example: E-mini S&P 500 Index Futures,Jun-2021,ETH Description of Instrument |
| `exchange` | string example: q Exchange Code |
| `exchangeName` | string Exchange Name |
| `futureActiveSymbol` | string example: /ESM21 Active symbol |
| `futureExpirationDate` | number ($int64) example: 1623988800000 Future expiration date in milliseconds since epoch |
| `futureIsActive` | boolean example: true Future is active |
| `futureMultiplier` | number ($double) example: 50 Future multiplier |
| `futurePriceFormat` | string example: D,D Price format |
| `futureSettlementPrice` | number ($double) example: 4123 Future Settlement Price |
| `futureTradingHours` | string example: GLBX(de=1640;0=-1700151515301600;1=r-17001515r15301600d-15551640;7=d-16401555) Trading Hours |
| `product` | string example: /ES Futures product symbol |

## `ReferenceFutureOption`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Future Option security |
| `contractType` | ContractType string Indicates call or put Enum: [ P, C ] |
| `description` | string example: AMZN Aug 20 2021 2300 Put Description of Instrument |
| `exchange` | string example: q Exchange Code |
| `exchangeName` | string Exchange Name |
| `multiplier` | number ($double) example: 100 Option multiplier |
| `expirationDate` | integer ($int64) date of expiration in long |
| `expirationStyle` | string Style of expiration |
| `strikePrice` | number ($double) example: 2300 Strike Price |
| `underlying` | string example: AMZN Aug 20 2021 2300 Put A company, index or fund name |

## `ReferenceIndex`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Index security |
| `description` | string example: DOW JONES 30 INDUSTRIALS Description of Instrument |
| `exchange` | string example: q Exchange Code |
| `exchangeName` | string Exchange Name |

## `ReferenceMutualFund`

| Field | Details |
| --- | --- |
| `description:` | Reference data of MutualFund security |
| `cusip` | string example: A23456789 CUSIP of Instrument |
| `description` | string example: Apple Inc. - Common Stock Description of Instrument |
| `exchange` | string default: m Exchange Code |
| `exchangeName` | string default: MUTUAL_FUND Exchange Name |

## `ReferenceOption`

| Field | Details |
| --- | --- |
| `description:` | Reference data of Option security |
| `contractType` | ContractType string Indicates call or put Enum: [ P, C ] |
| `cusip` | string example: 0AMZN.TK12300000 CUSIP of Instrument |
| `daysToExpiration` | integer ($int32) example: 94 Days to Expiration |
| `deliverables` | string example: $6024.37 cash in lieu of shares, 212 shares of AZN Unit of trade |
| `description` | string example: AMZN Aug 20 2021 2300 Put Description of Instrument |
| `exchange` | string default: o Exchange Code |
| `exchangeName` | string Exchange Name |
| `exerciseType` | ExerciseType string option contract exercise type America or European Enum: [ A, E ] |
| `expirationDay` | integer ($int32) example: 20 maximum: 31 minimum: 1 Expiration Day |
| `expirationMonth` | integer ($int32) example: 8 maximum: 12 minimum: 1 Expiration Month |
| `expirationType` | ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] |
| `expirationYear` | integer ($int32) example: 2021 Expiration Year |
| `isPennyPilot` | boolean example: true Is this contract part of the Penny Pilot program |
| `lastTradingDay` | integer ($int64) example: 1629504000000 milliseconds since epoch |
| `multiplier` | number ($double) example: 100 Option multiplier |
| `settlementType` | SettlementType string option contract settlement type AM or PM Enum: [ A, P ] |
| `strikePrice` | number ($double) example: 2300 Strike Price |
| `underlying` | string example: AMZN Aug 20 2021 2300 Put A company, index or fund name |

## `RegularMarket`

| Field | Details |
| --- | --- |
| `description:` | Market info of security |
| `regularMarketLastPrice` | number ($double) example: 124.85 Regular market last price |
| `regularMarketLastSize` | integer ($int32) example: 51771 Regular market last size |
| `regularMarketNetChange` | number ($double) example: -1.42 Regular market net change |
| `regularMarketPercentChange` | number ($double) example: -1.1246 Regular market percent change |
| `regularMarketTradeTime` | integer ($int64) example: 1621368000400 Regular market trade time in milliseconds since Epoch |

## `AssetMainType`

No top-level properties exposed in the expanded schema block.

## `EquityAssetSubType`

No top-level properties exposed in the expanded schema block.

## `MutualFundAssetSubType`

No top-level properties exposed in the expanded schema block.

## `ContractType`

No top-level properties exposed in the expanded schema block.

## `SettlementType`

No top-level properties exposed in the expanded schema block.

## `ExpirationType`

No top-level properties exposed in the expanded schema block.

## `FundStrategy`

No top-level properties exposed in the expanded schema block.

## `ExerciseType`

No top-level properties exposed in the expanded schema block.

## `DivFreq`

No top-level properties exposed in the expanded schema block.

## `QuoteType`

No top-level properties exposed in the expanded schema block.

## `ErrorResponse`

| Field | Details |
| --- | --- |
| `errors` | [] #/components/schemas/Error Error { id string ($uuid) readOnly: true example: 9821320c-8500-4edf-bd46-a9299c13d2e0 Unique error id. status string readOnly: true example: 400 The HTTP status code . Enum: [ 400, 401, 404, 500 ] title string readOnly: true example: Missing header Short error description. detail string readOnly: true example: Search combination should not exceed 500. Detailed error description. source #/components/schemas/ErrorSource ErrorSource { description: Who is responsible for triggering these errors. pointer [] readOnly: true example: List [ "/data/attributes/symbols", "/data/attributes/cusips", "/data/attributes/ssids" ] list of attributes which lead to this error message. string parameter string readOnly: true example: fields parameter name which lead to this error message. header string readOnly: true example: Schwab-Client-CorrelId header name which lead to this error message. } } |

## `Error`

| Field | Details |
| --- | --- |
| `id` | string ($uuid) readOnly: true example: 9821320c-8500-4edf-bd46-a9299c13d2e0 Unique error id. |
| `status` | string readOnly: true example: 400 The HTTP status code . Enum: [ 400, 401, 404, 500 ] |
| `title` | string readOnly: true example: Missing header Short error description. |
| `detail` | string readOnly: true example: Search combination should not exceed 500. Detailed error description. |
| `source` | #/components/schemas/ErrorSource ErrorSource { description: Who is responsible for triggering these errors. pointer [] readOnly: true example: List [ "/data/attributes/symbols", "/data/attributes/cusips", "/data/attributes/ssids" ] list of attributes which lead to this error message. string parameter string readOnly: true example: fields parameter name which lead to this error message. header string readOnly: true example: Schwab-Client-CorrelId header name which lead to this error message. } |

## `ErrorSource`

| Field | Details |
| --- | --- |
| `description:` | Who is responsible for triggering these errors. |
| `pointer` | [] readOnly: true example: List [ "/data/attributes/symbols", "/data/attributes/cusips", "/data/attributes/ssids" ] list of attributes which lead to this error message. string |
| `parameter` | string readOnly: true example: fields parameter name which lead to this error message. |
| `header` | string readOnly: true example: Schwab-Client-CorrelId header name which lead to this error message. |

## `OptionChain`

| Field | Details |
| --- | --- |
| `symbol` | string |
| `status` | string |
| `underlying` | #/components/schemas/Underlying Underlying { ask number ($double) askSize integer ($int32) bid number ($double) bidSize integer ($int32) change number ($double) close number ($double) delayed boolean description string exchangeName string Enum: [ IND, ASE, NYS, NAS, NAP, PAC, OPR, BATS ] fiftyTwoWeekHigh number ($double) fiftyTwoWeekLow number ($double) highPrice number ($double) last number ($double) lowPrice number ($double) mark number ($double) markChange number ($double) markPercentChange number ($double) openPrice number ($double) percentChange number ($double) quoteTime integer ($int64) symbol string totalVolume integer ($int64) tradeTime integer ($int64) } |
| `strategy` | string Enum: [ SINGLE, ANALYTICAL, COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, BUTTERFLY, CONDOR, DIAGONAL, COLLAR, ROLL ] |
| `interval` | number ($double) |
| `isDelayed` | boolean |
| `isIndex` | boolean |
| `daysToExpiration` | number ($double) |
| `interestRate` | number ($double) |
| `underlyingPrice` | number ($double) |
| `volatility` | number ($double) |
| `callExpDateMap` | { < * >: #/components/schemas/OptionContractMap OptionContractMap { < * >: #/components/schemas/OptionContract OptionContract { putCall string Enum: [ PUT, CALL ] symbol string description string exchangeName string bidPrice number ($double) askPrice number ($double) lastPrice number ($double) markPrice number ($double) bidSize integer ($int32) askSize integer ($int32) lastSize integer ($int32) highPrice number ($double) lowPrice number ($double) openPrice number ($double) closePrice number ($double) totalVolume integer ($int32) tradeDate number ($integer) quoteTimeInLong integer ($int32) tradeTimeInLong integer ($int32) netChange number ($double) volatility number ($double) delta number ($double) gamma number ($double) theta number ($double) vega number ($double) rho number ($double) timeValue number ($double) openInterest number ($double) isInTheMoney boolean theoreticalOptionValue number ($double) theoreticalVolatility number ($double) isMini boolean isNonStandard boolean optionDeliverablesList [] #/components/schemas/OptionDeliverables OptionDeliverables { symbol string assetType string deliverableUnits string currencyType string } strikePrice number ($double) expirationDate string daysToExpiration number ($int) expirationType ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] lastTradingDay number ($long) multiplier number ($double) settlementType SettlementType string option contract settlement type AM or PM Enum: [ A, P ] deliverableNote string isIndexOption boolean percentChange number ($double) markChange number ($double) markPercentChange number ($double) isPennyPilot boolean intrinsicValue number ($double) optionRoot string } } } |
| `putExpDateMap` | { < * >: #/components/schemas/OptionContractMap OptionContractMap { < * >: #/components/schemas/OptionContract OptionContract { putCall string Enum: [ PUT, CALL ] symbol string description string exchangeName string bidPrice number ($double) askPrice number ($double) lastPrice number ($double) markPrice number ($double) bidSize integer ($int32) askSize integer ($int32) lastSize integer ($int32) highPrice number ($double) lowPrice number ($double) openPrice number ($double) closePrice number ($double) totalVolume integer ($int32) tradeDate number ($integer) quoteTimeInLong integer ($int32) tradeTimeInLong integer ($int32) netChange number ($double) volatility number ($double) delta number ($double) gamma number ($double) theta number ($double) vega number ($double) rho number ($double) timeValue number ($double) openInterest number ($double) isInTheMoney boolean theoreticalOptionValue number ($double) theoreticalVolatility number ($double) isMini boolean isNonStandard boolean optionDeliverablesList [] #/components/schemas/OptionDeliverables OptionDeliverables { symbol string assetType string deliverableUnits string currencyType string } strikePrice number ($double) expirationDate string daysToExpiration number ($int) expirationType ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] lastTradingDay number ($long) multiplier number ($double) settlementType SettlementType string option contract settlement type AM or PM Enum: [ A, P ] deliverableNote string isIndexOption boolean percentChange number ($double) markChange number ($double) markPercentChange number ($double) isPennyPilot boolean intrinsicValue number ($double) optionRoot string } } } |

## `OptionContractMap`

| Field | Details |
| --- | --- |
| `< * >:` | #/components/schemas/OptionContract OptionContract { putCall string Enum: [ PUT, CALL ] symbol string description string exchangeName string bidPrice number ($double) askPrice number ($double) lastPrice number ($double) markPrice number ($double) bidSize integer ($int32) askSize integer ($int32) lastSize integer ($int32) highPrice number ($double) lowPrice number ($double) openPrice number ($double) closePrice number ($double) totalVolume integer ($int32) tradeDate number ($integer) quoteTimeInLong integer ($int32) tradeTimeInLong integer ($int32) netChange number ($double) volatility number ($double) delta number ($double) gamma number ($double) theta number ($double) vega number ($double) rho number ($double) timeValue number ($double) openInterest number ($double) isInTheMoney boolean theoreticalOptionValue number ($double) theoreticalVolatility number ($double) isMini boolean isNonStandard boolean optionDeliverablesList [] #/components/schemas/OptionDeliverables OptionDeliverables { symbol string assetType string deliverableUnits string currencyType string } strikePrice number ($double) expirationDate string daysToExpiration number ($int) expirationType ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] lastTradingDay number ($long) multiplier number ($double) settlementType SettlementType string option contract settlement type AM or PM Enum: [ A, P ] deliverableNote string isIndexOption boolean percentChange number ($double) markChange number ($double) markPercentChange number ($double) isPennyPilot boolean intrinsicValue number ($double) optionRoot string } |

## `Underlying`

| Field | Details |
| --- | --- |
| `ask` | number ($double) |
| `askSize` | integer ($int32) |
| `bid` | number ($double) |
| `bidSize` | integer ($int32) |
| `change` | number ($double) |
| `close` | number ($double) |
| `delayed` | boolean |
| `description` | string |
| `exchangeName` | string Enum: [ IND, ASE, NYS, NAS, NAP, PAC, OPR, BATS ] |
| `fiftyTwoWeekHigh` | number ($double) |
| `fiftyTwoWeekLow` | number ($double) |
| `highPrice` | number ($double) |
| `last` | number ($double) |
| `lowPrice` | number ($double) |
| `mark` | number ($double) |
| `markChange` | number ($double) |
| `markPercentChange` | number ($double) |
| `openPrice` | number ($double) |
| `percentChange` | number ($double) |
| `quoteTime` | integer ($int64) |
| `symbol` | string |
| `totalVolume` | integer ($int64) |
| `tradeTime` | integer ($int64) |

## `OptionDeliverables`

| Field | Details |
| --- | --- |
| `symbol` | string |
| `assetType` | string |
| `deliverableUnits` | string |
| `currencyType` | string |

## `OptionContract`

| Field | Details |
| --- | --- |
| `putCall` | string Enum: [ PUT, CALL ] |
| `symbol` | string |
| `description` | string |
| `exchangeName` | string |
| `bidPrice` | number ($double) |
| `askPrice` | number ($double) |
| `lastPrice` | number ($double) |
| `markPrice` | number ($double) |
| `bidSize` | integer ($int32) |
| `askSize` | integer ($int32) |
| `lastSize` | integer ($int32) |
| `highPrice` | number ($double) |
| `lowPrice` | number ($double) |
| `openPrice` | number ($double) |
| `closePrice` | number ($double) |
| `totalVolume` | integer ($int32) |
| `tradeDate` | number ($integer) |
| `quoteTimeInLong` | integer ($int32) |
| `tradeTimeInLong` | integer ($int32) |
| `netChange` | number ($double) |
| `volatility` | number ($double) |
| `delta` | number ($double) |
| `gamma` | number ($double) |
| `theta` | number ($double) |
| `vega` | number ($double) |
| `rho` | number ($double) |
| `timeValue` | number ($double) |
| `openInterest` | number ($double) |
| `isInTheMoney` | boolean |
| `theoreticalOptionValue` | number ($double) |
| `theoreticalVolatility` | number ($double) |
| `isMini` | boolean |
| `isNonStandard` | boolean |
| `optionDeliverablesList` | [] #/components/schemas/OptionDeliverables OptionDeliverables { symbol string assetType string deliverableUnits string currencyType string } |
| `strikePrice` | number ($double) |
| `expirationDate` | string |
| `daysToExpiration` | number ($int) |
| `expirationType` | ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] |
| `lastTradingDay` | number ($long) |
| `multiplier` | number ($double) |
| `settlementType` | SettlementType string option contract settlement type AM or PM Enum: [ A, P ] |
| `deliverableNote` | string |
| `isIndexOption` | boolean |
| `percentChange` | number ($double) |
| `markChange` | number ($double) |
| `markPercentChange` | number ($double) |
| `isPennyPilot` | boolean |
| `intrinsicValue` | number ($double) |
| `optionRoot` | string |

## `ExpirationChain`

| Field | Details |
| --- | --- |
| `status` | string |
| `expirationList` | [] #/components/schemas/Expiration Expiration { description: expiration type daysToExpiration integer ($int32) expiration string expirationType ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] standard boolean settlementType SettlementType string option contract settlement type AM or PM Enum: [ A, P ] optionRoots string } |

## `Expiration`

| Field | Details |
| --- | --- |
| `description:` | expiration type |
| `daysToExpiration` | integer ($int32) |
| `expiration` | string |
| `expirationType` | ExpirationType string M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options). Enum: [ M, Q, S, W ] |
| `standard` | boolean |
| `settlementType` | SettlementType string option contract settlement type AM or PM Enum: [ A, P ] |
| `optionRoots` | string |
