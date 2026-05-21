import { render, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Separator.examples";

describe("Separator", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".separator")).toBeInTheDocument();
      });
    }
  });

  it("renders surrounding content", () => {
    const { container } = render(
      examples.items["Vertical"]!.render() as React.ReactElement,
    );
    expect(within(container).getByText("Left")).toBeInTheDocument();
    expect(within(container).getByText("Right")).toBeInTheDocument();
  });
});
