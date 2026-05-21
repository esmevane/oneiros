import { Separator } from "./Separator";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Separator",
  description:
    "Visual divider built on Base UI's Separator primitive. " +
    "Renders an accessible horizontal or vertical rule.",

  properties: [
    {
      name: "orientation",
      type: '"horizontal" | "vertical"',
      default: '"horizontal"',
      description: "Direction of the separator.",
    },
  ],

  items: {
    Horizontal: {
      description: "Default horizontal separator.",
      code: `<div>
  <p>Above</p>
  <Separator />
  <p>Below</p>
</div>`,
      render: () => (
        <div>
          <p>Above</p>
          <Separator />
          <p>Below</p>
        </div>
      ),
    },
    Vertical: {
      description: "Vertical separator inside a flex row.",
      code: `<div style={{ display: "flex", alignItems: "center", gap: 8 }}>
  <span>Left</span>
  <Separator orientation="vertical" />
  <span>Right</span>
</div>`,
      render: () => (
        <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
          <span>Left</span>
          <Separator orientation="vertical" />
          <span>Right</span>
        </div>
      ),
    },
  },
};

export default examples;
