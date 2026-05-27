/** Type-level assertions for `ModelPaths` over a composed `ModelSchema`.
 *  Purely compile-time. */

import { model } from "../model";
import type { ModelPaths } from "./paths";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const dashboardLike = model({
  id: "dashboard",
  requests: { hostInfo: { init: { ok: false } } },
});

type DashboardPaths = ModelPaths<(typeof dashboardLike)["schema"]>;

/** Reserved keys (`type`, `id`, `initial`) must not appear in paths.
 *  Substates must be reachable as dot-paths. */
export type _DashboardPathsAreExact = Expect<
  Equal<
    DashboardPaths,
    | "requests"
    | "requests.hostInfo"
    | "requests.hostInfo.pristine"
    | "requests.hostInfo.idle"
    | "requests.hostInfo.requesting"
    | "requests.hostInfo.success"
    | "requests.hostInfo.failure"
  >
>;

/** Composed schema with multiple primitives produces a wider path union. */
const wide = model({
  id: "wide",
  requests: { fetchUser: { init: null } },
  dialogs: ["confirm"],
});

type WidePaths = ModelPaths<(typeof wide)["schema"]>;

/** Each top-level primitive contributes a path subtree. */
export type _WidePathsIncludeBothPrimitives = Expect<
  Equal<
    WidePaths,
    | "requests"
    | "requests.fetchUser"
    | "requests.fetchUser.pristine"
    | "requests.fetchUser.idle"
    | "requests.fetchUser.requesting"
    | "requests.fetchUser.success"
    | "requests.fetchUser.failure"
    | "dialogs"
    | "dialogs.confirm"
    | "dialogs.confirm.open"
    | "dialogs.confirm.closed"
  >
>;
