/** Type-level assertions for the `requests` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { requests } from "./requests";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = requests({
  requests: {
    hostInfo: { init: { ok: false } },
    metrics: { init: { count: 0 } },
  },
});

type ProbeStates = (typeof probe)["config"]["states"];

/** The reduced config should narrow to per-request keys, not Record<string, unknown>. */
export type _RequestsConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "hostInfo" | "metrics">
>;

/** Each request's sub-states should be the canonical five. */
type HostInfoSubstates = ProbeStates["hostInfo"]["states"];
export type _RequestsConfigHasFiveSubstates = Expect<
  Equal<
    keyof HostInfoSubstates,
    "pristine" | "idle" | "requesting" | "success" | "failure"
  >
>;

/** The id label should be precisely typed against the request name. */
export type _RequestsConfigIdIsLiteral = Expect<
  Equal<ProbeStates["hostInfo"]["id"], "request/hostInfo">
>;
