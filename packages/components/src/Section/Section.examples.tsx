import { Section } from "./Section";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Section",
  description:
    "Labeled, collapsible content block. Section.Header is the toggle, " +
    'Section.Body the panel. State ("expanded" | "collapsed") lives in ' +
    "Section.Container and reaches sub-parts via context.",

  properties: [
    {
      name: "collapsed",
      type: "boolean",
      default: "false",
      description: "Sets initial state to collapsed.",
    },
  ],

  items: {
    Expanded: {
      description: "Default — starts expanded.",
      code: `<Section.Container>
  <Section.Header>Concepts</Section.Header>
  <Section.Body>
    <p>Body content.</p>
  </Section.Body>
</Section.Container>`,
      render: () => (
        <Section.Container>
          <Section.Header>Concepts</Section.Header>
          <Section.Body>
            <p>Body content.</p>
          </Section.Body>
        </Section.Container>
      ),
    },
    Collapsed: {
      description: "Starts collapsed via the collapsed prop.",
      code: `<Section.Container collapsed>
  <Section.Header>Reference</Section.Header>
  <Section.Body>
    <p>Hidden until toggled.</p>
  </Section.Body>
</Section.Container>`,
      render: () => (
        <Section.Container collapsed>
          <Section.Header>Reference</Section.Header>
          <Section.Body>
            <p>Hidden until toggled.</p>
          </Section.Body>
        </Section.Container>
      ),
    },
  },
};

export default examples;
