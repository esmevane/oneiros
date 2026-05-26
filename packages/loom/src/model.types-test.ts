/** Type-level assertions for the composed `ModelSchema` exposed via
 *  `model()`'s phantom `schema` field. Purely compile-time. */

import { model } from "./model";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

/** Mirrors the dashboard model — requests only. */
const dashboardLike = model({
  id: "dashboard",
  requests: {
    hostInfo: { init: { ok: false } },
  },
});

type DashboardSchema = (typeof dashboardLike)["schema"];

/** Only requests should appear; no forms/dialogs/screens/etc. */
export type _DashboardSchemaKeysAreOnlyRequests = Expect<
  Equal<keyof DashboardSchema, "requests">
>;

/** The requests path narrows to the configured request. */
export type _DashboardRequestsKeyNarrows = Expect<
  Equal<keyof DashboardSchema["requests"]["states"], "hostInfo">
>;

/** A model with multiple primitives should compose all of them. */
const wide = model({
  id: "wide",
  requests: { fetchUser: { init: null } },
  dialogs: ["confirm"],
  screens: ["home", "settings"],
  workers: { poll: {} },
});

type WideSchema = (typeof wide)["schema"];

export type _WideSchemaHasFourKeys = Expect<
  Equal<keyof WideSchema, "requests" | "dialogs" | "screens" | "workers">
>;

export type _WideDialogsNarrows = Expect<
  Equal<keyof WideSchema["dialogs"]["states"], "confirm">
>;

export type _WideScreensNarrows = Expect<
  Equal<keyof WideSchema["screens"]["states"], "home" | "settings">
>;

/** An empty model should produce an empty schema (no keys). */
const empty = model({ id: "empty" });
type EmptySchema = (typeof empty)["schema"];

export type _EmptySchemaHasNoKeys = Expect<Equal<keyof EmptySchema, never>>;
