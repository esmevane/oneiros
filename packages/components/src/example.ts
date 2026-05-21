import type { ReactNode } from "react";

interface Example {
  description: string;
  code: string;
  render: () => ReactNode;
}

interface Property {
  name: string;
  type: string;
  default?: string;
  description: string;
}

interface ExampleCollection {
  name: string;
  description: string;
  properties?: Property[];
  items: Record<string, Example>;
}

export type { Example, ExampleCollection, Property };
