import { Stack } from "./Stack";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Stack",
  description:
    "Vertical flex layout with token-driven gap. " +
    "The default composition primitive for arranging elements in a column.",

  properties: [
    {
      name: "gap",
      type: '"3xs" | "xxs" | "xs" | "sm" | "md" | "lg" | "xl" | "xxl" | "3xl"',
      description:
        "Gap between children, mapped to space tokens. Defaults to md.",
    },
  ],

  items: {
    Default: {
      description: "Stack with default gap.",
      code: `<Stack>
  <div>First</div>
  <div>Second</div>
  <div>Third</div>
</Stack>`,
      render: () => (
        <Stack>
          <div>First</div>
          <div>Second</div>
          <div>Third</div>
        </Stack>
      ),
    },
    TightGap: {
      description: "Stack with tight spacing.",
      code: `<Stack gap="xs">
  <div>Tight first</div>
  <div>Tight second</div>
</Stack>`,
      render: () => (
        <Stack gap="xs">
          <div>Tight first</div>
          <div>Tight second</div>
        </Stack>
      ),
    },
    WideGap: {
      description: "Stack with generous spacing.",
      code: `<Stack gap="xl">
  <div>Wide first</div>
  <div>Wide second</div>
</Stack>`,
      render: () => (
        <Stack gap="xl">
          <div>Wide first</div>
          <div>Wide second</div>
        </Stack>
      ),
    },
  },
};

export default examples;
