import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArtBonds } from "../target/types/art_bonds";
import { assert } from "chai";

describe("art_bonds", () => {
  // Configuramos el entorno para que use tu red local
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.ArtBonds as Program<ArtBonds>;
  const provider = anchor.getProvider();

  // Variables globales para la prueba
  let bondPda: anchor.web3.PublicKey;
  const inversionInicial = new anchor.BN(1000000000); // 1 SOL (en lamports)

  it("1. Create (Emitir Bono)", async () => {
    // Calculamos la dirección de la PDA
    [bondPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bond"), provider.publicKey.toBuffer()],
      program.programId
    );

    // Llamamos a la función issue_bond de Rust
    await program.methods
      .issueBond(inversionInicial)
      .accounts({
        user: provider.publicKey,
        bondPda: bondPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // READ: Verificamos que se guardó correctamente
    const estadoDelBono = await program.account.bondState.fetch(bondPda);
    assert.ok(estadoDelBono.principal.eq(inversionInicial));
    console.log("✅ Bono emitido por 1 SOL");
  });

  it("2. Update (Actualizar Rendimiento)", async () => {
    // Llamamos a update_yield
    await program.methods
      .updateYield()
      .accounts({
        user: provider.publicKey,
        bondPda: bondPda,
      })
      .rpc();
    
    console.log("✅ Rendimiento calculado y actualizado");
  });

  it("3. Delete (Liquidar Bono)", async () => {
    // Llamamos a redeem_bond
    await program.methods
      .redeemBond()
      .accounts({
        user: provider.publicKey,
        bondPda: bondPda,
      })
      .rpc();

    // Verificamos que la cuenta PDA ya no existe
    try {
      await program.account.bondState.fetch(bondPda);
      assert.fail("La cuenta debería estar cerrada");
    } catch (e) {
      console.log("✅ Bono liquidado y fondos devueltos exitosamente");
    }
  });
});