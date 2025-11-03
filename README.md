# ⚖️ PATS Escrow — Task 2

This project demonstrates how to send **PATS tokens** (Token-2022) into an escrow account, define a recipient, and later release those tokens under set conditions.

--- STILL UNDER Development ..

## 🔧 Setup Summary

| Component | Address | Description |
|------------|----------|-------------|
| **PATS Mint (Token-2022)** | `jBhgEBgg6uxxWnRzpw18p35Z1n48U9LAuW4z2USDUn6` | Custom SPL Token created in Task 2.1 |
| **Vault PDA** | `AajyzPcxuS9R5SvyJB1zEwxJqRkuVLp663THFBZAcVEa` | Holds locked tokens in Task 2.2 |
| **Escrow PDA** | *(to be generated on deploy)* | Temporary program-owned account |
| **Token Program** | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | SPL Token-2022 Program |
| **Network** | Devnet | |

---

## 🪙 1. Initialize Escrow

This creates the escrow PDA, its token account, and transfers tokens from the initializer (you) to the escrow.

```bash
anchor build
anchor deploy --provider.cluster devnet
anchor test --provider.cluster devnet