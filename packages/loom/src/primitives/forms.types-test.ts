/** Type-level assertions for the `forms` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { forms } from "./forms";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = forms({
  forms: {
    signIn: {
      init: { email: "" },
      validate: async () => undefined,
    },
    profile: {
      init: { name: "" },
      validate: async () => undefined,
    },
  },
});

type ProbeStates = (typeof probe)["config"]["states"];

/** Each configured form should be addressable by name. */
export type _FormsConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "signIn" | "profile">
>;

/** Each form should expose the two parallel regions. */
type SignInRegions = ProbeStates["signIn"]["states"];
export type _FormsConfigHasTwoRegions = Expect<
  Equal<keyof SignInRegions, "activity" | "validation">
>;

/** Activity substates: idle, active. */
type SignInActivity = SignInRegions["activity"]["states"];
export type _FormsActivityHasTwoStates = Expect<
  Equal<keyof SignInActivity, "idle" | "active">
>;

/** Validation substates: pristine, valid, invalid, validating. */
type SignInValidation = SignInRegions["validation"]["states"];
export type _FormsValidationHasFourStates = Expect<
  Equal<keyof SignInValidation, "pristine" | "valid" | "invalid" | "validating">
>;

/** The id label should be precisely typed against the form name. */
export type _FormsConfigIdIsLiteral = Expect<
  Equal<ProbeStates["signIn"]["id"], "form/signIn">
>;
