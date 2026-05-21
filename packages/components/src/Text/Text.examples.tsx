import { Text } from "./Text";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Text",
  description:
    "Token-driven typography primitive. Composes font size, family, " +
    "weight, and color through design tokens.",

  properties: [
    {
      name: "size",
      type: "TextSize",
      description: "Font size token. Maps to --font-size-*.",
    },
    {
      name: "font",
      type: '"sans" | "serif" | "mono"',
      description: "Font family.",
    },
    {
      name: "weight",
      type: '"normal" | "medium" | "semibold" | "bold"',
      description: "Font weight.",
    },
    {
      name: "color",
      type: '"default" | "muted" | "secondary"',
      description: "Text color.",
    },
    { name: "italic", type: "boolean", description: "Italic style." },
    {
      name: "uppercase",
      type: "boolean",
      description: "Uppercase with wide letter spacing.",
    },
    {
      name: "as",
      type: "ElementType",
      description: "Rendered element. Defaults to span.",
    },
  ],

  items: {
    Default: {
      description: "Default text rendering.",
      code: `<Text>Default text</Text>`,
      render: () => <Text>Default text</Text>,
    },
    Mono: {
      description: "Monospace text at small size.",
      code: `<Text font="mono" size="3xs" color="muted">agent.process</Text>`,
      render: () => (
        <Text font="mono" size="3xs" color="muted">
          agent.process
        </Text>
      ),
    },
    Heading: {
      description: "Large bold heading.",
      code: `<Text as="h2" size="lg" weight="bold" font="sans">Dashboard</Text>`,
      render: () => (
        <Text as="h2" size="lg" weight="bold" font="sans">
          Dashboard
        </Text>
      ),
    },
    Uppercase: {
      description: "Uppercase label style.",
      code: `<Text size="3xs" uppercase weight="semibold" color="muted">Section Title</Text>`,
      render: () => (
        <Text size="3xs" uppercase weight="semibold" color="muted">
          Section Title
        </Text>
      ),
    },
  },
};

export default examples;
