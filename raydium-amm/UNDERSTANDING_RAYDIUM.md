# Understanding Raydium AMM - A Beginner's Guide

## What You're Looking At

You're studying the **Raydium AMM** - a special type of decentralized exchange on Solana that combines two trading methods:
1. **Automated Market Maker (AMM)** - Like a vending machine
2. **Order Book** - Like a traditional marketplace

## The Big Picture: How It All Works

### The Vending Machine Analogy

Imagine a vending machine that:
- Holds SOL (apples) and USDC (oranges)
- Lets you swap apples for oranges instantly
- **BUT ALSO** can walk over to a nearby marketplace (Serum/OpenBook) and place buy/sell orders

This is what makes Raydium special!

### The Key Files

```
raydium-amm/program/src/
├── state.rs        ← YOU ARE HERE (The "memory" - what data we store)
├── instruction.rs  ← The "commands" (what actions users can do)
├── processor.rs    ← The "brain" (how we execute those commands)
├── math.rs         ← The "calculator" (price calculations)
└── error.rs        ← The "error messages"
```

## Understanding `state.rs` - The Memory

This file defines **what information the AMM remembers**. Think of it as the blueprint for the AMM's brain.

### The Main Structures

#### 1. **AmmInfo** - The Master Control Panel
```
This is THE most important structure. It stores EVERYTHING about a pool:
- How much SOL is in the vault?
- How much USDC is in the vault?
- What fees do we charge?
- What's our current status (open? closed? emergency mode?)
- Where is our orderbook account?
```

**Real-world example:**
```
SOL/USDC Pool:
- coin_vault: 10,000 SOL
- pc_vault: 500,000 USDC  
- Current price: ~$50 per SOL
- Status: SwapOnly (normal operation)
- Fees: 0.25% per swap
```

#### 2. **TargetOrders** - The Orderbook Manager
```
This is where Raydium's magic happens!
- buy_orders[50]: "I want to buy SOL at $49.90, $49.80, $49.70..."
- sell_orders[50]: "I want to sell SOL at $50.10, $50.20, $50.30..."
- placed_x/placed_y: How much is currently in active orders
```

**Why 50 orders?**
The AMM spreads its liquidity across multiple price levels on the orderbook, creating a "ladder" of buy and sell orders. This provides better price discovery.

#### 3. **Fees** - The Revenue Model
```
Four types of fees:
1. swap_fee (0.25%): What you pay when swapping
2. trade_fee (0.25%): What you pay when filling orderbook orders
3. pnl_fee (12%): Protocol's cut of market-making profits
4. min_separate (0.05%): Minimum price gap between orders
```

**Example:**
```
You swap $1,000 USDC for SOL:
- Swap fee: $2.50 (goes to liquidity providers)
- You receive: $997.50 worth of SOL
```

#### 4. **AmmStatus** - The Operating Mode
```
Think of this like a store's hours sign:
- SwapOnly: "OPEN - Normal trading"
- WaitingTrade: "OPENING SOON - Scheduled launch"
- WithdrawOnly: "CLOSING - Withdrawals only"
- Disabled: "CLOSED - Emergency"
```

### The Lifecycle of a Swap

When someone swaps 1 SOL for USDC, here's what happens:

```
1. Check AmmStatus
   └─> Is swap_permission() true? ✓

2. Read AmmInfo
   └─> coin_vault: 10,000 SOL
   └─> pc_vault: 500,000 USDC
   └─> fees.swap_fee: 25/10000 (0.25%)

3. Calculate (in math.rs)
   └─> Price = pc_vault / coin_vault = $50 per SOL
   └─> You get: ~$49.875 USDC (after 0.25% fee)

4. Update AmmInfo
   └─> coin_vault: 10,001 SOL (you deposited 1)
   └─> pc_vault: 499,950.125 USDC (you withdrew ~49.875)

5. Update StateData (statistics)
   └─> swap_coin_in_amount += 1 SOL
   └─> swap_pc_out_amount += 49.875 USDC
   └─> swap_acc_pc_fee += 0.125 USDC
```

### The Orderbook Integration

This is what makes Raydium unique. Here's how it works:

```
STEP 1: Planning (happens periodically via "MonitorStep")
├─> AMM looks at its reserves: 10,000 SOL, 500,000 USDC
├─> Calculates fair price: $50 per SOL
└─> Plans orders:
    ├─> Buy orders: $49.90, $49.80, $49.70... (below fair price)
    └─> Sell orders: $50.10, $50.20, $50.30... (above fair price)

STEP 2: Execution
├─> Cancel old orders that are no longer needed
├─> Place new orders on Serum/OpenBook
└─> Update TargetOrders to track what's placed

STEP 3: When orders fill
├─> Someone on OpenBook buys SOL at $50.10
├─> AMM's sell order fills
├─> AMM receives USDC, loses SOL
└─> Profit! (Sold above fair price)
```

### Why This Hybrid Approach?

**Pure AMM (Uniswap):**
- ✓ Always available
- ✗ Suffers from "slippage" on large trades
- ✗ Isolated from other markets

**Pure Orderbook (Serum/OpenBook):**
- ✓ Better prices for large trades
- ✗ Needs active market makers
- ✗ Can have low liquidity

**Raydium (Hybrid):**
- ✓ Always available (AMM fallback)
- ✓ Better prices (orderbook integration)
- ✓ Shared liquidity with Serum/OpenBook
- ✓ Earns fees from both sources

## Key Concepts to Understand

### 1. **Liquidity Providers (LPs)**
People who deposit SOL + USDC into the pool. They get:
- LP tokens (proof of ownership)
- Share of all swap fees (0.25%)
- Share of orderbook trading profits

### 2. **The Constant Product Formula**
```
k = x * y

Where:
- x = amount of token A (SOL)
- y = amount of token B (USDC)
- k = constant

When you swap, k stays the same!
```

**Example:**
```
Before: 10,000 SOL * 500,000 USDC = 5,000,000,000
You swap 1 SOL in
After: 10,001 SOL * 499,950.125 USDC ≈ 5,000,000,000 (same k!)
```

### 3. **Slippage**
The bigger your trade, the worse your price:
```
Swap 1 SOL: Get $49.875 (price: $49.875)
Swap 100 SOL: Get $4,761 (price: $47.61) ← worse!
Swap 1000 SOL: Get $33,333 (price: $33.33) ← much worse!
```

This is why the orderbook integration helps - it can fill large orders at better prices.

### 4. **Impermanent Loss**
If you're an LP and prices change a lot, you might have been better off just holding the tokens. This is called "impermanent loss" and is a key risk for LPs.

## Next Steps

Now that you understand `state.rs`, you should look at:

1. **instruction.rs** - See what commands users can send (Swap, Deposit, Withdraw, etc.)
2. **processor.rs** - See how those commands are executed
3. **math.rs** - See the actual price calculation formulas

## Questions to Test Your Understanding

1. What's the difference between `SwapOnly` and `OrderBookOnly` status?
2. Why does the AMM need 50 buy orders and 50 sell orders?
3. What happens to the `swap_fee` - where does it go?
4. How does the AMM know what price to place its orderbook orders at?
5. What's stored in `TargetOrders.placed_x` vs `AmmInfo.coin_vault`?

## Common Confusion Points

**Q: Why are there TWO fee types (swap_fee and trade_fee)?**
A: swap_fee is for direct AMM swaps. trade_fee is for when someone fills one of the AMM's orderbook orders. They can be different!

**Q: What's the difference between `coin` and `pc`?**
A: `coin` = base token (e.g., SOL), `pc` = quote token (e.g., USDC). "PC" stands for "price currency".

**Q: Why use fractions (numerator/denominator) instead of decimals?**
A: Blockchains don't have floating-point numbers! Fractions are exact and avoid rounding errors.

**Q: What's `nonce` in AmmInfo?**
A: It's a number used to generate a unique "program derived address" (PDA) that controls the pool's token vaults. Think of it as a seed for creating the pool's wallet address.

---

**Remember:** You're looking at production code that handles millions of dollars. Take your time, and don't worry if it doesn't all click immediately. Understanding AMMs is a journey!
