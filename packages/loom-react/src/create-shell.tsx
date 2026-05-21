import {
  computed,
  effect,
  signal,
  type ReadonlySignal,
  type Signal,
} from "@preact/signals-react";
import { useSignals } from "@preact/signals-react/runtime";
import { useActorRef } from "@xstate/react";
import { useEffect, useMemo, type ReactNode } from "react";
import {
  createActor,
  type Actor,
  type AnyEventObject,
  type AnyStateMachine,
  type SnapshotFrom,
} from "xstate";
import { fromConfig, type Bindings } from "@oneiros/loom";

interface ShellModel<TMachine extends AnyStateMachine = AnyStateMachine> {
  machine: TMachine;
  createEvents: (
    send: (event: AnyEventObject) => void,
  ) => Record<string, unknown>;
  registries: Record<string, unknown>;
}

interface CreateShellInput<GivenModel extends ShellModel> {
  model: GivenModel;
}

interface ShellSignals<GivenModel extends ShellModel> {
  service: Signal<Actor<GivenModel["machine"]>>;
  events: Signal<ReturnType<GivenModel["createEvents"]>>;
  snapshot: Signal<SnapshotFrom<GivenModel["machine"]>>;
  match: ReadonlySignal<(path: string) => boolean>;
  reset: () => void;
}

interface ControllerProps {
  bindings: Bindings;
  children?: ReactNode;
}

/** Factory: weave a model into a React shell. Returns the Controller
 *  component, signal-backed hooks, and the underlying signal store. One
 *  call per app. */
export function createShell<GivenModel extends ShellModel>(
  input: CreateShellInput<GivenModel>,
) {
  const { model } = input;

  type Model = GivenModel;
  type ModelActor = Actor<Model["machine"]>;
  type ModelSnapshot = SnapshotFrom<Model["machine"]>;
  type ModelEvents = ReturnType<Model["createEvents"]>;

  const buildDefault = () => {
    const service = createActor(model.machine) as ModelActor;
    return {
      service,
      events: model.createEvents(() => {}) as ModelEvents,
      snapshot: service.getSnapshot() as ModelSnapshot,
    };
  };

  const defaults = buildDefault();

  const service = signal<ModelActor>(defaults.service);
  const events = signal<ModelEvents>(defaults.events);
  const snapshot = signal<ModelSnapshot>(defaults.snapshot);

  const match = computed(
    () =>
      (path: string): boolean =>
        (
          snapshot.value as unknown as { matches: (p: string) => boolean }
        ).matches(path),
  );

  effect(() => {
    const current = service.value;
    snapshot.value = current.getSnapshot() as ModelSnapshot;
    const subscription = current.subscribe((value) => {
      snapshot.value = value as ModelSnapshot;
    });
    return () => subscription.unsubscribe();
  });

  const reset = () => {
    const next = buildDefault();
    service.value = next.service;
    events.value = next.events;
    snapshot.value = next.snapshot;
  };

  const signals: ShellSignals<Model> = {
    service,
    events,
    snapshot,
    match,
    reset,
  };

  function Controller({ bindings, children }: ControllerProps) {
    const provided = useMemo(
      () =>
        model.machine.provide({
          actors: fromConfig(bindings).actors as never,
        }),
      [bindings],
    );

    const liveService = useActorRef(provided) as ModelActor;

    const liveEvents = useMemo(
      () =>
        model.createEvents((event) =>
          liveService.send(event as Parameters<typeof liveService.send>[0]),
        ) as ModelEvents,
      [liveService],
    );

    useEffect(() => {
      signals.service.value = liveService;
      signals.events.value = liveEvents;
      return () => {
        signals.reset();
      };
    }, [liveService, liveEvents]);

    return <>{children}</>;
  }

  function useEvents(): ModelEvents {
    useSignals();
    return signals.events.value;
  }

  function useSelector<Selected>(
    selector: (snapshot: ModelSnapshot) => Selected,
  ): Selected {
    useSignals();
    return selector(signals.snapshot.value);
  }

  function useMatches(path: string): boolean {
    useSignals();
    return signals.match.value(path);
  }

  function useRegistries(): Model["registries"] {
    return model.registries;
  }

  return {
    Controller,
    useEvents,
    useSelector,
    useMatches,
    useRegistries,
    signals,
  } as const;
}
