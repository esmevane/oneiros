import { assign, type AnyEventObject } from "xstate";
import type { ModelEvent, ModelNodeId, Send } from "../types";

type NavigationId = ModelNodeId<"navigation", "stack">;
type NavigationEventKind = "push" | "replace" | "reset" | "pop";

/** Assembled navigation config — one `active` state with dynamic event
 *  handlers. Exposed so `ModelSchema` composition can address `navigation.active`. */
export type NavigationConfig = {
  id: NavigationId;
  initial: "active";
  states: { active: { on: Record<string, unknown> } };
};

/** A frame on the navigation stack. */
export interface NavigationFrame<RouteName extends string> {
  route: RouteName;
  params: Record<string, unknown>;
}

/** Stack-based navigation. The current route is the top of the stack; back
 *  is `pop()`. Routes are checked at the type level — passing an unknown
 *  route fails to compile. The chart fragment has one state and uses internal
 *  transitions to mutate `context.navigation`. */
export function navigation<RouteName extends string>({
  routes = [],
  initial,
}: Partial<{ routes: readonly RouteName[]; initial: RouteName }>) {
  type Routes = (typeof routes)[number];

  type NavigationEventLabel<
    GivenEvent extends NavigationEventKind,
    GivenRoute extends string = "",
  > = GivenEvent extends "pop"
    ? ModelEvent<"navigation", "pop", "">
    : ModelEvent<"navigation", GivenEvent, GivenRoute>;

  type PushEvent = {
    [Route in Routes]: {
      type: NavigationEventLabel<"push", Route>;
      route: Route;
      params: Record<string, unknown>;
    };
  }[Routes];

  type ReplaceEvent = {
    [Route in Routes]: {
      type: NavigationEventLabel<"replace", Route>;
      route: Route;
      params: Record<string, unknown>;
    };
  }[Routes];

  type ResetEvent = {
    [Route in Routes]: {
      type: NavigationEventLabel<"reset", Route>;
      route: Route;
    };
  }[Routes];

  type PopEvent = { type: NavigationEventLabel<"pop"> };

  type NavigationEvents = PushEvent | ReplaceEvent | ResetEvent | PopEvent;

  type NavigationContext = {
    stack: NavigationFrame<Routes>[];
    current: Routes;
  };

  const labeler = {
    id: (): NavigationId => "navigation/stack",
    push: <Route extends Routes>(
      route: Route,
    ): NavigationEventLabel<"push", Route> => `@model.navigation.push.${route}`,
    replace: <Route extends Routes>(
      route: Route,
    ): NavigationEventLabel<"replace", Route> =>
      `@model.navigation.replace.${route}`,
    reset: <Route extends Routes>(
      route: Route,
    ): NavigationEventLabel<"reset", Route> =>
      `@model.navigation.reset.${route}`,
    pop: (): NavigationEventLabel<"pop"> => "@model.navigation.pop.",
  };

  type ActionArgs = {
    context: { navigation: NavigationContext };
    event: AnyEventObject;
  };

  const initialFrame: NavigationFrame<Routes> = {
    route: (initial ?? routes[0] ?? "") as Routes,
    params: {},
  };

  const context: NavigationContext = {
    stack: initial ? [initialFrame] : [],
    current: initialFrame.route,
  };

  const actions: Record<string, unknown> = {};
  const stateOn: Record<string, unknown> = {};

  for (const route of routes) {
    const push = labeler.push(route);
    const replace = labeler.replace(route);
    const reset = labeler.reset(route);

    actions[push] = assign({
      navigation: ({ context: ctx, event }: ActionArgs) => {
        const params =
          (event as { params?: Record<string, unknown> }).params ?? {};
        const frame: NavigationFrame<Routes> = { route, params };
        return {
          stack: [...ctx.navigation.stack, frame],
          current: route,
        };
      },
    });

    actions[replace] = assign({
      navigation: ({ context: ctx, event }: ActionArgs) => {
        const params =
          (event as { params?: Record<string, unknown> }).params ?? {};
        const frame: NavigationFrame<Routes> = { route, params };
        const stack = [...ctx.navigation.stack];
        if (stack.length === 0) {
          stack.push(frame);
        } else {
          stack[stack.length - 1] = frame;
        }
        return { stack, current: route };
      },
    });

    actions[reset] = assign({
      navigation: () => {
        const frame: NavigationFrame<Routes> = { route, params: {} };
        return { stack: [frame], current: route };
      },
    });

    stateOn[push] = { actions: [push] };
    stateOn[replace] = { actions: [replace] };
    stateOn[reset] = { actions: [reset] };
  }

  const popAction = labeler.pop();
  actions[popAction] = assign({
    navigation: ({ context: ctx }: ActionArgs) => {
      if (ctx.navigation.stack.length <= 1) {
        return ctx.navigation;
      }
      const stack = ctx.navigation.stack.slice(0, -1);
      const top = stack[stack.length - 1];
      return {
        stack,
        current: (top?.route ?? ctx.navigation.current) as Routes,
      };
    },
  });
  stateOn[popAction] = { actions: [popAction] };

  const config = {
    id: labeler.id(),
    initial: "active" as const,
    states: {
      active: { on: stateOn },
    },
  } satisfies NavigationConfig;

  const createEvents = (send: Send<NavigationEvents>) => ({
    push: <Route extends Routes>(
      route: Route,
      params: Record<string, unknown> = {},
    ) =>
      send({
        type: labeler.push(route),
        route,
        params,
      } as NavigationEvents),
    replace: <Route extends Routes>(
      route: Route,
      params: Record<string, unknown> = {},
    ) =>
      send({
        type: labeler.replace(route),
        route,
        params,
      } as NavigationEvents),
    reset: <Route extends Routes>(route?: Route) =>
      send({
        type: labeler.reset((route ?? initial ?? routes[0]) as Route),
        route: (route ?? initial ?? routes[0]) as Route,
      } as NavigationEvents),
    pop: () => send({ type: labeler.pop() } as NavigationEvents),
  });

  return {
    createEvents,
    config,
    context,
    actions,
    actors: {},
    events: {} as NavigationEvents,
    types: {} as { routes: Routes; frame: NavigationFrame<Routes> },
  } as const;
}
