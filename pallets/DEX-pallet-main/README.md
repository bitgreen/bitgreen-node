# Substrate pallet for token swap

## Overview

**Substrate pallet to facilitate purchase of tokens using another token.**

## Callable Functions

- `CreateSellOrder(token_id, volume, price, seller)`

  Creates on-chain sell order for “Token A” from owner in exchange for “Token B”. Order is static, cannot be modified, only cancelled. Returns success and orderID, or failure and error code.

- `CancelSellOrder(order_id, seller)`

  Cancel on-chain sell order via orderID. Returns success, or failure and error code. There are 2 commissions that must be paid by the buyer: 1) a 2% fee in “Token B” 2) a fee of 0.2 “Token B” per “Token A” purchased. Token A can only be bought sold in integers, no fractional Token A”

- `BuyOrder(project_id, bundle_id, volume, seller)`

  On-chain action to fill a sell order. Calling wallet must contain enough funds to cover the
  purchase price (in Token B) plus commissions of 2% + $0.20 per token.

## Events

- `SellOrderCreated (token_id, amount, price, seller)`

  Emits sell order details

- `SellOrderCancelled (order_id, status)`

  Emits when sell order is cancelled

- `BuyOrderFilled (order_id, amount_sold, amount_paid, seller, buyer)`

  Emits when sell order is fulfilled
