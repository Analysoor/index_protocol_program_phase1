import {
    Keypair,
    Connection,
    Transaction,
    LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { mint } from "./custom";
import buyerRawKey from "yourkeypair.json";
import { findMinterState } from "./pda";
import { MinterState } from "./generated";
const connection = new Connection("", "confirmed");

const buyer = Keypair.fromSecretKey(Uint8Array.from(buyerRawKey));

async function runMint(tick: string) {
    const burnableMint = Keypair.generate();
    const instructions = await mint(
        connection,
        buyer.publicKey,
        burnableMint.publicKey,
        tick,
    );
    const txn = new Transaction().add(...instructions);
    const txn_id = await connection.sendTransaction(txn, [buyer, burnableMint]);
    console.log(`https://solscan.io/tx/${txn_id}`);
    console.log(`${txn_id}`);
}

async function getMintState() {
    const [minterState] = findMinterState(buyer.publicKey, "zero");
    const minterStateData = await MinterState.fromAccountAddress(
        connection,
        minterState,
    );
    console.log(minterStateData.pretty());
}

runMint("tbtb");
// getMintState()
