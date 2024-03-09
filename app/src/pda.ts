import { PublicKey } from "@metaplex-foundation/js";
import { PROGRAM_ID } from "./generated";
import {
    INDEX_PROTOCOL_STATE_PREFIX,
    MINTER_STATE_PREFIX,
    MINT_DATA_CONFIG_PREFIX,
} from "./constants";

export function findIndexProtocolState() {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(INDEX_PROTOCOL_STATE_PREFIX)],
        PROGRAM_ID,
    );
}

export function findMintDataConfig(tick: String) {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(MINT_DATA_CONFIG_PREFIX), Buffer.from(tick)],
        PROGRAM_ID,
    );
}

export function findMinterState(minterPubkey: PublicKey, tick: String) {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from(MINTER_STATE_PREFIX),
            Buffer.from(tick),
            minterPubkey.toBuffer(),
        ],
        PROGRAM_ID,
    );
}
