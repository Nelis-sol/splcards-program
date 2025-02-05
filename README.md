# About the project

<img width="400" alt="SPL Cards app" src="https://github.com/Nelis-sol/splcards-program/blob/main/spl-card-app.jpg">


## SPL Cards program

SPL Cards offers highly convenient and safe storage of crypto, using physical hardware card wallets. Simply tap the card with your mobile - and easily access your funds. Great for those well-versed in crypto, and for those that are new here. The card wallets come with a Solana/Anchor program that can be used to add extra security layers, and programmable policies. Policies can be set at the wallet level (all tokens) and on token level (applicable to that specific token mint).

The program works by enabling users to wrap their tokens into a wrapped token. The wrapped token is on the token2022 standard and user Token Extensions (including transfer hooks). On every transaction, these transfer hooks are triggered and checks are done to ensure if the transaction is valid (and adheres to the user-defined policies).

The SPL Cards program was part of my work with Turbine and can be found here: [WBA repository](https://github.com/Web3-Builders-Alliance/Nelis-sol_Sol_1Q24/tree/main/capstone/splcards).

<br />
**Optional SPL Cardspolicies include:**
  * **Add 2nd signer** (transactions need 2 signers)
  * **Create allow list with destination addresses** (only transactions to these addresses are allowed)
  * **Create blacklisted addresses** (transactions to these addresses will be blocked)
  * **Create spending time window** (transactions outside of a certain time window will be blocked)
  * **Create spending limit** (transactions will be blocked if the amount spent exceeds the limit, or require a 2nd signer)

<br />

## Devnet and published IDL
This program is deployed on Solana devnet with program id: `6jPXVk78mLJq3MAz24gasxmYmV2f3bYDd5Rp5zK92tew`.<br /><br />
The IDL is published onchain:
https://solscan.io/account/6jPXVk78mLJq3MAz24gasxmYmV2f3bYDd5Rp5zK92tew?cluster=devnet

<br />

## Built with

- [x] **Rust**
- [x] **Anchor**  
- [x] **Token Extensions** (transfer hooks)

<br />

____

<br />

## Install & run

### 1. Install Rust, Cargo
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
```

If you have Rust already installed, make sure to update to the latest stable version.
```
$ rustup update
```
<br />

### 2. Install Anchor, AVM
```
$ cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
$ avm install 0.29.0
$ avm use 0.29.0
```
<br />

### 3. Deploy program on devnet
Update the program id if necessary (in the `lib.rs` and `anchor.toml` files).

```
$ anchor build
$ anchor deploy
```
<br />

### 4. Run tests
```
$ anchor test
```

<br />

# Contact
Contact me via email (nelis.sol@protonmail.com) or on X (@nelis-sol)

<br /><br />

<img width="800" alt="SPL Card" src="https://github.com/Nelis-sol/splcards-program/blob/main/spl-cards-v2.jpeg">
&nbsp;&nbsp;

