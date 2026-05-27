import {
  fromPromise,
  type AnyEventObject,
  type PromiseActorLogic,
} from "xstate";
import type { ModelEvent, ModelNodeId, Send } from "../types";

type WorkerId<GivenWorker extends string> = ModelNodeId<"worker", GivenWorker>;
type WorkerEventKind = "start" | "stop" | "restart";

/** Assembled workers config — preserves per-worker keying so downstream
 *  path derivation (via `ModelPaths`) can see each worker as a known node. */
export type WorkersConfig<
  WorkerMap extends Record<string, { init?: unknown }>,
> = {
  type: "parallel";
  states: {
    [GivenWorker in keyof WorkerMap & string]: {
      id: WorkerId<GivenWorker>;
      initial: "idle";
      states: { idle: object; running: object; stopped: object };
    };
  };
};

/** Long-running background work. Each worker has an idle/running/stopped
 *  lifecycle and an actor placeholder that the shell's bindings supply at
 *  runtime. Workers are useful for SSE subscriptions, polling loops, and
 *  anything async-perpetual. */
export function workers<
  WorkerKey extends string,
  WorkerMap extends Record<WorkerKey, { init?: unknown }>,
>({ workers = {} as WorkerMap }: Partial<{ workers: WorkerMap }>) {
  type AllWorkers = keyof WorkerMap & string;

  type WorkerEventLabel<
    GivenEvent extends WorkerEventKind,
    GivenWorker extends string,
  > = ModelEvent<"workers", GivenEvent, GivenWorker>;

  type WorkerEventMap<GivenEvent extends WorkerEventKind> = {
    [GivenWorker in AllWorkers]: {
      type: WorkerEventLabel<GivenEvent, GivenWorker>;
    };
  };

  type WorkerEvents =
    WorkerEventMap<WorkerEventKind>[keyof WorkerEventMap<WorkerEventKind>];

  const labeler = {
    id: (worker: AllWorkers): WorkerId<typeof worker> => `worker/${worker}`,
    start: <Worker extends AllWorkers>(
      worker: Worker,
    ): WorkerEventLabel<"start", Worker> => `@model.workers.start.${worker}`,
    stop: <Worker extends AllWorkers>(
      worker: Worker,
    ): WorkerEventLabel<"stop", Worker> => `@model.workers.stop.${worker}`,
    restart: <Worker extends AllWorkers>(
      worker: Worker,
    ): WorkerEventLabel<"restart", Worker> =>
      `@model.workers.restart.${worker}`,
  };

  const actors = (Object.keys(workers) as AllWorkers[]).reduce(
    (mapSoFar, worker) => ({
      ...mapSoFar,
      [worker]: fromPromise<unknown, unknown>(async () => undefined),
    }),
    {} as Record<string, PromiseActorLogic<unknown, unknown, AnyEventObject>>,
  );

  const config = (Object.keys(workers) as AllWorkers[]).reduce(
    (chart, worker) => {
      const id = labeler.id(worker);
      const start = labeler.start(worker);
      const stop = labeler.stop(worker);
      const restart = labeler.restart(worker);

      return {
        type: "parallel" as const,
        states: {
          ...chart.states,
          [worker]: {
            id,
            initial: "idle" as const,
            states: {
              idle: { on: { [start]: "running" } },
              running: {
                invoke: {
                  src: worker,
                  input: (all: unknown) => all,
                  onDone: { target: "idle" },
                  onError: { target: "stopped" },
                },
                on: {
                  [stop]: "stopped",
                  [restart]: "running",
                },
              },
              stopped: {
                on: {
                  [start]: "running",
                  [restart]: "running",
                },
              },
            },
          },
        },
      };
    },
    {
      type: "parallel" as const,
      states: {} as WorkersConfig<WorkerMap>["states"],
    },
  ) satisfies WorkersConfig<WorkerMap>;

  const createEvents = (send: Send<WorkerEvents>) =>
    (Object.keys(workers) as AllWorkers[]).reduce(
      (mapSoFar, worker) => ({
        ...mapSoFar,
        [worker]: {
          start: () => send({ type: labeler.start(worker) }),
          stop: () => send({ type: labeler.stop(worker) }),
          restart: () => send({ type: labeler.restart(worker) }),
        },
      }),
      {} as {
        [Worker in AllWorkers]: {
          start: () => void;
          stop: () => void;
          restart: () => void;
        };
      },
    );

  return {
    createEvents,
    config,
    context: {},
    actions: {},
    actors,
    events: {} as WorkerEvents,
    types: {} as Record<AllWorkers, WorkerMap[AllWorkers]>,
  } as const;
}
