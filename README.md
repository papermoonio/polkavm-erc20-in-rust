# ERC20 in Rust for PolkaVM

The repo gives an example how to write a ERC20 contract in Rust, which compatible with Solidity interface.

## Installation

### Rust

### Polkatool

### Node, Typescript and Yarn

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
