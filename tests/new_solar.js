const { TOKEN_PROGRAM_ID } = require("@solana/spl-token");

const anchor = require("@project-serum/anchor");
const serumCmn = require("@project-serum/common");

const Keypair = anchor.web3.Keypair;
const PublicKey = anchor.web3.PublicKey;

describe("new_solar", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NewSolar;

  const initializeAccount = Keypair.generate();
  const vaultAccount = Keypair.generate();

  let mint = PublicKey.default;
  let vaultAuthority = PublicKey.default;
  let ownerTokenAccount = PublicKey.default;

  it("Setup Test State", async () => {
    const [_mint, _ownerTokenAccount] = await serumCmn.createMintAndVault(
      provider,
      new anchor.BN(1000000),
      provider.wallet.publicKey,
      0
    );

    const [_vaultAuthority, _vault_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("authority"))],
      program.programId
    );

    mint = _mint;
    vaultAuthority = _vaultAuthority;
    ownerTokenAccount = _ownerTokenAccount;
  });

  it("Is initialized!", async () => {
    console.log(TOKEN_PROGRAM_ID, "TOKEN_PROGRAM_ID");
    await program.rpc.initialize(
      new anchor.BN(1000),
      [
        {
          apr: new anchor.BN(12),
          minAmount: new anchor.BN(321),
          duration: new anchor.BN(121),
        },
      ],
      {
        accounts: {
          initializeAccount: initializeAccount.publicKey,
          vaultAccount: vaultAccount.publicKey,
          mint,
          vaultAuthority,
          ownerTokenAccount,
          owner: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [initializeAccount, vaultAccount],
      }
    );  
  });
});
