import { Badge } from "./Badge";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Badge",
  description:
    "Status indicator with a leading dot. Semantic color variants " +
    "communicate state at a glance.",

  properties: [
    {
      name: "variant",
      type: '"primary" | "accent" | "success" | "warn" | "info" | "error" | "muted"',
      default: '"muted"',
      description: "Semantic color variant for the badge.",
    },
  ],

  items: {
    Primary: {
      description: "Active or in-progress state.",
      code: '<Badge variant="primary">Active</Badge>',
      render: () => <Badge variant="primary">Active</Badge>,
    },
    Accent: {
      description: "Highlighted or featured item.",
      code: '<Badge variant="accent">Featured</Badge>',
      render: () => <Badge variant="accent">Featured</Badge>,
    },
    Success: {
      description: "Healthy or completed state.",
      code: '<Badge variant="success">Healthy</Badge>',
      render: () => <Badge variant="success">Healthy</Badge>,
    },
    Warn: {
      description: "Needs attention.",
      code: '<Badge variant="warn">Drifting</Badge>',
      render: () => <Badge variant="warn">Drifting</Badge>,
    },
    Info: {
      description: "Informational context.",
      code: '<Badge variant="info">Queued</Badge>',
      render: () => <Badge variant="info">Queued</Badge>,
    },
    Error: {
      description: "Critical or failed state.",
      code: '<Badge variant="error">Critical</Badge>',
      render: () => <Badge variant="error">Critical</Badge>,
    },
    Muted: {
      description: "Default / neutral state.",
      code: '<Badge variant="muted">Inactive</Badge>',
      render: () => <Badge variant="muted">Inactive</Badge>,
    },
  },
};

export default examples;
