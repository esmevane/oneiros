import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Text.examples";

describe("Text", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".text")).toBeInTheDocument();
      });
    }
  });

  it("renders as span by default", () => {
    const { container } = render(
      examples.items["Default"]!.render() as React.ReactElement,
    );
    const text = container.querySelector(".text");
    expect(text?.tagName).toBe("SPAN");
  });

  it("renders as specified element", () => {
    const { container } = render(
      examples.items["Heading"]!.render() as React.ReactElement,
    );
    const heading = container.querySelector("h2.text");
    expect(heading).toBeInTheDocument();
  });
});
