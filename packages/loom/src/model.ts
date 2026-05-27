import { setup, type AnyEventObject, type AnyStateMachine } from "xstate";
import {
  commands as commandsPrimitive,
  dialogs as dialogsPrimitive,
  forms as formsPrimitive,
  i18n as i18nPrimitive,
  navigation as navigationPrimitive,
  requests as requestsPrimitive,
  screens as screensPrimitive,
  workers as workersPrimitive,
  type Bundle,
  type CommandDefinition,
  type CommandsConfig,
  type DialogsConfig,
  type FormsConfig,
  type I18nConfig,
  type NavigationConfig,
  type RequestsConfig,
  type ScreensConfig,
  type WorkersConfig,
} from "./primitives";
import type { Send } from "./types";

/** Composed schema of a model — mirrors the runtime `states` envelope so
 *  consumers can derive `ModelPaths<ModelSchema<Input>>` for typestate-aware
 *  matchers. Each primitive contributes its key only when the input declares
 *  the corresponding field. */
export type ModelSchema<Input> = (Input extends {
  requests: infer R extends Record<string, { init: unknown }>;
}
  ? { requests: RequestsConfig<R> }
  : object) &
  (Input extends {
    forms: infer F extends Record<
      string,
      { init: unknown; validate: (model: unknown) => Promise<unknown> }
    >;
  }
    ? { forms: FormsConfig<F> }
    : object) &
  (Input extends { dialogs: infer D extends readonly string[] }
    ? { dialogs: DialogsConfig<D[number]> }
    : object) &
  (Input extends { screens: infer S extends readonly string[] }
    ? { screens: ScreensConfig<S[number]> }
    : object) &
  (Input extends { navigation: { routes: readonly string[] } }
    ? { navigation: NavigationConfig }
    : object) &
  (Input extends {
    workers: infer W extends Record<string, { init?: unknown }>;
  }
    ? { workers: WorkersConfig<W> }
    : object) &
  (Input extends { commands: Record<string, CommandDefinition> }
    ? { commands: CommandsConfig }
    : object) &
  (Input extends {
    i18n: { locales: infer L extends Record<string, Bundle> };
  }
    ? { i18n: I18nConfig<L> }
    : object);

/** ModelInput threads the consumer's configuration for each primitive. All
 *  fields optional — omitted primitives contribute no states. */
export interface ModelInput<
  RequestKey extends string = string,
  RequestMap extends Record<RequestKey, { init: unknown }> = Record<
    RequestKey,
    { init: unknown }
  >,
  FormKey extends string = string,
  FormMap extends Record<
    FormKey,
    { init: unknown; validate: (model: unknown) => Promise<unknown> }
  > = Record<
    FormKey,
    { init: unknown; validate: (model: unknown) => Promise<unknown> }
  >,
  DialogKey extends string = string,
  ScreenKey extends string = string,
  RouteKey extends string = string,
  WorkerKey extends string = string,
  WorkerMap extends Record<WorkerKey, { init?: unknown }> = Record<
    WorkerKey,
    { init?: unknown }
  >,
  CommandKey extends string = string,
  CommandMap extends Record<
    CommandKey,
    import("./primitives").CommandDefinition
  > = Record<CommandKey, import("./primitives").CommandDefinition>,
  LocaleKey extends string = string,
  LocaleMap extends Record<LocaleKey, import("./primitives").Bundle> = Record<
    LocaleKey,
    import("./primitives").Bundle
  >,
> {
  id?: string;
  requests?: RequestMap;
  forms?: FormMap;
  dialogs?: DialogKey[];
  screens?: ScreenKey[];
  navigation?: { routes: readonly RouteKey[]; initial?: RouteKey };
  workers?: WorkerMap;
  commands?: CommandMap;
  i18n?: { locales: LocaleMap; default?: LocaleKey };
}

/** Compose the eight primitives into a single typesafe model. Returns the
 *  assembled xstate machine plus the typed event factory, runtime registries
 *  (commands, i18n bundles), and metadata.
 *
 *  Uses `const Input` so the input's literal shape — including which optional
 *  fields are present — flows through to `ModelSchema<Input>` for downstream
 *  path narrowing. */
export function model<const Input extends ModelInput>(input: Input) {
  const requestsFragment = requestsPrimitive({ requests: input.requests });
  const formsFragment = formsPrimitive({ forms: input.forms });
  const dialogsFragment = dialogsPrimitive({ dialogs: input.dialogs });
  const screensFragment = screensPrimitive({ screens: input.screens });
  const navigationFragment = navigationPrimitive({
    routes: input.navigation?.routes,
    initial: input.navigation?.initial,
  });
  const workersFragment = workersPrimitive({ workers: input.workers });
  const commandsFragment = commandsPrimitive({ commands: input.commands });
  const i18nFragment = i18nPrimitive({
    locales: input.i18n?.locales,
    default: input.i18n?.default,
  });

  // Collect the chart fragments under their domain envelope. Only include
  // primitives the consumer configured — empty primitives contribute nothing.
  const states: Record<string, unknown> = {};
  if (input.requests) states.requests = requestsFragment.config;
  if (input.forms) states.forms = formsFragment.config;
  if (input.dialogs) states.dialogs = dialogsFragment.config;
  if (input.screens) states.screens = screensFragment.config;
  if (input.navigation) states.navigation = navigationFragment.config;
  if (input.workers) states.workers = workersFragment.config;
  if (input.commands) states.commands = commandsFragment.config;
  if (input.i18n) states.i18n = i18nFragment.config;

  const context: Record<string, unknown> = {};
  if (input.requests) context.requests = requestsFragment.context;
  if (input.forms) context.forms = formsFragment.context;
  if (input.navigation) context.navigation = navigationFragment.context;
  if (input.commands) context.commands = commandsFragment.context;

  const actors = {
    ...requestsFragment.actors,
    ...formsFragment.actors,
    ...workersFragment.actors,
  };

  const actions = {
    ...requestsFragment.actions,
    ...formsFragment.actions,
    ...navigationFragment.actions,
    ...commandsFragment.actions,
  };

  // setup() with permissive types — primitive-specific types are exposed
  // through the typed createEvents / snapshot accessors below. The result
  // is widened to AnyStateMachine so downstream consumers (createShell)
  // can accept it without StateSchema-mismatch noise.
  const machine = setup({
    types: {
      context: {} as typeof context,
      events: {} as AnyEventObject,
    },
    actors: actors as unknown as Record<string, never>,
    actions: actions as unknown as Record<string, never>,
  }).createMachine({
    id: input.id ?? "model",
    type: "parallel",
    context,
    states: states as unknown as Record<string, never>,
  }) as unknown as AnyStateMachine;

  const createEvents = (send: Send<AnyEventObject>) => ({
    ...(input.requests
      ? { requests: requestsFragment.createEvents(send) }
      : {}),
    ...(input.forms ? { forms: formsFragment.createEvents(send) } : {}),
    ...(input.dialogs ? { dialogs: dialogsFragment.createEvents(send) } : {}),
    ...(input.screens ? { screens: screensFragment.createEvents(send) } : {}),
    ...(input.navigation
      ? { navigation: navigationFragment.createEvents(send) }
      : {}),
    ...(input.workers ? { workers: workersFragment.createEvents(send) } : {}),
    ...(input.commands
      ? { commands: commandsFragment.createEvents(send) }
      : {}),
    ...(input.i18n ? { i18n: i18nFragment.createEvents(send) } : {}),
  });

  return {
    machine,
    createEvents,
    fragments: {
      requests: requestsFragment,
      forms: formsFragment,
      dialogs: dialogsFragment,
      screens: screensFragment,
      navigation: navigationFragment,
      workers: workersFragment,
      commands: commandsFragment,
      i18n: i18nFragment,
    },
    /** Runtime registries — surfaces consume these alongside the chart. */
    registries: {
      commands: commandsFragment.registry,
      i18n: {
        locales: i18nFragment.locales,
        translate: i18nFragment.translate,
      },
    },
    /** Phantom — type-level mirror of the assembled `states` envelope.
     *  Consumed by `createShell` to type `useMatches` against a precise
     *  path union. No runtime value. */
    schema: {} as ModelSchema<Input>,
  } as const;
}

export type ModelOf<GivenInput extends ModelInput> = ReturnType<
  typeof model<GivenInput>
>;
