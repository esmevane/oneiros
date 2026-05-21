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
} from "./primitives";
import type { Send } from "./types";

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
 *  (commands, i18n bundles), and metadata. */
export function model<
  RequestKey extends string,
  RequestMap extends Record<RequestKey, { init: unknown }>,
  FormKey extends string,
  FormMap extends Record<
    FormKey,
    { init: unknown; validate: (model: unknown) => Promise<unknown> }
  >,
  DialogKey extends string,
  ScreenKey extends string,
  RouteKey extends string,
  WorkerKey extends string,
  WorkerMap extends Record<WorkerKey, { init?: unknown }>,
  CommandKey extends string,
  CommandMap extends Record<
    CommandKey,
    import("./primitives").CommandDefinition
  >,
  LocaleKey extends string,
  LocaleMap extends Record<LocaleKey, import("./primitives").Bundle>,
>(
  input: ModelInput<
    RequestKey,
    RequestMap,
    FormKey,
    FormMap,
    DialogKey,
    ScreenKey,
    RouteKey,
    WorkerKey,
    WorkerMap,
    CommandKey,
    CommandMap,
    LocaleKey,
    LocaleMap
  >,
) {
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
  } as const;
}

export type ModelOf<GivenInput extends ModelInput> = ReturnType<
  typeof model<
    Extract<keyof NonNullable<GivenInput["requests"]>, string>,
    NonNullable<GivenInput["requests"]>,
    Extract<keyof NonNullable<GivenInput["forms"]>, string>,
    NonNullable<GivenInput["forms"]>,
    Extract<NonNullable<GivenInput["dialogs"]>[number], string>,
    Extract<NonNullable<GivenInput["screens"]>[number], string>,
    Extract<NonNullable<GivenInput["navigation"]>["routes"][number], string>,
    Extract<keyof NonNullable<GivenInput["workers"]>, string>,
    NonNullable<GivenInput["workers"]>,
    Extract<keyof NonNullable<GivenInput["commands"]>, string>,
    NonNullable<GivenInput["commands"]>,
    Extract<keyof NonNullable<GivenInput["i18n"]>["locales"], string>,
    NonNullable<GivenInput["i18n"]>["locales"]
  >
>;
