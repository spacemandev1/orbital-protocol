export function quoteBuy(
  virtualSol: bigint,
  virtualTokens: bigint,
  solIn: bigint
): bigint {
  if (virtualSol <= 0n || virtualTokens <= 0n || solIn <= 0n) {
    throw new Error("invalid reserves/input");
  }

  const k = virtualSol * virtualTokens;
  const newSol = virtualSol + solIn;
  const newTokens = k / newSol;

  return virtualTokens - newTokens;
}

export function quoteSell(
  virtualSol: bigint,
  virtualTokens: bigint,
  tokenIn: bigint
): bigint {
  if (virtualSol <= 0n || virtualTokens <= 0n || tokenIn <= 0n) {
    throw new Error("invalid reserves/input");
  }

  const k = virtualSol * virtualTokens;
  const newTokens = virtualTokens + tokenIn;
  const newSol = k / newTokens;

  return virtualSol - newSol;
}
