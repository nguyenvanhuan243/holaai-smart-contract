## Basic smart contract for pump.fun

You can check frontend and backend repo as well.

You can contact me if you want a better product.

New features on an updated version
- All handled in smart contracts
Pumpfun uses backend client code to fetch buy transaction and create raydium pool.
I've handled all that parts on smart contract to enhance the security and availability.

- Added some launch phases
There's some phases to launch a token like `Presale`.
If the users want to snipe a token, they can bid for the token in `Presale` phase before `Launch`.

- Raydium/Meteora
Token launchers can migrate their tokens to Raydium or Migrate as their wish after the curve is completed.


# Pump.fun Smart Contract
This is the Rust/Anchor smart contract for Pump.fun, which includes functionalities to add virtual liquidity pools (LP), remove LP, and create Raydium Pools.

### Transactions
You can review the transactions for removing virtual LP and creating Raydium Pools in this smart contract:
https://explorer.solana.com/tx/4L6MWmtV1ZsT8NFfbtu68ZYyYVbpvZ4iynJhPdZw8jESi28TxwojjTFs88Q5QRdNUb297aWfkKcoYP9Ya8npx8AV?cluster=devnet

### Best IDE
- https://beta.solpg.io/


### Faucet solana
- https://faucet.solana.com

```
cargo build
```

```
cargo publish -p bonding_curve
```

# Fix bug
```
cargo login "API_TOKEN"
```

Now in a terminal, execute cargo login <API_KEY>. You’re now logged into Crates.io and can publish!

# Rust package
Get token: https://crates.io/settings/tokens


# Pump.fun Smart Contract
Tools for Pump.fun
This is the Rust/Anchor smart contract for Pump.fun, which includes functionalities to add virtual liquidity pools (LP), remove LP, and create Raydium Pools.

### Best IDE
- https://beta.solpg.io/

## Create token repo
- Create token repo: https://github.com/nguyenvanhuan243/pumpfun-create-token-sol
- Video create token: https://drive.google.com/file/d/1yoSPI1iEjbDZDQPrbUxWCEj45hrk_U2Q/view?usp=sharing

## Bonding curve repo
- Video Deploy: https://drive.google.com/file/d/1FXKkmb0mUBhEXtNvQbRu_sKo-nCCp6Do/view?usp=sharing
- Call smart contract function
![alt text](image.png)

