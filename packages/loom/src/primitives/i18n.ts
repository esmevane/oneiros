import type { GetKeypaths, ModelEvent, ModelNodeId, Send } from "../types";

type I18nId<GivenLocale extends string> = ModelNodeId<"locale", GivenLocale>;

/** Translation bundle — nested objects keyed by string. Templates use
 *  lodash-style `{{name}}` interpolation. */
export type Bundle = { [key: string]: string | Bundle };

/** Look up a value at a dot-path within a bundle. Returns the path itself
 *  when missing. Templates render `{{name}}` against `params`. */
function lookupBundle(
  bundle: Bundle,
  path: string,
  params?: Record<string, unknown>,
): string {
  const segments = path.split(".");
  let current: string | Bundle = bundle;
  for (const segment of segments) {
    if (typeof current === "string") return path;
    const next: string | Bundle | undefined = current[segment];
    if (next === undefined) return path;
    current = next;
  }
  if (typeof current !== "string") return path;
  if (!params) return current;
  return current.replace(/\{\{([^}]+)\}\}/g, (_, name: string) => {
    const value = params[name.trim()];
    return value === undefined ? `{{${name}}}` : String(value);
  });
}

/** Internationalization primitive. One state per locale; switch is the
 *  only event. Surfaces a typed `translate(currentLocale, key, params)`
 *  helper. */
export function i18n<
  LocaleKey extends string,
  LocaleMap extends Record<LocaleKey, Bundle>,
>({
  locales = {} as LocaleMap,
  default: defaultLocale,
}: Partial<{ locales: LocaleMap; default: LocaleKey }>) {
  type AllLocales = keyof LocaleMap & string;

  type SwitchEvent = {
    [Locale in AllLocales]: {
      type: ModelEvent<"i18n", "switch", Locale>;
    };
  }[AllLocales];

  const initial = (defaultLocale ?? (Object.keys(locales)[0] as AllLocales)) as
    | AllLocales
    | undefined;

  const labeler = {
    id: <Locale extends AllLocales>(locale: Locale): I18nId<Locale> =>
      `locale/${locale}`,
    switch: <Locale extends AllLocales>(
      locale: Locale,
    ): ModelEvent<"i18n", "switch", Locale> => `@model.i18n.switch.${locale}`,
  };

  const config = (Object.keys(locales) as AllLocales[]).reduce(
    (chart, locale) => {
      const id = labeler.id(locale);
      const switchTo = labeler.switch(locale);
      return {
        initial: chart.initial || (initial ?? locale),
        on: { ...chart.on, [switchTo]: `#${id}` as const },
        states: { ...chart.states, [locale]: { id } },
      };
    },
    {
      initial: "" as string,
      on: {} as Record<string, unknown>,
      states: {} as Record<string, unknown>,
    },
  );

  const createEvents = (send: Send<SwitchEvent>) => ({
    switch: <Locale extends AllLocales>(locale: Locale) =>
      send({ type: labeler.switch(locale) } as SwitchEvent),
  });

  /** Translate a dot-path against the given locale's bundle. Use the typed
   *  keypaths from `types.keys` for autocompletion. */
  const translate = <Locale extends AllLocales>(
    locale: Locale,
    path: GetKeypaths<LocaleMap[Locale]>,
    params?: Record<string, unknown>,
  ): string => {
    const bundle = locales[locale];
    if (!bundle) return path as string;
    return lookupBundle(bundle, path as string, params);
  };

  return {
    createEvents,
    config,
    context: {},
    actions: {},
    actors: {},
    translate,
    locales: locales as Readonly<LocaleMap>,
    events: {} as SwitchEvent,
    types: {} as {
      locale: AllLocales;
      keys: LocaleMap extends Record<AllLocales, infer B>
        ? B extends Bundle
          ? GetKeypaths<B>
          : never
        : never;
    },
  } as const;
}
