
import { ABI } from "./erc20";
import { readFileSync } from "fs";
import { ethers, JsonRpcProvider, Wallet, Contract } from "ethers";
import { config } from "dotenv";

function generateRandomEthersWallet(provider: JsonRpcProvider) {
    const account = ethers.Wallet.createRandom();
    const wallet = new ethers.Wallet(account.privateKey, provider);
    return wallet;
}

function uint8ArrayToHex(arr: Uint8Array): string {
    return Array.from(arr)
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
}

function getBytecode(): string {
    const path = "../erc20.polkavm";
    const bytecode = readFileSync(path);
    const bytecodeHex = uint8ArrayToHex(bytecode);
    const validBytecode = bytecodeHex.replace("0x", "");
    return validBytecode;
}

async function deploy(provider: JsonRpcProvider, etherWallet: Wallet) {
    const bytecode = getBytecode();
    const factory = new ethers.ContractFactory(ABI, bytecode, etherWallet);
    const contract = await factory.deploy(
        "name",
        "symbol",
        BigInt(18),
        BigInt(1234),
    );

    await contract.waitForDeployment();
    const contractAddress = contract.target.toString();
    console.log("contract address is: ", contractAddress)
    return contractAddress;
}

async function main() {
    const url = "https://westend-asset-hub-eth-rpc.polkadot.io";
    const provider = new ethers.JsonRpcProvider(url);
    config();
    let privateKey = process.env.AH_PRIV_KEY || "";
    const walletClient = new Wallet(privateKey, provider);

    const contractAddress = await deploy(provider, walletClient)

    const contract = new Contract(
        contractAddress,
        ABI,
        walletClient,
    );

    // wallet from env private key
    const walletAddress = await walletClient.getAddress();

    // generate a random wallet for transferFrom test
    const randomWalletClient = generateRandomEthersWallet(provider);
    const recipientAddress = await randomWalletClient.getAddress();

    // transfer native token to new wallet for sending transferFrom later
    const tx = {
        to: recipientAddress,
        value: BigInt(1e18),
    };
    const txResponse = await walletClient.sendTransaction(tx);
    await txResponse.wait();
    const recipientBalance = await provider.getBalance(recipientAddress);
    console.log("recipientBalance is: ", recipientBalance);

    const name = await contract.name();
    const symbol = await contract.symbol();
    const decimals = await contract.decimals();
    const totalSupply = await contract.totalSupply();
    const balance = await contract.balanceOf(walletAddress);
    const allowance = await contract.allowance(walletAddress, recipientAddress);

    // check init status
    console.log("Name: ", name);
    console.log("Symbol: ", symbol);
    console.log("Decimals: ", decimals);
    console.log("Total Supply: ", totalSupply);
    console.log("Balance: ", balance);
    console.log("Allowance: ", allowance);

    // test transfer erc20
    const transferAmount = BigInt(1e18);
    const transferTx = await contract.transfer(recipientAddress, transferAmount);
    await transferTx.wait();

    const myBalanceAfterTransfer = await contract.balanceOf(walletAddress);
    const randomBalanceAfterTransfer = await contract.balanceOf(recipientAddress);
    console.log(
        "My balance after transfer: ",
        myBalanceAfterTransfer / BigInt(1e18),
    );
    console.log(
        "Random wallet balance after transfer: ",
        randomBalanceAfterTransfer / BigInt(1e18),
    );

    // test approve erc20
    const approveAmount = BigInt(2e18);
    const approveTx = await contract.approve(recipientAddress, approveAmount);
    await approveTx.wait();
    const approveAllowance = await contract.allowance(
        walletAddress,
        recipientAddress,
    );
    console.log("Approve allowance: ", approveAllowance / BigInt(1e18));



    const contract2 = new Contract(
        contractAddress,
        ABI,
        randomWalletClient,
    );

    // test transferFrom, just transfer back to the original wallet
    const transferFromTx = await contract2.transferFrom(
        walletAddress,
        walletAddress,
        approveAmount / BigInt(2),
    );
    await transferFromTx.wait();
    const allowanceAfterTransferFrom = await contract.allowance(
        walletAddress,
        recipientAddress,
    );
    console.log(
        "Approve allowance after transferFrom: ",
        allowanceAfterTransferFrom / BigInt(1e18),
    );
}

main()