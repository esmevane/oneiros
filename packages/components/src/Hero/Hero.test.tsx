import { render, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Hero.examples";

describe("Hero", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".container")).toBeInTheDocument();
      });
    }
  });

  it("renders title as h1", () => {
    const { container } = render(
      examples.items["Full"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("h1.title")).toBeInTheDocument();
  });

  it("renders all three parts when provided", () => {
    const { container } = render(
      examples.items["Full"]!.render() as React.ReactElement,
    );
    expect(within(container).getByText("Introduction")).toBeInTheDocument();
    expect(within(container).getByText("Oneiros")).toBeInTheDocument();
    expect(
      within(container).getByText("Continuity for AI agents."),
    ).toBeInTheDocument();
  });
});
