# Schwab Accounts and Trading REST API

Source: `trading-production-specs.html`

Endpoint count: `13`

## GET `/accounts/accountNumbers`

Operation: `Accounts.getAccountNumbers`

Get list of account numbers and their encrypted values

Account numbers in plain text cannot be used outside of headers or request/response bodies. As the first step consumers must invoke this service to retrieve the list of plain text/encrypted value pairs, and use encrypted account values for all subsequent calls for any accountNumber request.

Parameters: none.

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | List of valid "accounts", matching the provided input parameters. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts`

Operation: `Accounts.getAccounts`

Get linked account(s) balances and positions for the logged in user.

All the linked account information for the user logged in. The balances on these accounts are displayed by default however the positions on these accounts will be displayed based on the "positions" flag.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `fields` | `query` | `string` | `false` | This allows one to determine which fields they want returned. Possible value in this String can be: ; positions ; Example:; fields=positions |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | List of valid "accounts", matching the provided input parameters. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts/{accountNumber}`

Operation: `Accounts.getAccount`

Get a specific account balance and positions for the logged in user.

Specific account information with balances and positions. The balance information on these accounts is displayed by default but Positions will be returned based on the "positions" flag.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `fields` | `query` | `string` | `false` | This allows one to determine which fields they want returned. Possible values in this String can be: ; positions ; Example:; fields=positions |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | A valid account, matching the provided input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts/{accountNumber}/orders`

Operation: `Orders.getOrdersByPathParam`

Get all orders for a specific account.

All orders for a specific account. Orders retrieved can be filtered based on input parameters below. Maximum date range is 1 year.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `maxResults` | `query` | `integer ($int64)` | `false` | The max number of orders to retrieve. Default is 3000. |
| `fromEnteredTime` | `query` | `string` | `true` | Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are :; yyyy-MM-dd'T'HH:mm:ss.SSSZ Example fromEnteredTime is '2024-03-29T00:00:00.000Z'. 'toEnteredTime' must also be set. |
| `toEnteredTime` | `query` | `string` | `true` | Specifies that no orders entered after this time should be returned.Valid ISO-8601 formats are :; yyyy-MM-dd'T'HH:mm:ss.SSSZ . Example toEnteredTime is '2024-04-28T23:59:59.000Z'. 'fromEnteredTime' must also be set. |
| `status` | `query` | `string` | `false` | Specifies that only orders of this status should be returned. Available values : AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | A List of orders for the account, matching the provided input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## POST `/accounts/{accountNumber}/orders`

Operation: `Orders.placeOrder`

Place order for a specific account.

Place an order for a specific account.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |

### Request Body

Required: `true`

Content types: `application/json`

The new Order Object.

```json
{
"session"
:
"NORMAL"
,
"duration"
:
"DAY"
,
"orderType"
:
"MARKET"
,
"cancelTime"
:
"2026-05-05T06:45:48.040Z"
,
"complexOrderStrategyType"
:
"NONE"
,
"quantity"
:
0
,
"filledQuantity"
:
0
,
"remainingQuantity"
:
0
,
"destinationLinkName"
:
"string"
,
"releaseTime"
:
"2026-05-05T06:45:48.040Z"
,
"stopPrice"
:
0
,
"stopPriceLinkBasis"
:
"MANUAL"
,
"stopPriceLinkType"
:
"VALUE"
,
"stopPriceOffset"
:
0
,
"stopType"
:
"STANDARD"
,
"priceLinkBasis"
:
"MANUAL"
,
"priceLinkType"
:
"VALUE"
,
"price"
:
0
,
"taxLotMethod"
:
"FIFO"
,
"orderLegCollection"
: [
    {
"orderLegType"
:
"EQUITY"
,
"legId"
:
0
,
"instrument"
: {
"cusip"
:
"string"
,
"symbol"
:
"string"
,
"description"
:
"string"
,
"instrumentId"
:
0
,
"netChange"
:
0
,
"type"
:
"SWEEP_VEHICLE"
      },
"instruction"
:
"BUY"
,
"positionEffect"
:
"OPENING"
,
"quantity"
:
0
,
"quantityType"
:
"ALL_SHARES"
,
"divCapGains"
:
"REINVEST"
,
"toSymbol"
:
"string"
    }
  ],
"activationPrice"
:
0
,
"specialInstruction"
:
"ALL_OR_NONE"
,
"orderStrategyType"
:
"SINGLE"
,
"orderId"
:
0
,
"cancelable"
:
false
,
"editable"
:
false
,
"status"
:
"AWAITING_PARENT_ORDER"
,
"enteredTime"
:
"2026-05-05T06:45:48.040Z"
,
"closeTime"
:
"2026-05-05T06:45:48.040Z"
,
"accountNumber"
:
0
,
"orderActivityCollection"
: [
    {
"activityType"
:
"EXECUTION"
,
"executionType"
:
"FILL"
,
"quantity"
:
0
,
"orderRemainingQuantity"
:
0
,
"executionLegs"
: [
        {
"legId"
:
0
,
"price"
:
0
,
"quantity"
:
0
,
"mismarkedQuantity"
:
0
,
"instrumentId"
:
0
,
"time"
:
"2026-05-05T06:45:48.040Z"
        }
      ]
    }
  ],
"replacingOrderCollection"
: [
"string"
  ],
"childOrderStrategies"
: [
"string"
  ],
"statusDescription"
:
"string"
}
```

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `201` | Empty response body if an order was successfully placed/created. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts/{accountNumber}/orders/{orderId}`

Operation: `Orders.getOrder`

Get a specific order by its ID, for a specific account

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `orderId` | `path` | `integer ($int64)` | `true` | The ID of the order being retrieved. |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | An order object, matching the input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## DELETE `/accounts/{accountNumber}/orders/{orderId}`

Operation: `Orders.cancelOrder`

Cancel an order for a specific account

Cancel a specific order for a specific account;

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `orderId` | `path` | `integer ($int64)` | `true` | The ID of the order being cancelled |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | Empty response body if an order was successfully canceled. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## PUT `/accounts/{accountNumber}/orders/{orderId}`

Operation: `Orders.replaceOrder`

Replace order for a specific account

Replace an existing order for an account. The existing order will be replaced by the new order. Once replaced, the old order will be canceled and a new order will be created.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `orderId` | `path` | `integer ($int64)` | `true` | The ID of the order being retrieved. |

### Request Body

Required: `true`

Content types: `application/json`

The Order Object.

```json
{
"session"
:
"NORMAL"
,
"duration"
:
"DAY"
,
"orderType"
:
"MARKET"
,
"cancelTime"
:
"2026-05-05T06:45:48.050Z"
,
"complexOrderStrategyType"
:
"NONE"
,
"quantity"
:
0
,
"filledQuantity"
:
0
,
"remainingQuantity"
:
0
,
"destinationLinkName"
:
"string"
,
"releaseTime"
:
"2026-05-05T06:45:48.050Z"
,
"stopPrice"
:
0
,
"stopPriceLinkBasis"
:
"MANUAL"
,
"stopPriceLinkType"
:
"VALUE"
,
"stopPriceOffset"
:
0
,
"stopType"
:
"STANDARD"
,
"priceLinkBasis"
:
"MANUAL"
,
"priceLinkType"
:
"VALUE"
,
"price"
:
0
,
"taxLotMethod"
:
"FIFO"
,
"orderLegCollection"
: [
    {
"orderLegType"
:
"EQUITY"
,
"legId"
:
0
,
"instrument"
: {
"cusip"
:
"string"
,
"symbol"
:
"string"
,
"description"
:
"string"
,
"instrumentId"
:
0
,
"netChange"
:
0
,
"type"
:
"SWEEP_VEHICLE"
      },
"instruction"
:
"BUY"
,
"positionEffect"
:
"OPENING"
,
"quantity"
:
0
,
"quantityType"
:
"ALL_SHARES"
,
"divCapGains"
:
"REINVEST"
,
"toSymbol"
:
"string"
    }
  ],
"activationPrice"
:
0
,
"specialInstruction"
:
"ALL_OR_NONE"
,
"orderStrategyType"
:
"SINGLE"
,
"orderId"
:
0
,
"cancelable"
:
false
,
"editable"
:
false
,
"status"
:
"AWAITING_PARENT_ORDER"
,
"enteredTime"
:
"2026-05-05T06:45:48.050Z"
,
"closeTime"
:
"2026-05-05T06:45:48.050Z"
,
"accountNumber"
:
0
,
"orderActivityCollection"
: [
    {
"activityType"
:
"EXECUTION"
,
"executionType"
:
"FILL"
,
"quantity"
:
0
,
"orderRemainingQuantity"
:
0
,
"executionLegs"
: [
        {
"legId"
:
0
,
"price"
:
0
,
"quantity"
:
0
,
"mismarkedQuantity"
:
0
,
"instrumentId"
:
0
,
"time"
:
"2026-05-05T06:45:48.050Z"
        }
      ]
    }
  ],
"replacingOrderCollection"
: [
"string"
  ],
"childOrderStrategies"
: [
"string"
  ],
"statusDescription"
:
"string"
}
```

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `201` | Empty response body if an order was successfully replaced/created. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/orders`

Operation: `Orders.getOrdersByQueryParam`

Get all orders for all accounts

Get all orders for all accounts;

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `maxResults` | `query` | `integer ($int64)` | `false` | The max number of orders to retrieve. Default is 3000. |
| `fromEnteredTime` | `query` | `string` | `true` | Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are- yyyy-MM-dd'T'HH:mm:ss.SSSZ Date must be within 60 days from today's date. 'toEnteredTime' must also be set. |
| `toEnteredTime` | `query` | `string` | `true` | Specifies that no orders entered after this time should be returned.Valid ISO-8601 formats are - yyyy-MM-dd'T'HH:mm:ss.SSSZ. 'fromEnteredTime' must also be set. |
| `status` | `query` | `string` | `false` | Specifies that only orders of this status should be returned. Available values : AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | A List of orders for the specified account or if its not mentioned, for all the linked accounts, matching the provided input parameters. | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## POST `/accounts/{accountNumber}/previewOrder`

Operation: `Orders.previewOrder`

Preview order for a specific account.

Preview an order for a specific account.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |

### Request Body

Required: `true`

Content types: `application/json`

The Order Object.

```json
{
"orderId"
:
0
,
"orderStrategy"
: {
"accountNumber"
:
"string"
,
"advancedOrderType"
:
"NONE"
,
"closeTime"
:
"2026-05-05T06:45:48.056Z"
,
"enteredTime"
:
"2026-05-05T06:45:48.056Z"
,
"orderBalance"
: {
"orderValue"
:
0
,
"projectedAvailableFund"
:
0
,
"projectedBuyingPower"
:
0
,
"projectedCommission"
:
0
    },
"orderStrategyType"
:
"SINGLE"
,
"orderVersion"
:
0
,
"session"
:
"NORMAL"
,
"status"
:
"AWAITING_PARENT_ORDER"
,
"allOrNone"
:
true
,
"discretionary"
:
true
,
"duration"
:
"DAY"
,
"filledQuantity"
:
0
,
"orderType"
:
"MARKET"
,
"orderValue"
:
0
,
"price"
:
0
,
"quantity"
:
0
,
"remainingQuantity"
:
0
,
"sellNonMarginableFirst"
:
true
,
"settlementInstruction"
:
"REGULAR"
,
"strategy"
:
"NONE"
,
"amountIndicator"
:
"DOLLARS"
,
"orderLegs"
: [
      {
"askPrice"
:
0
,
"bidPrice"
:
0
,
"lastPrice"
:
0
,
"markPrice"
:
0
,
"projectedCommission"
:
0
,
"quantity"
:
0
,
"finalSymbol"
:
"string"
,
"legId"
:
0
,
"assetType"
:
"EQUITY"
,
"instruction"
:
"BUY"
      }
    ]
  },
"orderValidationResult"
: {
"alerts"
: [
      {
"validationRuleName"
:
"string"
,
"message"
:
"string"
,
"activityMessage"
:
"string"
,
"originalSeverity"
:
"ACCEPT"
,
"overrideName"
:
"string"
,
"overrideSeverity"
:
"ACCEPT"
      }
    ],
"accepts"
: [
      {
"validationRuleName"
:
"string"
,
"message"
:
"string"
,
"activityMessage"
:
"string"
,
"originalSeverity"
:
"ACCEPT"
,
"overrideName"
:
"string"
,
"overrideSeverity"
:
"ACCEPT"
      }
    ],
"rejects"
: [
      {
"validationRuleName"
:
"string"
,
"message"
:
"string"
,
"activityMessage"
:
"string"
,
"originalSeverity"
:
"ACCEPT"
,
"overrideName"
:
"string"
,
"overrideSeverity"
:
"ACCEPT"
      }
    ],
"reviews"
: [
      {
"validationRuleName"
:
"string"
,
"message"
:
"string"
,
"activityMessage"
:
"string"
,
"originalSeverity"
:
"ACCEPT"
,
"overrideName"
:
"string"
,
"overrideSeverity"
:
"ACCEPT"
      }
    ],
"warns"
: [
      {
"validationRuleName"
:
"string"
,
"message"
:
"string"
,
"activityMessage"
:
"string"
,
"originalSeverity"
:
"ACCEPT"
,
"overrideName"
:
"string"
,
"overrideSeverity"
:
"ACCEPT"
      }
    ]
  },
"commissionAndFee"
: {
"commission"
: {
"commissionLegs"
: [
        {
"commissionValues"
: [
            {
"value"
:
0
,
"type"
:
"COMMISSION"
            }
          ]
        }
      ]
    },
"fee"
: {
"feeLegs"
: [
        {
"feeValues"
: [
            {
"value"
:
0
,
"type"
:
"COMMISSION"
            }
          ]
        }
      ]
    },
"trueCommission"
: {
"commissionLegs"
: [
        {
"commissionValues"
: [
            {
"value"
:
0
,
"type"
:
"COMMISSION"
            }
          ]
        }
      ]
    }
  }
}
```

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | An order object, matching the input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts/{accountNumber}/transactions`

Operation: `Transactions.getTransactionsByPathParam`

Get all transactions information for a specific account.

All transactions for a specific account. Maximum number of transactions in response is 3000. Maximum date range is 1 year.

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `startDate` | `query` | `string` | `true` | Specifies that no transactions entered before this time should be returned. Valid ISO-8601 formats are :; yyyy-MM-dd'T'HH:mm:ss.SSSZ . Example start date is '2024-03-28T21:10:42.000Z'. The 'endDate' must also be set. |
| `endDate` | `query` | `string` | `true` | Specifies that no transactions entered after this time should be returned.Valid ISO-8601 formats are :; yyyy-MM-dd'T'HH:mm:ss.SSSZ . Example start date is '2024-05-10T21:10:42.000Z'. The 'startDate' must also be set. |
| `symbol` | `query` | `string` | `false` | It filters all the transaction activities based on the symbol specified. NOTE: If there is any special character in the symbol, please send th encoded value. |
| `types` | `query` | `string` | `true` | Specifies that only transactions of this status should be returned. Available values : TRADE, RECEIVE_AND_DELIVER, DIVIDEND_OR_INTEREST, ACH_RECEIPT, ACH_DISBURSEMENT, CASH_RECEIPT, CASH_DISBURSEMENT, ELECTRONIC_FUND, WIRE_OUT, WIRE_IN, JOURNAL, MEMORANDUM, MARGIN_CALL, MONEY_MARKET, SMA_ADJUSTMENT |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | A List of orders for the account, matching the provided input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/accounts/{accountNumber}/transactions/{transactionId}`

Operation: `Transactions.getTransactionsById`

Get specific transaction information for a specific account

### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `accountNumber` | `path` | `string` | `true` | The encrypted ID of the account |
| `transactionId` | `path` | `integer ($int64)` | `true` | The ID of the transaction being retrieved. |

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | A List of orders for the account, matching the provided input parameters | `Schwab-Client-CorrelId` |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |

## GET `/userPreference`

Operation: `UserPreference.getUserPreference`

Get user preference information for the logged in user.

Parameters: none.

### Responses

| Code | Description | Headers |
| --- | --- | --- |
| `200` | List of user preference values. |  |
| `400` | An error message indicating the validation problem with the request. | `Schwab-Client-CorrelID` |
| `401` | An error message indicating either authorization token is invalid or there are no accounts the caller is allowed to view or use for trading that are registered with the provided third party application | `Schwab-Client-CorrelID` |
| `403` | An error message indicating the caller is forbidden from accessing this service | `Schwab-Client-CorrelID` |
| `404` | An error message indicating the resource is not found | `Schwab-Client-CorrelID` |
| `500` | An error message indicating there was an unexpected server error | `Schwab-Client-CorrelID` |
| `503` | An error message indicating server has a temporary problem responding | `Schwab-Client-CorrelID` |
