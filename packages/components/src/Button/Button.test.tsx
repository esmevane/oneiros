import { render, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { Button } from "./Button";
import examples from "./Button.examples";

describe("Button", () => {
  describe("examples render without crashing", () => {
    for (const [name, example] of Object.entries(examples.items)) {
      it(name, () => {
        const { container } = render(example.render() as React.ReactElement);
        expect(container.querySelector("button")).toBeInTheDocument();
      });
    }
  });

  it("forwards click events", async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();

    const { container } = render(<Button onClick={onClick}>Click me</Button>);

    await user.click(
      within(container).getByRole("button", { name: "Click me" }),
    );
    expect(onClick).toHaveBeenCalledOnce();
  });

  it("respects disabled state", async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();

    const { container } = render(
      <Button disabled onClick={onClick}>
        Nope
      </Button>,
    );

    await user.click(within(container).getByRole("button", { name: "Nope" }));
    expect(onClick).not.toHaveBeenCalled();
  });

  it("applies variant class", () => {
    const { container } = render(
      examples.items["Danger"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".button.danger")).toBeInTheDocument();
  });

  it("applies size class", () => {
    const { container } = render(
      examples.items["Small"]!.render() as React.ReactElement,
    );
    expect(container.querySelector(".button.sm")).toBeInTheDocument();
  });
});
