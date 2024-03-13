import { PublicKey } from "@solana/web3.js";
import { findIndexProtocolState } from "../pda";
import {
    CloseMinterStatesInstructionAccounts,
    InitializeIndexProtocolStateInstructionAccounts,
    UpdateIndexProtocolStateInstructionArgs,
    createCloseMinterStatesInstruction,
    createInitializeIndexProtocolStateInstruction,
    createUpdateIndexProtocolStateInstruction,
} from "../generated";

export function intializeIndexProtocolState(
    superAuthority: PublicKey,
    counter: number,
) {
    const [indexProtocolState] = findIndexProtocolState();

    const initializeIndexProtocolStateInstructionAccounts: InitializeIndexProtocolStateInstructionAccounts =
        {
            superAuthority,
            indexProtocolState,
        };

    const initializeIpsIx = createInitializeIndexProtocolStateInstruction(
        initializeIndexProtocolStateInstructionAccounts,
        { counter },
    );
    return [initializeIpsIx];
}

export function updateIndexProtocolState(
    superAuthority: PublicKey,
    newSuperAuthority?: PublicKey,
    isPaused?: boolean,
    isOpen?: boolean,
) {
    const [indexProtocolState] = findIndexProtocolState();

    const updateIndexProtocolStateArgs: UpdateIndexProtocolStateInstructionArgs =
        {
            newSuperAuthority,
            isPaused,
            isOpen,
        };

    const updateIndexProtocolStateAccounts: InitializeIndexProtocolStateInstructionAccounts =
        {
            superAuthority,
            indexProtocolState,
        };

    const updateIpsIx = createUpdateIndexProtocolStateInstruction(
        updateIndexProtocolStateAccounts,
        updateIndexProtocolStateArgs,
    );
    return [updateIpsIx];
}

export function closeMinterStatesInstruction(
    superAuthority: PublicKey,
    accounts: PublicKey[],
) {
    const [indexProtocolState] = findIndexProtocolState();

    const closeMinterStateAccounts: CloseMinterStatesInstructionAccounts = {
        superAuthority,
        indexProtocolState,
    };

    closeMinterStateAccounts.anchorRemainingAccounts = accounts.map(
        (accountKey) => {
            return {
                pubkey: accountKey,
                isWritable: true,
                isSigner: false,
            };
        },
    );
    const closeMinterStateIx = createCloseMinterStatesInstruction(
        closeMinterStateAccounts,
    );
    return [closeMinterStateIx];
}
