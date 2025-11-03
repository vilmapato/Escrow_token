import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowToken } from "../target/types/escrow_token";

describe("escrow-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.escrowToken as Program<EscrowToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
