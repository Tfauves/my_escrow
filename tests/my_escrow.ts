import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyEscrow } from "../target/types/my_escrow";

describe("my_escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyEscrow as Program<MyEscrow>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
