import { render, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Card.examples";

describe("Card", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".card")).toBeInTheDocument();
      });
    }
  });

  it("renders title in header", () => {
    const { container } = render(
      examples.items["WithTitle"]!.render() as React.ReactElement,
    );
    expect(
      within(container).getByText("Agent Performance"),
    ).toBeInTheDocument();
  });

  it("renders subtitle", () => {
    const { container } = render(
      examples.items["WithSubtitle"]!.render() as React.ReactElement,
    );
    expect(
      within(container).getByText("3 agents · 12 proposals"),
    ).toBeInTheDocument();
  });

  it("omits header when no title or subtitle", () => {
    const { container } = render(
      examples.items["Basic"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".header")).not.toBeInTheDocument();
  });
});
