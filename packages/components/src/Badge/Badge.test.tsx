import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import examples from "./Badge.examples";

describe("Badge", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".badge")).toBeInTheDocument();
      });
    }
  });

  it("applies variant class", () => {
    const { container } = render(
      examples.items["Error"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".badge.error")).toBeInTheDocument();
  });

  it("defaults to muted variant", () => {
    const { container } = render(
      examples.items["Muted"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".badge.muted")).toBeInTheDocument();
  });
});
