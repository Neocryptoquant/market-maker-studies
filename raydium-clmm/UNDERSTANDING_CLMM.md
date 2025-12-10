# Understanding Raydium CLMM (Concentrated Liquidity Market Maker)

## What is CLMM?

**CLMM** stands for **Concentrated Liquidity Market Maker** - this is Raydium's V3 AMM, inspired by Uniswap V3.

### The Evolution

```
Raydium V1 (raydium-amm):
- Traditional AMM (x × y = k)
- Liquidity spread across entire price range (0 to ∞)
- Hybrid with orderbook

Raydium V3 (raydium-clmm):
- Concentrated Liquidity AMM
- Liquidity concentrated in specific price ranges
- Much more capital efficient
- No orderbook integration (pure AMM)
```

## Key Difference: Concentrated Liquidity

### Traditional AMM (V1)

```
Price Range: $0 ────────────────────────────────── $∞
Liquidity:    ████████████████████████████████████
              (spread thin across entire range)

Your $10,000: Provides liquidity for ALL prices
Efficiency:   LOW (most liquidity never used)
```

### Concentrated Liquidity (V3)

```
Price Range: $0 ──────[████]────────────────────── $∞
                      $45  $55
                   (concentrated here)

Your $10,000: Provides liquidity ONLY for $45-$55
Efficiency:   HIGH (all liquidity actively used)
```

## Real-World Example

### Scenario: SOL/USDC Pool

**Current Price:** $50 per SOL

**Traditional AMM (V1):**
```
You deposit: $10,000 (5 SOL + $250 USDC)
Liquidity covers: $0 to $∞
Active liquidity: ~$500 (only 5% is actually used!)
Capital efficiency: 5%
```

**Concentrated Liquidity (V3):**
```
You deposit: $10,000 (5 SOL + $250 USDC)
You choose range: $45 to $55 (±10% from current price)
Liquidity covers: ONLY $45 to $55
Active liquidity: $10,000 (100% is used!)
Capital efficiency: 2000% (20x better!)
```

## How It Works

### 1. **Price Ranges (Ticks)**

Instead of providing liquidity for all prices, you choose a range:

```
Example: SOL is $50

Conservative LP:
├─────────────────────────────┤
$40                          $60
(Wide range, safer, lower fees)

Aggressive LP:
      ├───────┤
      $48    $52
(Narrow range, riskier, higher fees)
```

### 2. **Ticks System**

Prices are divided into discrete "ticks":

```
Tick -2: $48.00
Tick -1: $49.00
Tick  0: $50.00  ← Current price
Tick +1: $51.00
Tick +2: $52.00
```

You provide liquidity between two ticks (e.g., Tick -1 to Tick +1).

### 3. **Active vs Inactive Liquidity**

```
Your Range: $48 - $52
Current Price: $50

Status: ACTIVE ✓
You earn fees!

---

Your Range: $48 - $52
Current Price: $55

Status: INACTIVE ❌
You earn NO fees (price moved out of range)
```

## Key Concepts

### 1. **Position NFTs**

In CLMM, your liquidity position is represented as an **NFT** (Non-Fungible Token).

**Why?**
- Each position has unique parameters (price range, amounts)
- Can't be fungible like traditional LP tokens
- You can have multiple positions with different ranges

```
Traditional AMM:
You get: 100 LP tokens (fungible, all identical)

CLMM:
You get: Position NFT #12345
  - Range: $48-$52
  - Liquidity: 10,000
  - Fees earned: $50
```

### 2. **Tick Spacing**

Not every price can be a tick. Ticks are spaced at intervals:

```
Fee Tier: 0.01% → Tick spacing: 1
Fee Tier: 0.05% → Tick spacing: 10
Fee Tier: 0.30% → Tick spacing: 60
Fee Tier: 1.00% → Tick spacing: 200

Higher fees = Wider tick spacing = Less precision
```

### 3. **Fee Tiers**

CLMM supports multiple fee tiers for the same pair:

```
SOL/USDC Pools:

Pool 1: 0.01% fee (for stablecoins, very tight ranges)
Pool 2: 0.05% fee (for correlated assets)
Pool 3: 0.30% fee (for most pairs)
Pool 4: 1.00% fee (for exotic/volatile pairs)
```

### 4. **Liquidity Depth**

```
Traditional AMM:
Price: $50
Liquidity: ████████████████████ (constant everywhere)

CLMM:
Price: $50
Liquidity: 
$40  ██
$45  ████
$48  ████████
$50  ████████████████  ← Most liquidity here!
$52  ████████
$55  ████
$60  ██
```

## Advantages of CLMM

### 1. **Capital Efficiency**

```
Same $10,000 investment:

Traditional AMM:
- Effective liquidity: $500
- Fees earned: $10/day

CLMM (concentrated):
- Effective liquidity: $10,000
- Fees earned: $200/day (20x more!)
```

### 2. **Customizable Strategies**

**Conservative (Wide Range):**
```
Range: $40 - $60
Risk: Low (price rarely exits range)
Fees: Lower (less concentrated)
Best for: Passive LPs
```

**Aggressive (Narrow Range):**
```
Range: $49 - $51
Risk: High (price often exits range)
Fees: Higher (very concentrated)
Best for: Active LPs
```

**Multi-Position:**
```
Position 1: $45 - $55 (50% of capital)
Position 2: $48 - $52 (30% of capital)
Position 3: $49 - $51 (20% of capital)

Strategy: Layered liquidity for different scenarios
```

### 3. **Better for Stablecoins**

```
USDC/USDT Pool:

Traditional AMM:
- Range: $0 to $∞
- Wasted liquidity: 99.9%

CLMM:
- Range: $0.99 to $1.01
- Wasted liquidity: 0%
- Capital efficiency: 1000x better!
```

## Disadvantages of CLMM

### 1. **Complexity**

```
Traditional AMM:
1. Deposit tokens
2. Get LP tokens
3. Done!

CLMM:
1. Choose price range
2. Choose fee tier
3. Monitor position
4. Rebalance when price moves
5. Collect fees manually
6. Much more active management!
```

### 2. **Impermanent Loss Risk**

```
If price moves out of your range:

Your position becomes:
- 100% one token (the less valuable one)
- 0% the other token
- Earning NO fees

Example:
Range: $48 - $52
Price moves to: $60

Your position: 100% USDC, 0% SOL
You missed: All the SOL price increase!
```

### 3. **Gas Costs**

```
Traditional AMM:
- Deposit: 1 transaction
- Withdraw: 1 transaction

CLMM:
- Create position: 1 transaction
- Collect fees: 1 transaction (periodic)
- Rebalance: 2 transactions (withdraw + deposit)
- More transactions = More fees
```

## CLMM vs Traditional AMM

| Feature | Traditional AMM | CLMM |
|---------|----------------|------|
| **Liquidity Range** | 0 to ∞ | Custom range |
| **Capital Efficiency** | Low | Very High |
| **Complexity** | Simple | Complex |
| **Management** | Passive | Active |
| **LP Token** | Fungible (ERC-20) | NFT (ERC-721) |
| **Fee Earnings** | Constant | Variable (depends on range) |
| **Impermanent Loss** | Moderate | Can be higher |
| **Best For** | Passive investors | Active traders |

## Technical Architecture

### Key Components

1. **Pool State**
   - Current price (sqrt price)
   - Current tick
   - Total liquidity
   - Fee growth trackers

2. **Position State**
   - Owner (NFT holder)
   - Lower tick
   - Upper tick
   - Liquidity amount
   - Fees owed

3. **Tick State**
   - Liquidity delta (how much liquidity starts/ends here)
   - Fee growth outside
   - Initialized flag

### The Math

**Square Root Price:**
```
Instead of storing price directly, CLMM stores sqrt(price)

Why? Makes the math more efficient!

Price = 50
Sqrt Price = 7.071...

All calculations use sqrt price internally
```

**Liquidity Formula:**
```
L = Δy / Δ(sqrt(P))

Where:
L = Liquidity
Δy = Change in token Y
Δ(sqrt(P)) = Change in square root price
```

## Use Cases

### 1. **Stablecoin Pools**
```
USDC/USDT:
Range: $0.998 - $1.002
Efficiency: 500x better than traditional AMM
Perfect for: Arbitrageurs, high-volume traders
```

### 2. **Correlated Assets**
```
ETH/wstETH:
Range: 0.95 - 1.05 ratio
Efficiency: 20x better
Perfect for: Yield farmers
```

### 3. **Volatile Pairs**
```
SOL/USDC:
Multiple positions at different ranges
Rebalance as price moves
Perfect for: Active market makers
```

## Summary

**CLMM is like upgrading from a shotgun to a sniper rifle:**

- **Traditional AMM:** Spray liquidity everywhere (inefficient but simple)
- **CLMM:** Concentrate liquidity precisely (efficient but complex)

**When to use CLMM:**
- ✅ You can actively manage positions
- ✅ You understand price ranges
- ✅ You want maximum capital efficiency
- ✅ You're providing liquidity for stablecoins or correlated assets

**When to use Traditional AMM:**
- ✅ You want passive income
- ✅ You don't want to monitor positions
- ✅ You're new to liquidity providing
- ✅ You want simplicity

---

**Next:** Let's dive into the code and see how Raydium implements this!
