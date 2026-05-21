import { Hero } from "./Hero";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Hero",
  description:
    "Page intro composition: eyebrow + title + subtitle. Hero.Container " +
    "wraps the trio with vertical rhythm and padding.",

  properties: [
    {
      name: "Hero.Eyebrow",
      type: "ReactNode",
      description: "Small uppercase label above the title.",
    },
    {
      name: "Hero.Title",
      type: "ReactNode",
      description: "Large heading. Renders <h1>.",
    },
    {
      name: "Hero.Subtitle",
      type: "ReactNode",
      description: "Italic supporting text below the title.",
    },
  ],

  items: {
    Full: {
      description: "All three parts.",
      code: `<Hero.Container>
  <Hero.Eyebrow>Introduction</Hero.Eyebrow>
  <Hero.Title>Oneiros</Hero.Title>
  <Hero.Subtitle>Continuity for AI agents.</Hero.Subtitle>
</Hero.Container>`,
      render: () => (
        <Hero.Container>
          <Hero.Eyebrow>Introduction</Hero.Eyebrow>
          <Hero.Title>Oneiros</Hero.Title>
          <Hero.Subtitle>Continuity for AI agents.</Hero.Subtitle>
        </Hero.Container>
      ),
    },
    TitleOnly: {
      description: "Just a title — eyebrow and subtitle are optional.",
      code: `<Hero.Container>
  <Hero.Title>Concepts</Hero.Title>
</Hero.Container>`,
      render: () => (
        <Hero.Container>
          <Hero.Title>Concepts</Hero.Title>
        </Hero.Container>
      ),
    },
  },
};

export default examples;
