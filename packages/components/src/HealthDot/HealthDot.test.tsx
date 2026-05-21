import { render, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./HealthDot.examples";

describe("HealthDot", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".health")).toBeInTheDocument();
      });
    }
  });

  it("applies status class to indicator", () => {
    const { container } = render(
      examples.items["Critical"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".indicator.critical")).toBeInTheDocument();
  });

  it("renders label when children provided", () => {
    const { container } = render(
      examples.items["Current"]!.render() as React.ReactElement,
    );
    expect(within(container).getByText("Current")).toBeInTheDocument();
  });

  it("renders without label when no children", () => {
    const { container } = render(
      examples.items["DotOnly"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".label")).not.toBeInTheDocument();
  });
});
