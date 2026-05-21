import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Layout.examples";

describe("Layout", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".container")).toBeInTheDocument();
      });
    }
  });

  it("renders semantic aside + main elements", () => {
    const { container } = render(
      examples.items["Basic"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("aside.aside")).toBeInTheDocument();
    expect(container.querySelector("main.main")).toBeInTheDocument();
  });
});
