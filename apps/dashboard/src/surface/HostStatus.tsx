import { useEffect } from "react";
import {
  Badge,
  Button,
  Card,
  HealthDot,
  Stack,
  Text,
} from "@oneiros/components";
import type { BadgeVariant } from "@oneiros/components";
import { useEvents, useSelector } from "../shell";
import type { HostInfo } from "../machine";

type Status = "pristine" | "requesting" | "success" | "idle" | "failure";

const healthByStatus: Record<
  Status,
  "current" | "drifting" | "critical" | "inactive"
> = {
  pristine: "inactive",
  requesting: "drifting",
  success: "current",
  idle: "current",
  failure: "critical",
};

const badgeByStatus: Record<Status, BadgeVariant> = {
  pristine: "muted",
  requesting: "info",
  success: "success",
  idle: "muted",
  failure: "error",
};

/** Pure surface — reads the requests.hostInfo slice and renders through
 *  the shared components. Card.header is already a flex row with gap, so
 *  header children just lay out naturally; the body uses Stack for the
 *  vertical rhythm. */
export function HostStatus() {
  const events = useEvents() as {
    requests: { hostInfo: { init: () => void } };
  };

  const matches = useSelector((snapshot) => ({
    pristine: snapshot.matches("requests.hostInfo.pristine"),
    requesting: snapshot.matches("requests.hostInfo.requesting"),
    success: snapshot.matches("requests.hostInfo.success"),
    idle: snapshot.matches("requests.hostInfo.idle"),
    failure: snapshot.matches("requests.hostInfo.failure"),
  }));

  const values = useSelector(
    (snapshot) =>
      (
        snapshot.context as {
          requests?: { hostInfo?: { values: HostInfo } };
        }
      ).requests?.hostInfo?.values,
  );

  const errors = useSelector(
    (snapshot) =>
      (
        snapshot.context as {
          requests?: { hostInfo?: { errors: string | string[] } };
        }
      ).requests?.hostInfo?.errors,
  );

  useEffect(() => {
    events.requests.hostInfo.init();
  }, [events]);

  const status: Status = matches.failure
    ? "failure"
    : matches.requesting
      ? "requesting"
      : matches.success
        ? "success"
        : matches.idle
          ? "idle"
          : "pristine";

  return (
    <Card
      header={
        <>
          <HealthDot status={healthByStatus[status]} />
          <Text size="lg" weight="semibold">
            Host
          </Text>
          <Badge variant={badgeByStatus[status]}>{status}</Badge>
        </>
      }
    >
      <Stack gap="sm">
        {values?.ok && (
          <Text size="sm" color="muted">
            Version: <Text font="mono">{values.version ?? "(unknown)"}</Text>
          </Text>
        )}
        {matches.failure && errors && (
          <Text size="sm" color="muted">
            Error:{" "}
            <Text font="mono" color="default">
              {String(errors)}
            </Text>
          </Text>
        )}
        <Button
          variant="ghost"
          size="sm"
          onClick={() => events.requests.hostInfo.init()}
        >
          Re-check
        </Button>
      </Stack>
    </Card>
  );
}
