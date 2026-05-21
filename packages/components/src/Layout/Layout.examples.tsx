import { Layout } from "./Layout";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Layout",
  description:
    "Page chrome — sticky aside + scrolling main, two-column grid with " +
    "a max-width constraint. The shape every full-page surface lives in.",

  properties: [
    {
      name: "Layout.Container",
      type: "ReactNode",
      description: "Grid wrapper; renders <div>.",
    },
    {
      name: "Layout.Aside",
      type: "ReactNode",
      description: "Sticky aside slot; renders <aside>.",
    },
    {
      name: "Layout.Main",
      type: "ReactNode",
      description: "Scrolling main slot; renders <main>.",
    },
  ],

  items: {
    Basic: {
      description: "Container with an aside and a main.",
      code: `<Layout.Container>
  <Layout.Aside>aside</Layout.Aside>
  <Layout.Main>main</Layout.Main>
</Layout.Container>`,
      render: () => (
        <Layout.Container>
          <Layout.Aside>aside</Layout.Aside>
          <Layout.Main>main</Layout.Main>
        </Layout.Container>
      ),
    },
  },
};

export default examples;
