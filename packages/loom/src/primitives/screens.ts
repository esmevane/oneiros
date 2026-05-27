import type { ModelEvent, ModelNodeId, Send } from "../types";

type ScreenId<GivenScreen extends string> = ModelNodeId<"screen", GivenScreen>;
type OpenScreen<GivenScreen extends string> = ModelEvent<
  "screens",
  "open",
  GivenScreen
>;

/** Assembled screens config — preserves per-screen keying so downstream
 *  path derivation (via `ModelPaths`) can see each screen as a known node. */
export type ScreensConfig<ScreenKey extends string> = {
  initial: string;
  on: Record<string, unknown>;
  states: { [GivenScreen in ScreenKey]: { id: ScreenId<GivenScreen> } };
};

/** Screens primitive — mode switcher. Exactly one screen is active at a
 *  time. No history (use the `navigation` primitive for a stack). */
export function screens<ScreenKey extends string>({
  screens = [],
}: Partial<{ screens: ScreenKey[] }>) {
  type AllScreens = (typeof screens)[number];

  type ScreenEventMap = {
    [Key in AllScreens]: { type: OpenScreen<Key> };
  };

  type ScreenEvents = ScreenEventMap[keyof ScreenEventMap];

  const labeler = {
    open: <GivenScreen extends AllScreens>(
      screen: GivenScreen,
    ): OpenScreen<GivenScreen> => `@model.screens.open.${screen}`,
    id: <GivenScreen extends AllScreens>(
      screen: GivenScreen,
    ): ScreenId<GivenScreen> => `screen/${screen}`,
  };

  const createEvents = (send: Send<ScreenEvents>) =>
    screens.reduce(
      (mapSoFar, screen) => ({
        ...mapSoFar,
        [screen]: {
          open: () => send({ type: labeler.open(screen) }),
        },
      }),
      {} as { [Key in AllScreens]: { open: () => void } },
    );

  const config = screens.reduce(
    (chart, screen) => {
      const id = labeler.id(screen);
      const open = labeler.open(screen);
      return {
        initial: chart.initial || screen,
        on: { ...chart.on, [open]: `#${id}` as const },
        states: { ...chart.states, [screen]: { id } },
      };
    },
    {
      initial: "" as string,
      on: {} as Record<string, unknown>,
      states: {} as ScreensConfig<ScreenKey>["states"],
    },
  ) satisfies ScreensConfig<ScreenKey>;

  return {
    createEvents,
    config,
    context: {},
    actions: {},
    actors: {},
    events: {} as ScreenEvents,
    types: {} as Record<AllScreens, never>,
  } as const;
}
