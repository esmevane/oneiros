/** Type-level assertions for the `screens` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { screens } from "./screens";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = screens({ screens: ["home", "settings"] });

type ProbeStates = (typeof probe)["config"]["states"];

export type _ScreensConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "home" | "settings">
>;

export type _ScreensIdIsLiteral = Expect<
  Equal<ProbeStates["home"]["id"], "screen/home">
>;
