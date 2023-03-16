# Currency Exchange Rate Calculator

## Problem Statement

We are faced with the challenge of calculating exchange rates between different currencies while minimizing redundant computations. To achieve this, we have two database tables, namely `Currencies` and `Conversion Rates`.

Our objective is to create a currency exchange rate calculator that can compute exchange rates between currencies with the least possible computational cost. To achieve this, we aim to keep a tree structure in memory and utilize a hashmap.

## Different Types of Exchange Rates

There are two types of exchange rates that we need to consider:

- Direct Rate: This rate is calculated directly from the main currency to the target currency. It is prioritized and must show the source and recency.
- Derived Rate: This rate includes the path of how the rate is calculated, which must be the shortest path. Each item in the path must show the source and recency. The derived rate itself must show a calculated recency.

## Required Operations

The currency exchange rate calculator must be able to perform the following operations:

- Get the exchange rate between the main currency and any arbitrary currency
- Add a new exchange rate between two currencies
  - If the new rate affects the current tree, the tree needs to be recalculated.

## Implementation Approach

To implement the currency exchange rate calculator, we can follow these steps:

- Get:
  - Retrieve the target node from the hashmap
  - Trace back by looking back until we reach the root (the main currency)
- Add a new rate:
  - Check if any of the rates are already in the hashmap
    - If true:
      - Recalculate BFS (Breadth-First Search) from the root
      - Send a subscriber event to invalidate all cached rates in the frontend
    - If false:
      - Ignore the new rate

By utilizing a tree structure and a hashmap, we can significantly reduce the computational cost of computing exchange rates. Additionally, by implementing the above operations, we can ensure that our calculator is efficient and effective.
