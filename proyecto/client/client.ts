import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MicroGrant as Program<any>;
  const wallet = provider.wallet;

  const nombreProyecto = "Proyecto_Ingenieria_2026";
  const metaSol = 0.5 * anchor.web3.LAMPORTS_PER_SOL; // Meta de 0.5 SOL

  const [proyectoPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("grant"), Buffer.from(nombreProyecto)],
    program.programId
  );

  console.log("-----------------------------------------");
  console.log("🌟 CROWDFUNDING: MICRO-GRANT");
  console.log("Proyecto PDA:", proyectoPDA.toString());

  // 1. CREAR PROYECTO
  try {
    await program.methods
      .crearProyecto(nombreProyecto, new anchor.BN(metaSol))
      .accounts({
        proyecto: proyectoPDA,
        autor: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("✅ Proyecto creado exitosamente.");
  } catch (e) {
    console.log("ℹ️ El proyecto ya existe.");
  }

  // 2. DONAR
  try {
    const donacion = 0.1 * anchor.web3.LAMPORTS_PER_SOL;
    console.log(`Enviando donación de 0.1 SOL...`);
    await program.methods
      .donar(new anchor.BN(donacion))
      .accounts({
        proyecto: proyectoPDA,
        donador: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("✅ Donación enviada a la bóveda del proyecto.");
  } catch (err) {
    console.error("❌ Error al donar:", err);
  }

  // 3. CONSULTAR ESTADO
  const data = await program.account.project.fetch(proyectoPDA);
  console.log("-----------------------------------------");
  console.log(`PROYECTO: ${data.nombre}`);
  console.log(`META: ${data.meta.toNumber() / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  console.log(`RECAUDADO: ${data.recaudado.toNumber() / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  
  if (data.recaudado.toNumber() >= data.meta.toNumber()) {
      console.log("🎉 ¡META ALCANZADA! Ya puedes retirar los fondos.");
  } else {
      console.log("⏳ Aún falta para llegar a la meta.");
  }
  console.log("-----------------------------------------");
}

main();
