import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { readJSON } from "fs-extra";
import path = require("node:path");
import { stringToU8a, u8aToHex } from '@polkadot/util';

async function main () {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });

    const keyring = new Keyring({ type: 'ecdsa' });
    const pair =  keyring.addFromUri('//Alice');


    const message = "My Message";
    const sig = Array.from(pair.sign(message));

    // Please paste contract address
    const contractAddress = "5EooexwWk5gkbDtWLk9goHS9WE9q1j2GYKJx4nTCDYspYx8N";
    const metadata = await readJSON(
        path.resolve(__dirname, "../contract/target/ink/verifier.json")
    );
    const contract = new ContractPromise(
        api,
        metadata,
        contractAddress
    );

    const storageDepositLimit = null;
    const gasLimit: any = api.registry.createType("WeightV2", {
        refTime: BigInt(10000000000),
        proofSize: BigInt(10000000000),
    });

    const queryResult = await contract.query["verify"](
        pair.address,
        {
          gasLimit,
          storageDepositLimit,
        },
        message,
        pair.address,
        sig,
    );

    console.log(`Query result: ${queryResult.output?.toString()}`);

    await api.disconnect();


    console.log("Test")
    const message2 = stringToU8a('this is our message');
    const signature = pair.sign(message2);
    const isValid = pair.verify(message2, signature, pair.publicKey);
    console.log(isValid)
}

main().catch(console.error).finally(() => process.exit());
