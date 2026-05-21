import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Link.examples";

describe("Link", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector("a")).toBeInTheDocument();
      });
    }
  });

  it("inline link uses the link class", () => {
    const { container } = render(
      examples.items["Inline"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("a.link")).toBeInTheDocument();
  });

  it("nav link uses the nav class", () => {
    const { container } = render(
      examples.items["Nav"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("a.nav")).toBeInTheDocument();
  });

  it("active variant adds active class", () => {
    const { container } = render(
      examples.items["NavActive"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("a.nav.active")).toBeInTheDocument();
  });

  it("sub variant adds sub class", () => {
    const { container } = render(
      examples.items["NavSub"]!.render() as React.ReactElement,
    );
    expect(container.querySelector("a.nav.sub")).toBeInTheDocument();
  });
});
