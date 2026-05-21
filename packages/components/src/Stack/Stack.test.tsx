import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Stack.examples";

describe("Stack", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".stack")).toBeInTheDocument();
      });
    }
  });

  it("applies gap via custom property", () => {
    const { container } = render(
      examples.items["TightGap"]!.render() as React.ReactElement,
    );
    const stack = container.querySelector(".stack") as HTMLElement;
    expect(stack.style.getPropertyValue("--stack-gap")).toBe("var(--space-xs)");
  });
});
