import { expect } from "chai";
import { quoteBuy, quoteSell } from "../sdk/src/quotes.js";

describe("orbital math", () => {
  it("quotes a buy", () => {
    const out = quoteBuy(
      30_000_000_000n,
      1_000_000_000_000_000n,
      1_000_000_000n
    );

    expect(out > 0n).to.eq(true);
  });

  it("quotes a sell", () => {
    const out = quoteSell(
      30_000_000_000n,
      1_000_000_000_000_000n,
      1_000_000_000_000n
    );

    expect(out > 0n).to.eq(true);
  });
});
