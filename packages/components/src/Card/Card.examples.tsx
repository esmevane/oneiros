import { Card } from "./Card";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Card",
  description:
    "Bordered container with optional header. Foundation for " +
    "stat cards, component demos, and content sections.",

  properties: [
    {
      name: "title",
      type: "string",
      description: "Card header title text.",
    },
    {
      name: "subtitle",
      type: "string",
      description: "Secondary text displayed beside the title.",
    },
    {
      name: "header",
      type: "ReactNode",
      description: "Custom header content — replaces title and subtitle.",
    },
  ],

  items: {
    Basic: {
      description: "Body-only card for simple content.",
      code: "<Card><p>Card content goes here.</p></Card>",
      render: () => (
        <Card>
          <p>Card content goes here.</p>
        </Card>
      ),
    },
    WithTitle: {
      description: "Card with a title header.",
      code: `<Card title="Agent Performance">
  <p>Overview metrics and health indicators.</p>
</Card>`,
      render: () => (
        <Card title="Agent Performance">
          <p>Overview metrics and health indicators.</p>
        </Card>
      ),
    },
    WithSubtitle: {
      description: "Card with title and subtitle.",
      code: `<Card title="World Building" subtitle="3 agents · 12 proposals">
  <p>Team workspace details.</p>
</Card>`,
      render: () => (
        <Card title="World Building" subtitle="3 agents · 12 proposals">
          <p>Team workspace details.</p>
        </Card>
      ),
    },
  },
};

export default examples;
