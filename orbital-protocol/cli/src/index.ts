import { Command } from "commander";
import { Keypair } from "@solana/web3.js";
import { findLaunchPda, findPlatformPda } from "../../sdk/src/index.js";

const program = new Command();

program
  .name("orbital")
  .description("Orbital protocol CLI")
  .version("0.1.0");

program
  .command("platform")
  .description("Print the platform PDA")
  .action(() => {
    const [platform, bump] = findPlatformPda();
    console.log(JSON.stringify({
      platform: platform.toBase58(),
      bump
    }, null, 2));
  });

program
  .command("create")
  .description("Generate example creator/mint keys and derive launch PDA")
  .action(() => {
    const creator = Keypair.generate();
    const mint = Keypair.generate();

    const [launch, bump] = findLaunchPda(
      creator.publicKey,
      mint.publicKey
    );

    console.log(JSON.stringify({
      creator: creator.publicKey.toBase58(),
      mint: mint.publicKey.toBase58(),
      launch: launch.toBase58(),
      bump
    }, null, 2));
  });

program.parse();
