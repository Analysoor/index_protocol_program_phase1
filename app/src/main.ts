import {
    Keypair,
    Connection,
    Transaction,
    LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
    deploy,
    intializeIndexProtocolState,
    mint,
    updateIndexProtocolState,
} from "./custom";
import superAuthorityRawKey from "supauth.json";
import buyerRawKey from "buyer.json";
import { findIndexProtocolState, findMinterState } from "./pda";
import { IndexProtocolState, MintType, MinterState } from "./generated";

const connection = new Connection("", "confirmed");

const superAuthority = Keypair.fromSecretKey(
    Uint8Array.from(superAuthorityRawKey),
);
const buyer = Keypair.fromSecretKey(Uint8Array.from(buyerRawKey));

async function intiProtocol() {
    const instructions = intializeIndexProtocolState(
        superAuthority.publicKey,
        0,
    );
    const txn = new Transaction().add(...instructions);
    const txn_id = await connection.sendTransaction(txn, [superAuthority]);

    console.log(`https://solscan.io/tx/${txn_id}`);
    console.log(`${txn_id}`);
}

async function checkProtocolState() {
    const [indexProtocolState] = findIndexProtocolState();
    const indexProtocolStateData = await IndexProtocolState.fromAccountAddress(
        connection,
        indexProtocolState,
    );
    console.log(indexProtocolStateData.pretty());
}


async function uptadeProtocolState() {
    const instructions = updateIndexProtocolState(
        superAuthority.publicKey,
        superAuthority.publicKey,
        true,
        false,
    );
    
    const txn = new Transaction().add(...instructions);
    const txn_id = await connection.sendTransaction(txn, [superAuthority]);


    console.log(`https://solscan.io/tx/${txn_id}`);
    console.log(`${txn_id}`);
}

async function deployMint() {
    const currentTimeInSeconds = Math.floor(new Date().getTime() / 1000);
    console.log(currentTimeInSeconds);
    const startDate = currentTimeInSeconds +60;
    const instructions = await deploy(
        connection,
        superAuthority.publicKey,
        "zero",
        "zero",
        superAuthority.publicKey,
        MintType.SplToken,
        0.001 * LAMPORTS_PER_SOL,
        1,
        startDate,
        100,
        ["testingfilter"],
    );
    const txn = new Transaction().add(...instructions);
    const txn_id = await connection.sendTransaction(txn, [superAuthority]);


    console.log(`https://solscan.io/tx/${txn_id}`);
    console.log(`${txn_id}`);
}




async function runMint() {
    const burnableMint = Keypair.generate();
    const instructions = await mint(
        connection,
        buyer.publicKey,
        burnableMint.publicKey,
        "zero",
    );
    const txn = new Transaction().add(...instructions);
    const txn_id = await connection.sendTransaction(txn, [buyer, burnableMint]);
    console.log(`https://solscan.io/tx/${txn_id}`);
    console.log(`${txn_id}`);
}

async function getMintState() {

    const [minterState] = findMinterState(buyer.publicKey, "zero")
    const minterStateData = await MinterState.fromAccountAddress(connection, minterState);
    console.log(minterStateData.pretty())
}


// intiProtocol()
//checkProtocolState()
// uptadeProtocolState()
// checkProtocolState()
// deployMint()
// runMint();
// getMintState();