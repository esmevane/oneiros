/** Type-level assertions for the `workers` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { workers } from "./workers";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = workers({
  workers: {
    activityStream: {},
    healthPoll: {},
  },
});

type ProbeStates = (typeof probe)["config"]["states"];

export type _WorkersConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "activityStream" | "healthPoll">
>;

type ActivityStreamSubstates = ProbeStates["activityStream"]["states"];
export type _WorkersHasThreeSubstates = Expect<
  Equal<keyof ActivityStreamSubstates, "idle" | "running" | "stopped">
>;

export type _WorkersIdIsLiteral = Expect<
  Equal<ProbeStates["activityStream"]["id"], "worker/activityStream">
>;
