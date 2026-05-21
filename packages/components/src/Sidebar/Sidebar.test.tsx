import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import examples from "./Sidebar.examples";

describe("Sidebar", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".sidebar")).toBeInTheDocument();
      });
    }
  });
});
