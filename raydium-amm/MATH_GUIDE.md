# Math.rs - Understanding the AMM Calculator

## What We've Documented

We've added comprehensive comments to **math.rs**, the mathematical engine of the Raydium AMM.

## Key Functions Explained

### 1. **Swap Calculations (THE CORE)**

#### `swap_token_amount_base_in()`
**Scenario:** "I have 100 USDC. How much SOL do I get?"

**Formula:**
```
amount_out = (reserves_out × amount_in) / (reserves_in + amount_in)
```

**Example:**
```
Pool: 10,000 SOL and 500,000 USDC
You swap in: 1,000 USDC
You get: (10,000 × 1,000) / (500,000 + 1,000) = 19.96 SOL
```

#### `swap_token_amount_base_out()`
**Scenario:** "I want exactly 10 SOL. How much USDC do I need?"

**Formula:**
```
amount_in = (amount_out × reserves_in) / (reserves_out - amount_out)
```

**Example:**
```
Pool: 10,000 SOL and 500,000 USDC
You want: 10 SOL
You need: (10 × 500,000) / (10,000 - 10) = 500.25 USDC
```

**Important:** Uses `checked_ceil_div` (rounds UP) to protect the pool from losing money!

### 2. **Type Conversions**

- `to_u128()` - Convert u64 → u128 (always safe)
- `to_u64()` - Convert u128 → u64 (might fail if too big)

### 3. **Rounding**

- **Floor:** Round down (1.9 → 1) - Used when taking from user
- **Ceiling:** Round up (1.1 → 2) - Used when giving to user

This ensures the pool never loses money due to rounding!

### 4. **Large Integer Types**

- **U128:** 128-bit integer (0 to 340 undecillion)
- **U256:** 256-bit integer (for extreme precision)

**Why?** Blockchains don't have decimals (no 1.5, 3.14). Everything is whole numbers!

## The x × y = k Formula Explained

This is the **constant product formula** - the heart of every AMM.

### Before Swap
```
coin_reserves × pc_reserves = k (constant)
10,000 SOL × 500,000 USDC = 5,000,000,000
```

### After Swap (you swap in 1,000 USDC)
```
new_coin × new_pc = k (same constant!)
9,980.02 SOL × 501,000 USDC ≈ 5,000,000,000
```

**The pool automatically adjusts** to keep k constant!

## Why This Math Matters

### Example: Slippage

```
Small trade (1 USDC):
amount_out = (10,000 × 1) / (500,000 + 1) = 0.01999 SOL
Price: $50.00 per SOL ✓

Large trade (100,000 USDC):
amount_out = (10,000 × 100,000) / (500,000 + 100,000) = 1,666.67 SOL
Price: $60.00 per SOL ❌ (20% worse!)
```

**The bigger your trade, the worse your price!** This is slippage.

## Other Important Functions

### Decimal Normalization
- `normalize_decimal()` - Convert native decimals to system decimals
- `restore_decimal()` - Convert back

**Why?** SOL has 9 decimals, USDC has 6. We need a common format!

### Orderbook Conversions
- `convert_in_price()` - Serum price → internal price
- `convert_out_price()` - Internal price → Serum price
- `convert_in_vol()` - Serum volume → internal volume
- `convert_out_vol()` - Internal volume → Serum volume

**Why?** Raydium's internal format differs from Serum/OpenBook's format.

### Lot Size Functions
- `floor_lot()` - Round down to nearest lot size
- `ceil_lot()` - Round up to nearest lot size

**Why?** Orderbooks trade in "lots" (minimum units), not arbitrary amounts.

## Key Takeaways

1. **No Decimals:** Everything is integers (whole numbers)
2. **x × y = k:** The constant product formula drives all swaps
3. **Rounding Matters:** Always round in favor of the pool
4. **Precision:** Use U128/U256 to avoid overflow
5. **Conversions:** Constantly converting between different decimal formats

## The Math in Action

When you swap on Raydium:

```
1. User sends: "Swap 100 USDC for SOL"
2. Math.rs calculates: swap_token_amount_base_in(100, ...)
3. Result: "You get 1.996 SOL"
4. Pool updates: 
   - SOL: 10,000 → 9,998.004
   - USDC: 500,000 → 500,100
5. Check: 9,998.004 × 500,100 ≈ 5,000,000,000 ✓
```

## Files Completed

✅ **lib.rs** - Program structure
✅ **entrypoint.rs** - Entry point
✅ **state.rs** - Data structures
✅ **math.rs** - Calculations

## Next: instruction.rs

Now that you understand the math, we'll look at **instruction.rs** to see what commands users can send to the AMM!
