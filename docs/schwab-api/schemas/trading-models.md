# Schwab Accounts and Trading Models

Source: `trading-production-specs.html`

Model count: `84`

## `AccountNumberHash`

| Field | Details |
| --- | --- |
| `accountNumber` | string |
| `hashValue` | string |

## `session`

No top-level properties exposed in the expanded schema block.

## `duration`

No top-level properties exposed in the expanded schema block.

## `orderType`

No top-level properties exposed in the expanded schema block.

## `orderTypeRequest`

No top-level properties exposed in the expanded schema block.

## `complexOrderStrategyType`

No top-level properties exposed in the expanded schema block.

## `requestedDestination`

No top-level properties exposed in the expanded schema block.

## `stopPriceLinkBasis`

No top-level properties exposed in the expanded schema block.

## `stopPriceLinkType`

No top-level properties exposed in the expanded schema block.

## `stopPriceOffset`

No top-level properties exposed in the expanded schema block.

## `stopType`

No top-level properties exposed in the expanded schema block.

## `priceLinkBasis`

No top-level properties exposed in the expanded schema block.

## `priceLinkType`

No top-level properties exposed in the expanded schema block.

## `taxLotMethod`

No top-level properties exposed in the expanded schema block.

## `specialInstruction`

No top-level properties exposed in the expanded schema block.

## `orderStrategyType`

No top-level properties exposed in the expanded schema block.

## `status`

No top-level properties exposed in the expanded schema block.

## `amountIndicator`

No top-level properties exposed in the expanded schema block.

## `settlementInstruction`

No top-level properties exposed in the expanded schema block.

## `OrderStrategy`

| Field | Details |
| --- | --- |
| `accountNumber` | string |
| `advancedOrderType` | string Enum: [ NONE, OTO, OCO, OTOCO, OT2OCO, OT3OCO, BLAST_ALL, OTA, PAIR ] |
| `closeTime` | string ($date-time) |
| `enteredTime` | string ($date-time) |
| `orderBalance` | #/components/schemas/OrderBalance OrderBalance { orderValue number ($double) projectedAvailableFund number ($double) projectedBuyingPower number ($double) projectedCommission number ($double) } |
| `orderStrategyType` | orderStrategyType string Enum: [ SINGLE, CANCEL, RECALL, PAIR, FLATTEN, TWO_DAY_SWAP, BLAST_ALL, OCO, TRIGGER ] |
| `orderVersion` | number |
| `session` | session string Enum: [ NORMAL, AM, PM, SEAMLESS ] |
| `status` | apiOrderStatus string Enum: [ AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN ] |
| `allOrNone` | boolean |
| `discretionary` | boolean |
| `duration` | duration string Enum: [ DAY, GOOD_TILL_CANCEL, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, END_OF_WEEK, END_OF_MONTH, NEXT_END_OF_MONTH, UNKNOWN ] |
| `filledQuantity` | number ($double) |
| `orderType` | orderType string Enum: [ MARKET, LIMIT, STOP, STOP_LIMIT, TRAILING_STOP, CABINET, NON_MARKETABLE, MARKET_ON_CLOSE, EXERCISE, TRAILING_STOP_LIMIT, NET_DEBIT, NET_CREDIT, NET_ZERO, LIMIT_ON_CLOSE, UNKNOWN ] |
| `orderValue` | number ($double) |
| `price` | number ($double) |
| `quantity` | number ($double) |
| `remainingQuantity` | number ($double) |
| `sellNonMarginableFirst` | boolean |
| `settlementInstruction` | settlementInstruction string Enum: [ REGULAR, CASH, NEXT_DAY, UNKNOWN ] |
| `strategy` | complexOrderStrategyType string Enum: [ NONE, COVERED, VERTICAL, BACK_RATIO, CALENDAR, DIAGONAL, STRADDLE, STRANGLE, COLLAR_SYNTHETIC, BUTTERFLY, CONDOR, IRON_CONDOR, VERTICAL_ROLL, COLLAR_WITH_STOCK, DOUBLE_DIAGONAL, UNBALANCED_BUTTERFLY, UNBALANCED_CONDOR, UNBALANCED_IRON_CONDOR, UNBALANCED_VERTICAL_ROLL, MUTUAL_FUND_SWAP, CUSTOM ] |
| `amountIndicator` | amountIndicator string Enum: [ DOLLARS, SHARES, ALL_SHARES, PERCENTAGE, UNKNOWN ] |
| `orderLegs` | [] xml: OrderedMap { "name": "orderLeg", "wrapped": true } #/components/schemas/OrderLeg OrderLeg { askPrice number ($double) bidPrice number ($double) lastPrice number ($double) markPrice number ($double) projectedCommission number ($double) quantity number ($double) finalSymbol string legId number ($long) assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] instruction instruction string Enum: [ BUY, SELL, BUY_TO_COVER, SELL_SHORT, BUY_TO_OPEN, BUY_TO_CLOSE, SELL_TO_OPEN, SELL_TO_CLOSE, EXCHANGE, SELL_SHORT_EXEMPT ] } |

## `OrderLeg`

| Field | Details |
| --- | --- |
| `askPrice` | number ($double) |
| `bidPrice` | number ($double) |
| `lastPrice` | number ($double) |
| `markPrice` | number ($double) |
| `projectedCommission` | number ($double) |
| `quantity` | number ($double) |
| `finalSymbol` | string |
| `legId` | number ($long) |
| `assetType` | assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `instruction` | instruction string Enum: [ BUY, SELL, BUY_TO_COVER, SELL_SHORT, BUY_TO_OPEN, BUY_TO_CLOSE, SELL_TO_OPEN, SELL_TO_CLOSE, EXCHANGE, SELL_SHORT_EXEMPT ] |

## `OrderBalance`

| Field | Details |
| --- | --- |
| `orderValue` | number ($double) |
| `projectedAvailableFund` | number ($double) |
| `projectedBuyingPower` | number ($double) |
| `projectedCommission` | number ($double) |

## `OrderValidationResult`

| Field | Details |
| --- | --- |
| `alerts` | [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } |
| `accepts` | [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } |
| `rejects` | [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } |
| `reviews` | [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } |
| `warns` | [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } |

## `OrderValidationDetail`

| Field | Details |
| --- | --- |
| `validationRuleName` | string |
| `message` | string |
| `activityMessage` | string |
| `originalSeverity` | APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] |
| `overrideName` | string |
| `overrideSeverity` | APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] |

## `APIRuleAction`

No top-level properties exposed in the expanded schema block.

## `CommissionAndFee`

| Field | Details |
| --- | --- |
| `commission` | #/components/schemas/Commission Commission { commissionLegs [] #/components/schemas/CommissionLeg CommissionLeg { commissionValues [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } |
| `fee` | #/components/schemas/Fees Fees { feeLegs [] #/components/schemas/FeeLeg FeeLeg { feeValues [] #/components/schemas/FeeValue FeeValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } |
| `trueCommission` | #/components/schemas/Commission Commission { commissionLegs [] #/components/schemas/CommissionLeg CommissionLeg { commissionValues [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } |

## `Commission`

| Field | Details |
| --- | --- |
| `commissionLegs` | [] #/components/schemas/CommissionLeg CommissionLeg { commissionValues [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } |

## `CommissionLeg`

| Field | Details |
| --- | --- |
| `commissionValues` | [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } |

## `CommissionValue`

| Field | Details |
| --- | --- |
| `value` | number ($double) |
| `type` | FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] |

## `Fees`

| Field | Details |
| --- | --- |
| `feeLegs` | [] #/components/schemas/FeeLeg FeeLeg { feeValues [] #/components/schemas/FeeValue FeeValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } |

## `FeeLeg`

| Field | Details |
| --- | --- |
| `feeValues` | [] #/components/schemas/FeeValue FeeValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } |

## `FeeValue`

| Field | Details |
| --- | --- |
| `value` | number ($double) |
| `type` | FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] |

## `FeeType`

No top-level properties exposed in the expanded schema block.

## `Account`

| Field | Details |
| --- | --- |
| `securitiesAccount` | #/components/schemas/SecuritiesAccount SecuritiesAccount { oneOf -> #/components/schemas/MarginAccount MarginAccount { type string Enum: [ CASH, MARGIN ] accountNumber string roundTrips integer ($int32) isDayTrader boolean default: false isClosingOnlyRestricted boolean default: false pfcbFlag boolean default: false positions [] #/components/schemas/Position Position { shortQuantity number ($double) averagePrice number ($double) currentDayProfitLoss number ($double) currentDayProfitLossPercentage number ($double) longQuantity number ($double) settledLongQuantity number ($double) settledShortQuantity number ($double) agedQuantity number ($double) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOption… |

## `DateParam`

| Field | Details |
| --- | --- |
| `date` | string Valid ISO-8601 format is : yyyy-MM-dd'T'HH:mm:ss.SSSZ |

## `Order`

| Field | Details |
| --- | --- |
| `session` | session string Enum: [ NORMAL, AM, PM, SEAMLESS ] |
| `duration` | duration string Enum: [ DAY, GOOD_TILL_CANCEL, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, END_OF_WEEK, END_OF_MONTH, NEXT_END_OF_MONTH, UNKNOWN ] |
| `orderType` | orderType string Enum: [ MARKET, LIMIT, STOP, STOP_LIMIT, TRAILING_STOP, CABINET, NON_MARKETABLE, MARKET_ON_CLOSE, EXERCISE, TRAILING_STOP_LIMIT, NET_DEBIT, NET_CREDIT, NET_ZERO, LIMIT_ON_CLOSE, UNKNOWN ] |
| `cancelTime` | string ($date-time) |
| `complexOrderStrategyType` | complexOrderStrategyType string Enum: [ NONE, COVERED, VERTICAL, BACK_RATIO, CALENDAR, DIAGONAL, STRADDLE, STRANGLE, COLLAR_SYNTHETIC, BUTTERFLY, CONDOR, IRON_CONDOR, VERTICAL_ROLL, COLLAR_WITH_STOCK, DOUBLE_DIAGONAL, UNBALANCED_BUTTERFLY, UNBALANCED_CONDOR, UNBALANCED_IRON_CONDOR, UNBALANCED_VERTICAL_ROLL, MUTUAL_FUND_SWAP, CUSTOM ] |
| `quantity` | number ($double) |
| `filledQuantity` | number ($double) |
| `remainingQuantity` | number ($double) |
| `requestedDestination` | requestedDestination string Enum: [ INET, ECN_ARCA, CBOE, AMEX, PHLX, ISE, BOX, NYSE, NASDAQ, BATS, C2, AUTO ] |
| `destinationLinkName` | string |
| `releaseTime` | string ($date-time) |
| `stopPrice` | number ($double) |
| `stopPriceLinkBasis` | stopPriceLinkBasis string Enum: [ MANUAL, BASE, TRIGGER, LAST, BID, ASK, ASK_BID, MARK, AVERAGE ] |
| `stopPriceLinkType` | stopPriceLinkType string Enum: [ VALUE, PERCENT, TICK ] |
| `stopPriceOffset` | number ($double) |
| `stopType` | stopType string Enum: [ STANDARD, BID, ASK, LAST, MARK ] |
| `priceLinkBasis` | priceLinkBasis string Enum: [ MANUAL, BASE, TRIGGER, LAST, BID, ASK, ASK_BID, MARK, AVERAGE ] |
| `priceLinkType` | priceLinkType string Enum: [ VALUE, PERCENT, TICK ] |
| `price` | number ($double) |
| `taxLotMethod` | taxLotMethod string Enum: [ FIFO, LIFO, HIGH_COST, LOW_COST, AVERAGE_COST, SPECIFIC_LOT, LOSS_HARVESTER ] |
| `orderLegCollection` | [] xml: OrderedMap { "name": "orderLegCollection", "wrapped": true } #/components/schemas/OrderLegCollection OrderLegCollection { orderLegType string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] legId integer ($int64) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type str… |
| `activationPrice` | number ($double) |
| `specialInstruction` | specialInstruction string Enum: [ ALL_OR_NONE, DO_NOT_REDUCE, ALL_OR_NONE_DO_NOT_REDUCE ] |
| `orderStrategyType` | orderStrategyType string Enum: [ SINGLE, CANCEL, RECALL, PAIR, FLATTEN, TWO_DAY_SWAP, BLAST_ALL, OCO, TRIGGER ] |
| `orderId` | integer ($int64) |
| `cancelable` | boolean default: false |
| `editable` | boolean default: false |
| `status` | status string Enum: [ AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN ] |
| `enteredTime` | string ($date-time) |
| `closeTime` | string ($date-time) |
| `tag` | string |
| `accountNumber` | integer ($int64) |
| `orderActivityCollection` | [] xml: OrderedMap { "name": "orderActivity", "wrapped": true } #/components/schemas/OrderActivity OrderActivity { activityType string Enum: [ EXECUTION, ORDER_ACTION ] executionType string Enum: [ FILL ] quantity number ($double) orderRemainingQuantity number ($double) executionLegs [] xml: OrderedMap { "name": "executionLegs", "wrapped": true } #/components/schemas/ExecutionLeg ExecutionLeg { legId integer ($int64) price number ($double) quantity number ($double) mismarkedQuantity number ($double) instrumentId integer ($int64) time string ($date-time) } } |
| `replacingOrderCollection` | [] xml: OrderedMap { "name": "replacingOrder", "wrapped": true } { } |
| `childOrderStrategies` | [] xml: OrderedMap { "name": "childOrder", "wrapped": true } { } |
| `statusDescription` | string |

## `OrderRequest`

| Field | Details |
| --- | --- |
| `session` | session string Enum: [ NORMAL, AM, PM, SEAMLESS ] |
| `duration` | duration string Enum: [ DAY, GOOD_TILL_CANCEL, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, END_OF_WEEK, END_OF_MONTH, NEXT_END_OF_MONTH, UNKNOWN ] |
| `orderType` | orderTypeRequest string Same as orderType, but does not have UNKNOWN since this type is not allowed as an input Enum: [ MARKET, LIMIT, STOP, STOP_LIMIT, TRAILING_STOP, CABINET, NON_MARKETABLE, MARKET_ON_CLOSE, EXERCISE, TRAILING_STOP_LIMIT, NET_DEBIT, NET_CREDIT, NET_ZERO, LIMIT_ON_CLOSE ] |
| `cancelTime` | string ($date-time) |
| `complexOrderStrategyType` | complexOrderStrategyType string Enum: [ NONE, COVERED, VERTICAL, BACK_RATIO, CALENDAR, DIAGONAL, STRADDLE, STRANGLE, COLLAR_SYNTHETIC, BUTTERFLY, CONDOR, IRON_CONDOR, VERTICAL_ROLL, COLLAR_WITH_STOCK, DOUBLE_DIAGONAL, UNBALANCED_BUTTERFLY, UNBALANCED_CONDOR, UNBALANCED_IRON_CONDOR, UNBALANCED_VERTICAL_ROLL, MUTUAL_FUND_SWAP, CUSTOM ] |
| `quantity` | number ($double) |
| `filledQuantity` | number ($double) |
| `remainingQuantity` | number ($double) |
| `destinationLinkName` | string |
| `releaseTime` | string ($date-time) |
| `stopPrice` | number ($double) |
| `stopPriceLinkBasis` | stopPriceLinkBasis string Enum: [ MANUAL, BASE, TRIGGER, LAST, BID, ASK, ASK_BID, MARK, AVERAGE ] |
| `stopPriceLinkType` | stopPriceLinkType string Enum: [ VALUE, PERCENT, TICK ] |
| `stopPriceOffset` | number ($double) |
| `stopType` | stopType string Enum: [ STANDARD, BID, ASK, LAST, MARK ] |
| `priceLinkBasis` | priceLinkBasis string Enum: [ MANUAL, BASE, TRIGGER, LAST, BID, ASK, ASK_BID, MARK, AVERAGE ] |
| `priceLinkType` | priceLinkType string Enum: [ VALUE, PERCENT, TICK ] |
| `price` | number ($double) |
| `taxLotMethod` | taxLotMethod string Enum: [ FIFO, LIFO, HIGH_COST, LOW_COST, AVERAGE_COST, SPECIFIC_LOT, LOSS_HARVESTER ] |
| `orderLegCollection` | [] xml: OrderedMap { "name": "orderLegCollection", "wrapped": true } #/components/schemas/OrderLegCollection OrderLegCollection { orderLegType string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] legId integer ($int64) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type str… |
| `activationPrice` | number ($double) |
| `specialInstruction` | specialInstruction string Enum: [ ALL_OR_NONE, DO_NOT_REDUCE, ALL_OR_NONE_DO_NOT_REDUCE ] |
| `orderStrategyType` | orderStrategyType string Enum: [ SINGLE, CANCEL, RECALL, PAIR, FLATTEN, TWO_DAY_SWAP, BLAST_ALL, OCO, TRIGGER ] |
| `orderId` | integer ($int64) |
| `cancelable` | boolean default: false |
| `editable` | boolean default: false |
| `status` | status string Enum: [ AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN ] |
| `enteredTime` | string ($date-time) |
| `closeTime` | string ($date-time) |
| `accountNumber` | integer ($int64) |
| `orderActivityCollection` | [] xml: OrderedMap { "name": "orderActivity", "wrapped": true } #/components/schemas/OrderActivity OrderActivity { activityType string Enum: [ EXECUTION, ORDER_ACTION ] executionType string Enum: [ FILL ] quantity number ($double) orderRemainingQuantity number ($double) executionLegs [] xml: OrderedMap { "name": "executionLegs", "wrapped": true } #/components/schemas/ExecutionLeg ExecutionLeg { legId integer ($int64) price number ($double) quantity number ($double) mismarkedQuantity number ($double) instrumentId integer ($int64) time string ($date-time) } } |
| `replacingOrderCollection` | [] xml: OrderedMap { "name": "replacingOrder", "wrapped": true } { } |
| `childOrderStrategies` | [] xml: OrderedMap { "name": "childOrder", "wrapped": true } { } |
| `statusDescription` | string |

## `PreviewOrder`

| Field | Details |
| --- | --- |
| `orderId` | integer ($int64) |
| `orderStrategy` | #/components/schemas/OrderStrategy OrderStrategy { accountNumber string advancedOrderType string Enum: [ NONE, OTO, OCO, OTOCO, OT2OCO, OT3OCO, BLAST_ALL, OTA, PAIR ] closeTime string ($date-time) enteredTime string ($date-time) orderBalance #/components/schemas/OrderBalance OrderBalance { orderValue number ($double) projectedAvailableFund number ($double) projectedBuyingPower number ($double) projectedCommission number ($double) } orderStrategyType orderStrategyType string Enum: [ SINGLE, CANCEL, RECALL, PAIR, FLATTEN, TWO_DAY_SWAP, BLAST_ALL, OCO, TRIGGER ] orderVersion number session session string Enum: [ NORMAL, AM, PM, SEAMLESS ] status apiOrderStatus string Enum: [ AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN ] allOrNone boolean discretionary boolean duration duration string Enum: [ DAY, GOOD_TILL_CANCEL, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, END_OF_WEEK, END_OF_MONTH, NEXT_END_OF_MONTH, UNKNOWN ] filledQuantity number ($double) orderType orderType string Enum: [ MARKET, LIMIT, STOP, STOP_LIMIT, TRAILING_STOP, CABINET, NON_MARKETABLE, MARKET_ON_CLOSE, EXERCISE, TRAILING_STOP_LIMIT, NET_DEBIT, NET_CREDIT, NET_ZERO, LIMIT_ON_CLOSE, UNKNOWN ] orderValue number ($double) price number ($double) quantity number ($double) remainingQuantity number ($double) sellNonMarginableFirst boolean settlementInstruction settlementInstruction string Enum: [ REGULAR, CASH, NEXT_DAY, UNKNOWN ] strategy complexOrderStrategyType string Enum: [ NONE, COVERED, VERTICAL, BACK_RATIO, CALENDAR, DIAGONAL, STRADDLE, STRANGLE, COLLAR_SYNTHETIC, BUTTERFLY, CONDOR, IRON_CONDOR, VERTICAL_ROLL, COLLAR_WITH_STOCK, DOUBLE_DIAGONAL, UNBALANCED_BUTTERFLY, UNBALANCED_CONDOR, UNBALANCED_IRON_CONDOR, UNBALANCED_VERTICAL_ROLL, MUTUAL_FUND_SWAP, CUSTOM ] amountIndicator amountIndicator string Enum: [ DOLLARS, SHARES, ALL_SHARES, PERCENTAGE, UNKNOWN ] orderLegs [] xml: OrderedMap { "name": "orderLeg", "wrapped": true } #/components/schemas/OrderLeg OrderLeg { askPrice number ($double) bidPrice number ($double) lastPrice number ($double) markPrice number ($double) projectedCommission number ($double) quantity number ($double) finalSymbol string legId number ($long) assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE… |
| `orderValidationResult` | #/components/schemas/OrderValidationResult OrderValidationResult { alerts [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } accepts [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } rejects [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } reviews [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } warns [] #/components/schemas/OrderValidationDetail OrderValidationDetail { validationRuleName string message string activityMessage string originalSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] overrideName string overrideSeverity APIRuleAction string Enum: [ ACCEPT, ALERT, REJECT, REVIEW, UNKNOWN ] } } |
| `commissionAndFee` | #/components/schemas/CommissionAndFee CommissionAndFee { commission #/components/schemas/Commission Commission { commissionLegs [] #/components/schemas/CommissionLeg CommissionLeg { commissionValues [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } fee #/components/schemas/Fees Fees { feeLegs [] #/components/schemas/FeeLeg FeeLeg { feeValues [] #/components/schemas/FeeValue FeeValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } trueCommission #/components/schemas/Commission Commission { commissionLegs [] #/components/schemas/CommissionLeg CommissionLeg { commissionValues [] #/components/schemas/CommissionValue CommissionValue { value number ($double) type FeeType string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FTT, FUTURES_CLEARING_FEE, FUTURES_DESK_OFFICE_FEE, FUTURES_EXCHANGE_FEE, FUTURES_GLOBEX_FEE, FUTURES_NFA_FEE, FUTURES_PIT_BROKERAGE_FEE, FUTURES_TRANSACTION_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, TEFRA_TAX, STATE_TAX, UNKNOWN ] } } } } |

## `OrderActivity`

| Field | Details |
| --- | --- |
| `activityType` | string Enum: [ EXECUTION, ORDER_ACTION ] |
| `executionType` | string Enum: [ FILL ] |
| `quantity` | number ($double) |
| `orderRemainingQuantity` | number ($double) |
| `executionLegs` | [] xml: OrderedMap { "name": "executionLegs", "wrapped": true } #/components/schemas/ExecutionLeg ExecutionLeg { legId integer ($int64) price number ($double) quantity number ($double) mismarkedQuantity number ($double) instrumentId integer ($int64) time string ($date-time) } |

## `ExecutionLeg`

| Field | Details |
| --- | --- |
| `legId` | integer ($int64) |
| `price` | number ($double) |
| `quantity` | number ($double) |
| `mismarkedQuantity` | number ($double) |
| `instrumentId` | integer ($int64) |
| `time` | string ($date-time) |

## `Position`

| Field | Details |
| --- | --- |
| `shortQuantity` | number ($double) |
| `averagePrice` | number ($double) |
| `currentDayProfitLoss` | number ($double) |
| `currentDayProfitLossPercentage` | number ($double) |
| `longQuantity` | number ($double) |
| `settledLongQuantity` | number ($double) |
| `settledShortQuantity` | number ($double) |
| `agedQuantity` | number ($double) |
| `instrument` | #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } |
| `marketValue` | number ($double) |
| `maintenanceRequirement` | number ($double) |
| `averageLongPrice` | number ($double) |
| `averageShortPrice` | number ($double) |
| `taxLotAverageLongPrice` | number ($double) |
| `taxLotAverageShortPrice` | number ($double) |
| `longOpenProfitLoss` | number ($double) |
| `shortOpenProfitLoss` | number ($double) |
| `previousSessionLongQuantity` | number ($double) |
| `previousSessionShortQuantity` | number ($double) |
| `currentDayCost` | number ($double) |

## `ServiceError`

| Field | Details |
| --- | --- |
| `message` | string |
| `errors` | [] string |

## `OrderLegCollection`

| Field | Details |
| --- | --- |
| `orderLegType` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `legId` | integer ($int64) |
| `instrument` | #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } |
| `instruction` | instruction string Enum: [ BUY, SELL, BUY_TO_COVER, SELL_SHORT, BUY_TO_OPEN, BUY_TO_CLOSE, SELL_TO_OPEN, SELL_TO_CLOSE, EXCHANGE, SELL_SHORT_EXEMPT ] |
| `positionEffect` | string Enum: [ OPENING, CLOSING, AUTOMATIC ] |
| `quantity` | number ($double) |
| `quantityType` | string Enum: [ ALL_SHARES, DOLLARS, SHARES ] |
| `divCapGains` | string Enum: [ REINVEST, PAYOUT ] |
| `toSymbol` | string |

## `SecuritiesAccount`

| Field | Details |
| --- | --- |
| `oneOf ->` | #/components/schemas/MarginAccount MarginAccount { type string Enum: [ CASH, MARGIN ] accountNumber string roundTrips integer ($int32) isDayTrader boolean default: false isClosingOnlyRestricted boolean default: false pfcbFlag boolean default: false positions [] #/components/schemas/Position Position { shortQuantity number ($double) averagePrice number ($double) currentDayProfitLoss number ($double) currentDayProfitLossPercentage number ($double) longQuantity number ($double) settledLongQuantity number ($double) settledShortQuantity number ($double) agedQuantity number ($double) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { } #/components/schemas/AccountFixedIncome AccountFixedIncome { maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { } #/components/schemas/AccountOption AccountOption { optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } marketValue number ($double) maintenanceRequirement number ($double) averageLongPrice number ($double) averageShortPrice number ($double) taxLotAverageLongPrice number ($double) taxLotAverageShortPrice number ($double) longOpenProfitLoss number ($double) shortOpenProfitLoss number ($double) previousSessionLongQuantity number ($double) previousSessionShortQuantity number ($double) currentDayCost number ($double) } initialBalances #/components/schemas/MarginInitialBalance MarginInitialBalance { accruedInterest number ($double) availableFundsNonMarginableTrade number ($double) bondValue number ($double) buyingPower number ($double) cashBalance number ($double) cashAvailableForTrading number ($double) cashReceipts number ($double) dayTradingBuyingPower number ($double)… |

## `SecuritiesAccountBase`

| Field | Details |
| --- | --- |
| `type` | string Enum: [ CASH, MARGIN ] |
| `accountNumber` | string |
| `roundTrips` | integer ($int32) |
| `isDayTrader` | boolean default: false |
| `isClosingOnlyRestricted` | boolean default: false |
| `pfcbFlag` | boolean default: false |
| `positions` | [] #/components/schemas/Position Position { shortQuantity number ($double) averagePrice number ($double) currentDayProfitLoss number ($double) currentDayProfitLossPercentage number ($double) longQuantity number ($double) settledLongQuantity number ($double) settledShortQuantity number ($double) agedQuantity number ($double) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { } #/components/schemas/AccountFixedIncome AccountFixedIncome { maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { } #/components/schemas/AccountOption AccountOption { optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } marketValue number ($double) maintenanceRequirement number ($double) averageLongPrice number ($double) averageShortPrice number ($double) taxLotAverageLongPrice number ($double) taxLotAverageShortPrice number ($double) longOpenProfitLoss number ($double) shortOpenProfitLoss number ($double) previousSessionLongQuantity number ($double) previousSessionShortQuantity number ($double) currentDayCost number ($double) } |

## `MarginAccount`

| Field | Details |
| --- | --- |
| `type` | string Enum: [ CASH, MARGIN ] |
| `accountNumber` | string |
| `roundTrips` | integer ($int32) |
| `isDayTrader` | boolean default: false |
| `isClosingOnlyRestricted` | boolean default: false |
| `pfcbFlag` | boolean default: false |
| `positions` | [] #/components/schemas/Position Position { shortQuantity number ($double) averagePrice number ($double) currentDayProfitLoss number ($double) currentDayProfitLossPercentage number ($double) longQuantity number ($double) settledLongQuantity number ($double) settledShortQuantity number ($double) agedQuantity number ($double) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { } #/components/schemas/AccountFixedIncome AccountFixedIncome { maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { } #/components/schemas/AccountOption AccountOption { optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } marketValue number ($double) maintenanceRequirement number ($double) averageLongPrice number ($double) averageShortPrice number ($double) taxLotAverageLongPrice number ($double) taxLotAverageShortPrice number ($double) longOpenProfitLoss number ($double) shortOpenProfitLoss number ($double) previousSessionLongQuantity number ($double) previousSessionShortQuantity number ($double) currentDayCost number ($double) } |
| `initialBalances` | #/components/schemas/MarginInitialBalance MarginInitialBalance { accruedInterest number ($double) availableFundsNonMarginableTrade number ($double) bondValue number ($double) buyingPower number ($double) cashBalance number ($double) cashAvailableForTrading number ($double) cashReceipts number ($double) dayTradingBuyingPower number ($double) dayTradingBuyingPowerCall number ($double) dayTradingEquityCall number ($double) equity number ($double) equityPercentage number ($double) liquidationValue number ($double) longMarginValue number ($double) longOptionMarketValue number ($double) longStockValue number ($double) maintenanceCall number ($double) maintenanceRequirement number ($double) margin number ($double) marginEquity number ($double) moneyMarketFund number ($double) mutualFundValue number ($double) regTCall number ($double) shortMarginValue number ($double) shortOptionMarketValue number ($double) shortStockValue number ($double) totalCash number ($double) isInCall number ($double) unsettledCash number ($double) pendingDeposits number ($double) marginBalance number ($double) shortBalance number ($double) accountValue number ($double) } |
| `currentBalances` | #/components/schemas/MarginBalance MarginBalance { availableFunds number ($double) availableFundsNonMarginableTrade number ($double) buyingPower number ($double) buyingPowerNonMarginableTrade number ($double) dayTradingBuyingPower number ($double) dayTradingBuyingPowerCall number ($double) equity number ($double) equityPercentage number ($double) longMarginValue number ($double) maintenanceCall number ($double) maintenanceRequirement number ($double) marginBalance number ($double) regTCall number ($double) shortBalance number ($double) shortMarginValue number ($double) sma number ($double) isInCall number ($double) stockBuyingPower number ($double) optionBuyingPower number ($double) } |
| `projectedBalances` | #/components/schemas/MarginBalance MarginBalance { availableFunds number ($double) availableFundsNonMarginableTrade number ($double) buyingPower number ($double) buyingPowerNonMarginableTrade number ($double) dayTradingBuyingPower number ($double) dayTradingBuyingPowerCall number ($double) equity number ($double) equityPercentage number ($double) longMarginValue number ($double) maintenanceCall number ($double) maintenanceRequirement number ($double) marginBalance number ($double) regTCall number ($double) shortBalance number ($double) shortMarginValue number ($double) sma number ($double) isInCall number ($double) stockBuyingPower number ($double) optionBuyingPower number ($double) } |

## `MarginInitialBalance`

| Field | Details |
| --- | --- |
| `accruedInterest` | number ($double) |
| `availableFundsNonMarginableTrade` | number ($double) |
| `bondValue` | number ($double) |
| `buyingPower` | number ($double) |
| `cashBalance` | number ($double) |
| `cashAvailableForTrading` | number ($double) |
| `cashReceipts` | number ($double) |
| `dayTradingBuyingPower` | number ($double) |
| `dayTradingBuyingPowerCall` | number ($double) |
| `dayTradingEquityCall` | number ($double) |
| `equity` | number ($double) |
| `equityPercentage` | number ($double) |
| `liquidationValue` | number ($double) |
| `longMarginValue` | number ($double) |
| `longOptionMarketValue` | number ($double) |
| `longStockValue` | number ($double) |
| `maintenanceCall` | number ($double) |
| `maintenanceRequirement` | number ($double) |
| `margin` | number ($double) |
| `marginEquity` | number ($double) |
| `moneyMarketFund` | number ($double) |
| `mutualFundValue` | number ($double) |
| `regTCall` | number ($double) |
| `shortMarginValue` | number ($double) |
| `shortOptionMarketValue` | number ($double) |
| `shortStockValue` | number ($double) |
| `totalCash` | number ($double) |
| `isInCall` | number ($double) |
| `unsettledCash` | number ($double) |
| `pendingDeposits` | number ($double) |
| `marginBalance` | number ($double) |
| `shortBalance` | number ($double) |
| `accountValue` | number ($double) |

## `MarginBalance`

| Field | Details |
| --- | --- |
| `availableFunds` | number ($double) |
| `availableFundsNonMarginableTrade` | number ($double) |
| `buyingPower` | number ($double) |
| `buyingPowerNonMarginableTrade` | number ($double) |
| `dayTradingBuyingPower` | number ($double) |
| `dayTradingBuyingPowerCall` | number ($double) |
| `equity` | number ($double) |
| `equityPercentage` | number ($double) |
| `longMarginValue` | number ($double) |
| `maintenanceCall` | number ($double) |
| `maintenanceRequirement` | number ($double) |
| `marginBalance` | number ($double) |
| `regTCall` | number ($double) |
| `shortBalance` | number ($double) |
| `shortMarginValue` | number ($double) |
| `sma` | number ($double) |
| `isInCall` | number ($double) |
| `stockBuyingPower` | number ($double) |
| `optionBuyingPower` | number ($double) |

## `CashAccount`

| Field | Details |
| --- | --- |
| `type` | string Enum: [ CASH, MARGIN ] |
| `accountNumber` | string |
| `roundTrips` | integer ($int32) |
| `isDayTrader` | boolean default: false |
| `isClosingOnlyRestricted` | boolean default: false |
| `pfcbFlag` | boolean default: false |
| `positions` | [] #/components/schemas/Position Position { shortQuantity number ($double) averagePrice number ($double) currentDayProfitLoss number ($double) currentDayProfitLossPercentage number ($double) longQuantity number ($double) settledLongQuantity number ($double) settledShortQuantity number ($double) agedQuantity number ($double) instrument #/components/schemas/AccountsInstrument AccountsInstrument { oneOf -> #/components/schemas/AccountCashEquivalent AccountCashEquivalent { type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { } #/components/schemas/AccountFixedIncome AccountFixedIncome { maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { } #/components/schemas/AccountOption AccountOption { optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } } marketValue number ($double) maintenanceRequirement number ($double) averageLongPrice number ($double) averageShortPrice number ($double) taxLotAverageLongPrice number ($double) taxLotAverageShortPrice number ($double) longOpenProfitLoss number ($double) shortOpenProfitLoss number ($double) previousSessionLongQuantity number ($double) previousSessionShortQuantity number ($double) currentDayCost number ($double) } |
| `initialBalances` | #/components/schemas/CashInitialBalance CashInitialBalance { accruedInterest number ($double) cashAvailableForTrading number ($double) cashAvailableForWithdrawal number ($double) cashBalance number ($double) bondValue number ($double) cashReceipts number ($double) liquidationValue number ($double) longOptionMarketValue number ($double) longStockValue number ($double) moneyMarketFund number ($double) mutualFundValue number ($double) shortOptionMarketValue number ($double) shortStockValue number ($double) isInCall number ($double) unsettledCash number ($double) cashDebitCallValue number ($double) pendingDeposits number ($double) accountValue number ($double) } |
| `currentBalances` | #/components/schemas/CashBalance CashBalance { cashAvailableForTrading number ($double) cashAvailableForWithdrawal number ($double) cashCall number ($double) longNonMarginableMarketValue number ($double) totalCash number ($double) cashDebitCallValue number ($double) unsettledCash number ($double) } |
| `projectedBalances` | #/components/schemas/CashBalance CashBalance { cashAvailableForTrading number ($double) cashAvailableForWithdrawal number ($double) cashCall number ($double) longNonMarginableMarketValue number ($double) totalCash number ($double) cashDebitCallValue number ($double) unsettledCash number ($double) } |

## `CashInitialBalance`

| Field | Details |
| --- | --- |
| `accruedInterest` | number ($double) |
| `cashAvailableForTrading` | number ($double) |
| `cashAvailableForWithdrawal` | number ($double) |
| `cashBalance` | number ($double) |
| `bondValue` | number ($double) |
| `cashReceipts` | number ($double) |
| `liquidationValue` | number ($double) |
| `longOptionMarketValue` | number ($double) |
| `longStockValue` | number ($double) |
| `moneyMarketFund` | number ($double) |
| `mutualFundValue` | number ($double) |
| `shortOptionMarketValue` | number ($double) |
| `shortStockValue` | number ($double) |
| `isInCall` | number ($double) |
| `unsettledCash` | number ($double) |
| `cashDebitCallValue` | number ($double) |
| `pendingDeposits` | number ($double) |
| `accountValue` | number ($double) |

## `CashBalance`

| Field | Details |
| --- | --- |
| `cashAvailableForTrading` | number ($double) |
| `cashAvailableForWithdrawal` | number ($double) |
| `cashCall` | number ($double) |
| `longNonMarginableMarketValue` | number ($double) |
| `totalCash` | number ($double) |
| `cashDebitCallValue` | number ($double) |
| `unsettledCash` | number ($double) |

## `TransactionBaseInstrument`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |

## `AccountsBaseInstrument`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |

## `AccountsInstrument`

| Field | Details |
| --- | --- |
| `oneOf ->` | #/components/schemas/AccountCashEquivalent AccountCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/AccountEquity AccountEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountFixedIncome AccountFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) maturityDate string ($date-time) factor number ($double) variableRate number ($double) } #/components/schemas/AccountMutualFund AccountMutualFund { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/AccountOption AccountOption { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) optionDeliverables [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } putCall string Enum: [ PUT, CALL, UNKNOWN ] optionMultiplier integer ($int32) type string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] underlyingSymbol string } |

## `TransactionInstrument`

| Field | Details |
| --- | --- |
| `oneOf ->` | #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex Forex { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_… |

## `TransactionCashEquivalent`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] |

## `CollectiveInvestment`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] |

## `instruction`

No top-level properties exposed in the expanded schema block.

## `assetType`

No top-level properties exposed in the expanded schema block.

## `Currency`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |

## `TransactionEquity`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] |

## `TransactionFixedIncome`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] |
| `maturityDate` | string ($date-time) |
| `factor` | number ($double) |
| `multiplier` | number ($double) |
| `variableRate` | number ($double) |

## `Forex`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ STANDARD, NBBO, UNKNOWN ] |
| `baseCurrency` | #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } |
| `counterCurrency` | #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } |

## `Future`

| Field | Details |
| --- | --- |
| `activeContract` | boolean default: false |
| `type` | string Enum: [ STANDARD, UNKNOWN ] |
| `expirationDate` | string ($date-time) |
| `lastTradingDate` | string ($date-time) |
| `firstNoticeDate` | string ($date-time) |
| `multiplier` | number ($double) |
| `oneOf ->` | #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex Forex { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_… |

## `Index`

| Field | Details |
| --- | --- |
| `activeContract` | boolean default: false |
| `type` | string Enum: [ BROAD_BASED, NARROW_BASED, UNKNOWN ] |
| `oneOf ->` | #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex Forex { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_… |

## `TransactionMutualFund`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `fundFamilyName` | string |
| `fundFamilySymbol` | string |
| `fundGroup` | string |
| `type` | string Enum: [ NOT_APPLICABLE, OPEN_END_NON_TAXABLE, OPEN_END_TAXABLE, NO_LOAD_NON_TAXABLE, NO_LOAD_TAXABLE, UNKNOWN ] |
| `exchangeCutoffTime` | string ($date-time) |
| `purchaseCutoffTime` | string ($date-time) |
| `redemptionCutoffTime` | string ($date-time) |

## `TransactionOption`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `expirationDate` | string ($date-time) |
| `optionDeliverables` | [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/TransactionAPIOptionDeliverable TransactionAPIOptionDeliverable { rootSymbol string strikePercent integer ($int64) deliverableNumber integer ($int64) deliverableUnits number ($double) deliverable #/components/schemas/TransactionInstrument TransactionInstrument { oneOf -> #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY… |
| `optionPremiumMultiplier` | integer ($int64) |
| `putCall` | string Enum: [ PUT, CALL, UNKNOWN ] |
| `strikePrice` | number ($double) |
| `type` | string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] |
| `underlyingSymbol` | string |
| `underlyingCusip` | string |
| `deliverable` | #/components/schemas/TransactionInstrument TransactionInstrument { oneOf -> #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex… |

## `Product`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ TBD, UNKNOWN ] |

## `AccountCashEquivalent`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `type` | string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] |

## `AccountEquity`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |

## `AccountFixedIncome`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `maturityDate` | string ($date-time) |
| `factor` | number ($double) |
| `variableRate` | number ($double) |

## `AccountMutualFund`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |

## `AccountOption`

| Field | Details |
| --- | --- |
| `assetType *` | string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] |
| `cusip` | string |
| `symbol` | string |
| `description` | string |
| `instrumentId` | integer ($int64) |
| `netChange` | number ($double) |
| `optionDeliverables` | [] xml: OrderedMap { "name": "optionDeliverables", "wrapped": true } #/components/schemas/AccountAPIOptionDeliverable AccountAPIOptionDeliverable { symbol string ($int64) deliverableUnits number ($double) apiCurrencyType string Enum: [ USD, CAD, EUR, JPY ] assetType assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] } |
| `putCall` | string Enum: [ PUT, CALL, UNKNOWN ] |
| `optionMultiplier` | integer ($int32) |
| `type` | string Enum: [ VANILLA, BINARY, BARRIER, UNKNOWN ] |
| `underlyingSymbol` | string |

## `AccountAPIOptionDeliverable`

| Field | Details |
| --- | --- |
| `symbol` | string ($int64) |
| `deliverableUnits` | number ($double) |
| `apiCurrencyType` | string Enum: [ USD, CAD, EUR, JPY ] |
| `assetType` | assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] |

## `TransactionAPIOptionDeliverable`

| Field | Details |
| --- | --- |
| `rootSymbol` | string |
| `strikePercent` | integer ($int64) |
| `deliverableNumber` | integer ($int64) |
| `deliverableUnits` | number ($double) |
| `deliverable` | #/components/schemas/TransactionInstrument TransactionInstrument { oneOf -> #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex… |
| `assetType` | assetType string Enum: [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ] |

## `apiOrderStatus`

No top-level properties exposed in the expanded schema block.

## `TransactionType`

No top-level properties exposed in the expanded schema block.

## `Transaction`

| Field | Details |
| --- | --- |
| `activityId` | integer ($int64) |
| `time` | string ($date-time) |
| `user` | #/components/schemas/UserDetails UserDetails { cdDomainId string login string type string Enum: [ ADVISOR_USER, BROKER_USER, CLIENT_USER, SYSTEM_USER, UNKNOWN ] userId integer ($int64) systemUserName string firstName string lastName string brokerRepCode string } |
| `description` | string |
| `accountNumber` | string |
| `type` | TransactionType string Enum: [ TRADE, RECEIVE_AND_DELIVER, DIVIDEND_OR_INTEREST, ACH_RECEIPT, ACH_DISBURSEMENT, CASH_RECEIPT, CASH_DISBURSEMENT, ELECTRONIC_FUND, WIRE_OUT, WIRE_IN, JOURNAL, MEMORANDUM, MARGIN_CALL, MONEY_MARKET, SMA_ADJUSTMENT ] |
| `status` | string Enum: [ VALID, INVALID, PENDING, UNKNOWN ] |
| `subAccount` | string Enum: [ CASH, MARGIN, SHORT, DIV, INCOME, UNKNOWN ] |
| `tradeDate` | string ($date-time) |
| `settlementDate` | string ($date-time) |
| `positionId` | integer ($int64) |
| `orderId` | integer ($int64) |
| `netAmount` | number ($double) |
| `activityType` | string Enum: [ ACTIVITY_CORRECTION, EXECUTION, ORDER_ACTION, TRANSFER, UNKNOWN ] |
| `transferItems` | [] xml: OrderedMap { "name": "transferItems", "wrapped": true } #/components/schemas/TransferItem TransferItem { instrument #/components/schemas/TransactionInstrument TransactionInstrument { oneOf -> #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string… |

## `UserDetails`

| Field | Details |
| --- | --- |
| `cdDomainId` | string |
| `login` | string |
| `type` | string Enum: [ ADVISOR_USER, BROKER_USER, CLIENT_USER, SYSTEM_USER, UNKNOWN ] |
| `userId` | integer ($int64) |
| `systemUserName` | string |
| `firstName` | string |
| `lastName` | string |
| `brokerRepCode` | string |

## `TransferItem`

| Field | Details |
| --- | --- |
| `instrument` | #/components/schemas/TransactionInstrument TransactionInstrument { oneOf -> #/components/schemas/TransactionCashEquivalent TransactionCashEquivalent { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ] } #/components/schemas/CollectiveInvestment CollectiveInvestment { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ] } #/components/schemas/Currency Currency { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) } #/components/schemas/TransactionEquity TransactionEquity { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ] } #/components/schemas/TransactionFixedIncome TransactionFixedIncome { assetType * string Enum: [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ] cusip string symbol string description string instrumentId integer ($int64) netChange number ($double) type string Enum: [ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ] maturityDate string ($date-time) factor number ($double) multiplier number ($double) variableRate number ($double) } #/components/schemas/Forex… |
| `amount` | number ($double) |
| `cost` | number ($double) |
| `price` | number ($double) |
| `feeType` | string Enum: [ COMMISSION, SEC_FEE, STR_FEE, R_FEE, CDSC_FEE, OPT_REG_FEE, ADDITIONAL_FEE, MISCELLANEOUS_FEE, FUTURES_EXCHANGE_FEE, LOW_PROCEEDS_COMMISSION, BASE_CHARGE, GENERAL_CHARGE, GST_FEE, TAF_FEE, INDEX_OPTION_FEE, UNKNOWN ] |
| `positionEffect` | string Enum: [ OPENING, CLOSING, AUTOMATIC, UNKNOWN ] |

## `UserPreference`

| Field | Details |
| --- | --- |
| `accounts` | [] #/components/schemas/UserPreferenceAccount UserPreferenceAccount { accountNumber string primaryAccount boolean default: false type string nickName string accountColor string Green \| Blue displayAcctId string autoPositionEffect boolean default: false } |
| `streamerInfo` | [] #/components/schemas/StreamerInfo StreamerInfo { streamerSocketUrl string schwabClientCustomerId string schwabClientCorrelId string schwabClientChannel string schwabClientFunctionId string } |
| `offers` | [] #/components/schemas/Offer Offer { level2Permissions boolean default: false mktDataPermission string } |

## `UserPreferenceAccount`

| Field | Details |
| --- | --- |
| `accountNumber` | string |
| `primaryAccount` | boolean default: false |
| `type` | string |
| `nickName` | string |
| `accountColor` | string Green \| Blue |
| `displayAcctId` | string |
| `autoPositionEffect` | boolean default: false |

## `StreamerInfo`

| Field | Details |
| --- | --- |
| `streamerSocketUrl` | string |
| `schwabClientCustomerId` | string |
| `schwabClientCorrelId` | string |
| `schwabClientChannel` | string |
| `schwabClientFunctionId` | string |

## `Offer`

| Field | Details |
| --- | --- |
| `level2Permissions` | boolean default: false |
| `mktDataPermission` | string |
