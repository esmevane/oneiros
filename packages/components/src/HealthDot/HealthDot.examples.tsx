import { HealthDot } from "./HealthDot";
import type { ExampleCollection } from "../example";

const examples: ExampleCollection = {
  name: "Health Dot",
  description:
    "Pulsing status indicator for agent or system health. " +
    "Four states from active-green through inactive-gray.",

  properties: [
    {
      name: "status",
      type: '"current" | "drifting" | "critical" | "inactive"',
      default: '"inactive"',
      description: "Health state controlling color and pulse animation.",
    },
  ],

  items: {
    Current: {
      description: "System is healthy and active.",
      code: '<HealthDot status="current">Current</HealthDot>',
      render: () => <HealthDot status="current">Current</HealthDot>,
    },
    Drifting: {
      description: "System is diverging from expected state.",
      code: '<HealthDot status="drifting">Drifting</HealthDot>',
      render: () => <HealthDot status="drifting">Drifting</HealthDot>,
    },
    Critical: {
      description: "System requires immediate attention.",
      code: '<HealthDot status="critical">Critical</HealthDot>',
      render: () => <HealthDot status="critical">Critical</HealthDot>,
    },
    Inactive: {
      description: "System is offline or dormant.",
      code: '<HealthDot status="inactive">Inactive</HealthDot>',
      render: () => <HealthDot status="inactive">Inactive</HealthDot>,
    },
    DotOnly: {
      description: "Status dot without a label.",
      code: '<HealthDot status="current" />',
      render: () => <HealthDot status="current" />,
    },
  },
};

export default examples;
