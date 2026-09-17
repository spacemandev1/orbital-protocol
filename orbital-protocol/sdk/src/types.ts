import type { PublicKey } from "@solana/web3.js";

export type CreateLaunchInput = {
  creator: PublicKey;
  mint: PublicKey;
  name: string;
  symbol: string;
  uri: string;
  virtualSolReserve: bigint;
  virtualTokenReserve: bigint;
  creatorFeeBps: number;
};

export type LaunchSnapshot = {
  virtualSolReserve: bigint;
  virtualTokenReserve: bigint;
};
