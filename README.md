# ERC20 in Rust for PolkaVM

The repo gives an example how to write a ERC20 contract in Rust, which compatible with Solidity interface.

## Installation

### Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Polkatool

```shell
cargo install polkatool
```

### Node, Typescript and Yarn

It is recommented to install via nvm.

```shell
# Download and install nvm:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

# in lieu of restarting the shell
\. "$HOME/.nvm/nvm.sh"

# Download and install Node.js:
nvm install 22

# Verify the Node.js version:
node -v # Should print "v22.15.0".
nvm current # Should print "v22.15.0".

# Verify npm version:
npm -v # Should print "10.9.2".
```

```shell
npm install -g typescript
npm install -g yarn
```

## Build

Run ./build.sh, you will get a erc20.polkavm file for deployment and test

## Test

1. go to ts folder, `mv env.example .env`. input a key with some token in westend asset hub.
2. run `yarn` to install dependency
3. run `yarn erc20` to deploy erc20 contract and interact with it.

```shell
user@user-X870-EAGLE-WIFI7:~/github/papermoon/polkavm-erc20-in-rust/ts$ yarn erc20
yarn run v1.22.22
$ ts-node main.ts
contract address is:  0xAA00B7111CB9dd5074Db32E1B7917dD01ceb39dE
recipientBalance is:  1000000000000000000n
Name:  name
Symbol:  symbol
Decimals:  18n
Total Supply:  1234000000000000000000n
Balance:  1234000000000000000000n
Allowance:  0n
My balance after transfer:  1233n
Random wallet balance after transfer:  1n
Approve allowance:  2n
Approve allowance after transferFrom:  1n
Done in 38.25s.
```
