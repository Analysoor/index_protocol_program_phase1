import {
    PublicKey,
    ComputeBudgetProgram,
    Connection,
    SystemProgram,
} from "@solana/web3.js";
import {
    Config,
    DeployMintConfigInstructionAccounts,
    DeployMintConfigInstructionArgs,
    IndexProtocolState,
    MintDataConfig,
    MintInstructionAccounts,
    MintPhase,
    MintType,
    createDeployMintConfigInstruction,
    createMintInstruction,
} from "../generated";
import {
    findIndexProtocolState,
    findMintDataConfig,
    findMinterState,
} from "../pda";
import {
    MINT_SIZE,
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    createInitializeMintInstruction,
    createMintToInstruction,
    getAssociatedTokenAddressSync,
    getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";

export async function deploy(
    connection: Connection,
    authority: PublicKey,
    splTokenName: string,
    tick: string,
    fundsRecipient: PublicKey,
    mintType: MintType,
    price: number, // in lamports
    amountPerMint: number,
    startDate: number, // in unix timestamp
    supply: number,
    filter: string[],
) {
    const [indexProtocolState] = findIndexProtocolState();
    const [mintDataConfig] = findMintDataConfig(tick);
    console.log(mintDataConfig);
    console.log(indexProtocolState);

    // get current counter and 1 to it
    const indexProtocolStateData = await IndexProtocolState.fromAccountAddress(
        connection,
        indexProtocolState,
    );
    const configCounter = Number(indexProtocolStateData.counter) + 1;

    let config: Config = {
        price,
        amountPerMint,
        counter: configCounter,
        supply,
        tick,
        liquidityBootstrapping: false,
        filter,
        startDate,
    };
    const deployMintConfigInstructionArgs: DeployMintConfigInstructionArgs = {
        config,
        fundsRecipient,
        splTokenName,
        mintPhase: MintPhase.Minting,
        mintType,
    };
    const deployMintConfigInstructionAccounts: DeployMintConfigInstructionAccounts =
        {
            authority,
            indexProtocolState,
            mintDataConfig,
        };

    const deployIx = createDeployMintConfigInstruction(
        deployMintConfigInstructionAccounts,
        deployMintConfigInstructionArgs,
    );

    return [deployIx];
}

export async function mint(
    connection: Connection,
    buyer: PublicKey,
    burnableMint: PublicKey,
    tick: string,
) {
    const [indexProtocolState] = findIndexProtocolState();
    const [mintDataConfig] = findMintDataConfig(tick);
    const [minterState] = findMinterState(buyer, tick);
    const mintDataConfigeData = await MintDataConfig.fromAccountAddress(
        connection,
        mintDataConfig,
    );
    const fundsRecipient = mintDataConfigeData.fundsRecipient;

    const requiredBalance = await getMinimumBalanceForRentExemptMint(
        connection,
    );

    const burnableMintAta = getAssociatedTokenAddressSync(burnableMint, buyer);

    const createMintInstructions = [
        SystemProgram.createAccount({
            fromPubkey: buyer,
            newAccountPubkey: burnableMint,
            space: MINT_SIZE,
            lamports: requiredBalance,
            programId: TOKEN_PROGRAM_ID,
        }),
        createInitializeMintInstruction(
            burnableMint, // mint account Address
            0, // number of decimals of the new mint
            buyer, // mint account Authority
            buyer, // freeze Authority usually mint authority
            TOKEN_PROGRAM_ID,
        ),
        createAssociatedTokenAccountInstruction(
            buyer, //Payer
            burnableMintAta, // owner associated token account
            buyer, // token owner
            burnableMint, // mint account
        ),
        createMintToInstruction(
            burnableMint, // mint account
            burnableMintAta, // destination associated token account
            buyer, // authority
            1, //number of tokens
        ),
    ];

    const mintInstructionAccounts: MintInstructionAccounts = {
        feePayer: buyer,
        indexProtocolState,
        mintDataConfig,
        minterState,
        burnableMint,
        burnableMintAta,
        fundsRecipient,
        splTokenProgram: TOKEN_PROGRAM_ID,
    };

    const mintIx = createMintInstruction(mintInstructionAccounts, {
        tick,
    });
    return [...createMintInstructions, mintIx];
}
