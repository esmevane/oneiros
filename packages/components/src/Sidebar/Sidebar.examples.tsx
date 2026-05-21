import { Sidebar } from "./Sidebar";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Sidebar",
  description:
    "Sticky-area content wrapper. Provides padding and vertical rhythm. " +
    "Designed to fill Layout.Aside.",

  items: {
    Basic: {
      description: "Sidebar with a few simple children.",
      code: `<Sidebar>
  <div>brand</div>
  <div>nav</div>
</Sidebar>`,
      render: () => (
        <Sidebar>
          <div>brand</div>
          <div>nav</div>
        </Sidebar>
      ),
    },
  },
};

export default examples;
