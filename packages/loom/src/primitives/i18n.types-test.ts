/** Type-level assertions for the `i18n` primitive's emitted schema.
 *  Purely compile-time — running typecheck IS the test. */

import { i18n } from "./i18n";

type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type Expect<T extends true> = T;

const probe = i18n({
  locales: {
    en: { hello: "Hello" },
    fr: { hello: "Bonjour" },
  },
});

type ProbeStates = (typeof probe)["config"]["states"];

export type _I18nConfigHasNamedKeys = Expect<
  Equal<keyof ProbeStates, "en" | "fr">
>;

export type _I18nIdIsLiteral = Expect<
  Equal<ProbeStates["en"]["id"], "locale/en">
>;
