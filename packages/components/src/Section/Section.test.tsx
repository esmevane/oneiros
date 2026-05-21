import { render, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import examples from "./Section.examples";
import { Section } from "./Section";

describe("Section", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector(".container")).toBeInTheDocument();
      });
    }
  });

  it("starts expanded by default", () => {
    const { container } = render(
      examples.items["Expanded"]!.render() as React.ReactElement,
    );
    const header = container.querySelector(".header");
    expect(header?.getAttribute("data-state")).toBe("expanded");
    expect(header?.getAttribute("aria-expanded")).toBe("true");
    expect(container.querySelector(".body[hidden]")).not.toBeInTheDocument();
  });

  it("starts collapsed when collapsed prop is set", () => {
    const { container } = render(
      examples.items["Collapsed"]!.render() as React.ReactElement,
    );
    const header = container.querySelector(".header");
    expect(header?.getAttribute("data-state")).toBe("collapsed");
    expect(header?.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".body[hidden]")).toBeInTheDocument();
  });

  it("toggles state when header is clicked", async () => {
    const user = userEvent.setup();
    const { container } = render(
      <Section.Container>
        <Section.Header>Toggle me</Section.Header>
        <Section.Body>
          <p>Body</p>
        </Section.Body>
      </Section.Container>,
    );

    const header = within(container).getByRole("button", { name: "Toggle me" });
    expect(header.getAttribute("data-state")).toBe("expanded");

    await user.click(header);
    expect(header.getAttribute("data-state")).toBe("collapsed");

    await user.click(header);
    expect(header.getAttribute("data-state")).toBe("expanded");
  });

  it("throws when Section.Header is used outside Section.Container", () => {
    const errorSpy = vi
      .spyOn(console, "error")
      .mockImplementation(() => undefined);
    expect(() => render(<Section.Header>Orphan</Section.Header>)).toThrowError(
      /Section\.Container/,
    );
    errorSpy.mockRestore();
  });
});
