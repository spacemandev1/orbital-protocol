import { PublicKey } from "@solana/web3.js";

export const ORBITAL_PROGRAM_ID = new PublicKey(
  "Orbita1111111111111111111111111111111111111"
);

export function findPlatformPda(
  programId: PublicKey = ORBITAL_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("platform")],
    programId
  );
}

export function findLaunchPda(
  creator: PublicKey,
  mint: PublicKey,
  programId: PublicKey = ORBITAL_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("launch"),
      creator.toBuffer(),
      mint.toBuffer()
    ],
    programId
  );
}
