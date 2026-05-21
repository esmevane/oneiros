import { Button } from "./Button";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Button",
  description:
    "Clickable action trigger built on Base UI's Button primitive. " +
    "Provides accessibility, keyboard handling, and disabled focus management.",

  properties: [
    {
      name: "variant",
      type: '"primary" | "accent" | "ghost" | "danger"',
      default: '"primary"',
      description: "Visual style variant.",
    },
    {
      name: "size",
      type: '"sm" | "md"',
      default: '"md"',
      description: "Button size.",
    },
    {
      name: "disabled",
      type: "boolean",
      default: "false",
      description: "Disables interaction. From Base UI.",
    },
  ],

  items: {
    Primary: {
      description: "Default action button.",
      code: "<Button>Save changes</Button>",
      render: () => <Button>Save changes</Button>,
    },
    Accent: {
      description: "Alternate emphasis for secondary actions.",
      code: '<Button variant="accent">Upgrade plan</Button>',
      render: () => <Button variant="accent">Upgrade plan</Button>,
    },
    Ghost: {
      description: "Transparent background, text-only.",
      code: '<Button variant="ghost">Cancel</Button>',
      render: () => <Button variant="ghost">Cancel</Button>,
    },
    Danger: {
      description: "Destructive action warning.",
      code: '<Button variant="danger">Delete</Button>',
      render: () => <Button variant="danger">Delete</Button>,
    },
    Small: {
      description: "Compact variant for dense layouts.",
      code: '<Button size="sm">Details</Button>',
      render: () => <Button size="sm">Details</Button>,
    },
    Disabled: {
      description: "Non-interactive state.",
      code: "<Button disabled>Unavailable</Button>",
      render: () => <Button disabled>Unavailable</Button>,
    },
  },
};

export default examples;
