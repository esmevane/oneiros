/** Type-level assertions for the `dialogs` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { dialogs } from "./dialogs";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = dialogs({ dialogs: ["settings", "about"] });

type ProbeStates = (typeof probe)["config"]["states"];

export type _DialogsConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "settings" | "about">
>;

type SettingsSubstates = ProbeStates["settings"]["states"];
export type _DialogsHasTwoSubstates = Expect<
  Equal<keyof SettingsSubstates, "open" | "closed">
>;

export type _DialogsIdIsLiteral = Expect<
  Equal<ProbeStates["settings"]["id"], "dialog/settings">
>;
