import { Link } from "./Link";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Link",
  description:
    "Semantic link primitives. Link is for inline prose; Link.Nav is " +
    "block-level navigation with active and sub variants.",

  properties: [
    {
      name: "Link.Nav.active",
      type: "boolean",
      default: "false",
      description: "Adds the active treatment (left-border indicator).",
    },
    {
      name: "Link.Nav.sub",
      type: "boolean",
      default: "false",
      description: "Indented mono treatment for nested nav children.",
    },
  ],

  items: {
    Inline: {
      description: "Inline link inside prose.",
      code: `<p>Read the <Link href="#">overview</Link> first.</p>`,
      render: () => (
        <p>
          Read the <Link href="#">overview</Link> first.
        </p>
      ),
    },
    Nav: {
      description: "Navigation link.",
      code: `<Link.Nav href="#">Concepts</Link.Nav>`,
      render: () => <Link.Nav href="#">Concepts</Link.Nav>,
    },
    NavActive: {
      description: "Active navigation link with left-border indicator.",
      code: `<Link.Nav href="#" active>Continuity</Link.Nav>`,
      render: () => (
        <Link.Nav href="#" active>
          Continuity
        </Link.Nav>
      ),
    },
    NavSub: {
      description: "Indented sub-navigation link.",
      code: `<Link.Nav href="#" sub>Vocabulary</Link.Nav>`,
      render: () => (
        <Link.Nav href="#" sub>
          Vocabulary
        </Link.Nav>
      ),
    },
  },
};

export default examples;
