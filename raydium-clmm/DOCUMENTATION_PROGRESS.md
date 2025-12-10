# Raydium CLMM Documentation Progress

## Overview

This is the **Raydium CLMM (Concentrated Liquidity Market Maker)** - Raydium's V3 AMM, inspired by Uniswap V3.

**Key Difference from V1 (raydium-amm):**
- V1: Traditional AMM with liquidity spread across entire price range (0 to ∞)
- V3: Concentrated liquidity in specific price ranges (e.g., $48-$52)
- Result: 20-2000x better capital efficiency!

## Architecture

### Technology Stack
- **Framework:** Anchor (Solana's framework for building programs)
- **Language:** Rust
- **Structure:** Much cleaner than V1 (Anchor handles a lot of boilerplate)

### Key Components

1. **lib.rs** - Main program entry point
   - Defines all instruction handlers
   - Uses Anchor's `#[program]` macro
   - Much simpler than V1's processor.rs

2. **states/** - Data structures
   - `pool_state.rs` - Pool configuration and state
   - `position_state.rs` - Individual liquidity positions (NFTs)
   - `tick_state.rs` - Tick data for price ranges
   - `oracle_state.rs` - Price oracle integration

3. **instructions/** - Instruction handlers
   - `create_pool.rs` - Initialize a new pool
   - `open_position.rs` - Create a liquidity position
   - `increase_liquidity.rs` - Add more liquidity
   - `decrease_liquidity.rs` - Remove liquidity
   - `swap.rs` - Execute token swaps
   - Many more...

4. **libraries/** - Helper functions
   - `tick_math.rs` - Tick and price calculations
   - `liquidity_math.rs` - Liquidity calculations
   - `swap_math.rs` - Swap calculations
   - `fixed_point_64.rs` - Q64.64 fixed-point math

## Key Concepts

### 1. **Positions as NFTs**

Unlike V1 where you get fungible LP tokens, in CLMM each position is a unique NFT:

```
Traditional AMM (V1):
Deposit → Get 100 LP tokens (fungible)

CLMM (V3):
Deposit → Get Position NFT #12345
  - Lower tick: -1000
  - Upper tick: 1000
  - Liquidity: 50000
  - Fees earned: $25.50
```

### 2. **Tick System**

Prices are divided into discrete ticks:

```
Tick -2: $48.00
Tick -1: $49.00
Tick  0: $50.00  ← Current price
Tick +1: $51.00
Tick +2: $52.00

Your position: Tick -1 to Tick +1
Active range: $49 - $51
```

### 3. **Square Root Price (Q64.64)**

CLMM uses square root price for efficiency:

```
Price = 50
Sqrt Price = 7.071...

Stored as Q64.64 fixed-point:
sqrt_price_x64 = 7.071... × 2^64
```

### 4. **Multiple Fee Tiers**

Same token pair can have multiple pools with different fees:

```
SOL/USDC:
- Pool 1: 0.01% fee (for tight ranges)
- Pool 2: 0.05% fee (standard)
- Pool 3: 0.30% fee (for wider ranges)
- Pool 4: 1.00% fee (for volatile pairs)
```

## Main Instructions

### Pool Management

1. **create_amm_config** - Create fee tier configuration
2. **create_pool** - Initialize a new pool
3. **update_pool_status** - Enable/disable trading

### Position Management

4. **open_position** - Create new liquidity position (NFT)
5. **increase_liquidity** - Add more liquidity to position
6. **decrease_liquidity** - Remove liquidity from position
7. **close_position** - Close position and burn NFT

### Trading

8. **swap** - Single-pool swap
9. **swap_v2** - Token2022 support
10. **swap_router_base_in** - Multi-hop swaps

### Rewards

11. **initialize_reward** - Set up reward program
12. **set_reward_params** - Update reward parameters
13. **collect_remaining_rewards** - Collect unclaimed rewards

### Fees

14. **collect_protocol_fee** - Collect protocol fees
15. **collect_fund_fee** - Collect fund fees

## File Structure

```
raydium-clmm/
├── programs/amm/src/
│   ├── lib.rs                    # Main program (all instructions)
│   ├── error.rs                  # Error definitions
│   ├── states/                   # Data structures
│   │   ├── pool_state.rs         # Pool configuration
│   │   ├── position_state.rs     # Position (NFT) data
│   │   ├── tick_state.rs         # Tick data
│   │   ├── tick_array_state.rs   # Tick array storage
│   │   ├── oracle_state.rs       # Price oracle
│   │   └── config.rs             # AMM configuration
│   ├── instructions/             # Instruction handlers
│   │   ├── create_pool.rs
│   │   ├── open_position.rs
│   │   ├── increase_liquidity.rs
│   │   ├── decrease_liquidity.rs
│   │   ├── swap.rs
│   │   └── ... (20+ more)
│   ├── libraries/                # Math and utilities
│   │   ├── tick_math.rs          # Tick calculations
│   │   ├── liquidity_math.rs     # Liquidity calculations
│   │   ├── swap_math.rs          # Swap calculations
│   │   └── fixed_point_64.rs     # Q64.64 math
│   └── util/                     # Helper utilities
└── UNDERSTANDING_CLMM.md         # Conceptual guide
```

## Comparison: V1 vs V3

| Feature | V1 (raydium-amm) | V3 (raydium-clmm) |
|---------|------------------|-------------------|
| **Framework** | Raw Solana | Anchor |
| **Liquidity** | Full range | Concentrated |
| **LP Token** | Fungible | NFT |
| **Capital Efficiency** | 1x | 20-2000x |
| **Complexity** | Medium | High |
| **Orderbook** | Yes (hybrid) | No (pure AMM) |
| **Fee Tiers** | Fixed | Multiple |
| **Code Size** | ~300KB | ~150KB (Anchor) |
| **Best For** | Passive LPs | Active LPs |

## Documentation Status

### ✅ Completed
- [x] UNDERSTANDING_CLMM.md - Conceptual guide
- [x] DOCUMENTATION_PROGRESS.md - This file

### 🚧 In Progress
- [ ] lib.rs - Main program comments
- [ ] states/ - State structure comments
- [ ] instructions/ - Instruction handler comments
- [ ] libraries/ - Math library comments

### 📝 Planned
- [ ] CLMM_MATH_GUIDE.md - Deep dive into the math
- [ ] POSITION_MANAGEMENT_GUIDE.md - How to manage positions
- [ ] SWAP_GUIDE.md - How swaps work in CLMM

## Key Differences from V1

### 1. **Anchor Framework**

V1 (Raw Solana):
```rust
// Manual account parsing
let amm_info = next_account_info(account_iter)?;
let authority = next_account_info(account_iter)?;
// ... 20 more accounts

// Manual validation
if amm_info.owner != program_id {
    return Err(ProgramError::InvalidAccountOwner);
}
```

V3 (Anchor):
```rust
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolState>,
    pub authority: Signer<'info>,
    // Anchor handles validation automatically!
}
```

### 2. **Position Representation**

V1:
```rust
// Fungible LP tokens
pub lp_mint: Pubkey,
pub lp_amount: u64,
```

V3:
```rust
// NFT positions
pub struct PersonalPositionState {
    pub nft_mint: Pubkey,
    pub pool_id: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    // Each position is unique!
}
```

### 3. **Price Representation**

V1:
```rust
// Simple ratio
let price = pc_amount / coin_amount;
```

V3:
```rust
// Square root price (Q64.64)
pub sqrt_price_x64: u128,
// More efficient for range calculations!
```

## Learning Path

1. **Start with V1 (raydium-amm)** if you're new to AMMs
   - Simpler concepts
   - Traditional x × y = k formula
   - Good foundation

2. **Then move to V3 (raydium-clmm)** for advanced concepts
   - Concentrated liquidity
   - Tick system
   - Position management

## Resources

- **Conceptual Guide:** `UNDERSTANDING_CLMM.md`
- **Uniswap V3 Whitepaper:** https://uniswap.org/whitepaper-v3.pdf
- **Raydium Docs:** https://docs.raydium.io/
- **Anchor Book:** https://book.anchor-lang.com/

---

**Status:** Documentation in progress. CLMM is more complex than V1 but offers much better capital efficiency!
