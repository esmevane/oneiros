import {
  assign,
  emit,
  fromPromise,
  type AnyEventObject,
  type PromiseActorLogic,
} from "xstate";
import type { ModelEvent, ModelNodeId, Send } from "../types";

type RequestId<GivenRequest extends string> = ModelNodeId<
  "request",
  GivenRequest
>;

type RequestEvent =
  | "init"
  | "error"
  | "update"
  | "clear"
  | "success"
  | "failure";

type RequestEventLabel<
  GivenEvent extends RequestEvent,
  GivenRequest extends string,
> = ModelEvent<"requests", GivenRequest, GivenEvent>;

/** Wraps a request lifecycle (pristine / idle / requesting / success / failure)
 *  in a typed chart fragment. Each request becomes a parallel substate
 *  driven by `init` events; the actor implementation is supplied at runtime
 *  via the shell's bindings. */
export function requests<
  RequestKey extends string,
  RequestMap extends Record<RequestKey, { init: unknown }>,
>({ requests = {} as RequestMap }: Partial<{ requests: RequestMap }>) {
  type AllRequests = keyof RequestMap & string;

  type RequestEventMap<GivenEvent extends RequestEvent> = {
    [GivenRequest in AllRequests]: {
      type: RequestEventLabel<GivenEvent, GivenRequest>;
    };
  };

  type RequestEvents =
    RequestEventMap<RequestEvent>[keyof RequestEventMap<RequestEvent>];

  type RequestContext = {
    [Request in AllRequests]: {
      values: RequestMap[Request]["init"];
      errors: string | string[] | RequestProblem[];
    };
  };

  const labeler: RequestLabeler<AllRequests> = {
    id: (request) => `request/${request}`,
    error: (request) => `@model.requests.${request}.error`,
    clear: (request) => `@model.requests.${request}.clear`,
    update: (request) => `@model.requests.${request}.update`,
    init: (request) => `@model.requests.${request}.init`,
    success: (request) => `@model.requests.${request}.success`,
    failure: (request) => `@model.requests.${request}.failure`,
  };

  type ActionArgs = {
    context: { requests: RequestContext };
    event: AnyEventObject;
  };

  const actors = (Object.keys(requests) as AllRequests[]).reduce(
    (chart, request) => ({
      ...chart,
      [request]: fromPromise(async () => requests[request].init),
    }),
    {} as {
      [Request in AllRequests]: PromiseActorLogic<
        unknown,
        RequestMap[Request]["init"]
      >;
    },
  );

  const actions = (Object.keys(requests) as AllRequests[]).reduce(
    (actionMap, request) => {
      const clear = labeler.clear(request);
      const error = labeler.error(request);
      const update = labeler.update(request);
      const success = labeler.success(request);
      const failure = labeler.failure(request);

      const block = {
        [success]: emit({ type: success }),
        [failure]: emit({ type: failure }),
        [error]: assign({
          requests: ({ context, event }: ActionArgs) => ({
            ...context.requests,
            [request]: {
              ...context.requests[request],
              errors: Reflect.get(event, "error"),
            },
          }),
        }),
        [clear]: assign({
          requests: ({ context }: ActionArgs) => ({
            ...context.requests,
            [request]: { ...context.requests[request], errors: [] },
          }),
        }),
        [update]: assign({
          requests: ({ context, event }: ActionArgs) => ({
            ...context.requests,
            [request]: {
              ...context.requests[request],
              values: Reflect.get(event, "output"),
            },
          }),
        }),
      } as const;

      return { ...actionMap, ...block };
    },
    {} as Record<string, unknown>,
  );

  const context = (Object.keys(requests) as AllRequests[]).reduce(
    (mapSoFar, request) => ({
      ...mapSoFar,
      [request]: { values: requests[request].init, errors: [] },
    }),
    {} as RequestContext,
  );

  const config = (Object.keys(requests) as AllRequests[]).reduce(
    (chart, request) => {
      const id = labeler.id(request);
      const clear = labeler.clear(request);
      const error = labeler.error(request);
      const update = labeler.update(request);
      const init = labeler.init(request);
      const success = labeler.success(request);
      const failure = labeler.failure(request);

      return {
        type: "parallel" as const,
        states: {
          ...chart.states,
          [request]: {
            id,
            initial: "pristine",
            states: {
              pristine: { on: { [init]: "requesting" } },
              idle: { on: { [init]: "requesting" } },
              requesting: {
                invoke: {
                  src: request,
                  input: (all: unknown) => all,
                  onError: { target: "failure", actions: [error] },
                  onDone: { target: "success", actions: [update] },
                },
              },
              success: {
                entry: [success],
                on: { [init]: { target: "requesting" } },
                after: { 100: "idle" },
              },
              failure: {
                entry: [failure],
                on: { [init]: { target: "requesting", actions: [clear] } },
              },
            },
          },
        },
      };
    },
    {
      type: "parallel" as const,
      states: {} as Record<string, unknown>,
    },
  );

  const createEvents = (send: Send<RequestEvents>) =>
    (Object.keys(requests) as AllRequests[]).reduce(
      (mapSoFar, request) => ({
        ...mapSoFar,
        [request]: {
          init: () => send({ type: labeler.init(request) }),
        },
      }),
      {} as { [Request in AllRequests]: { init: () => void } },
    );

  return {
    createEvents,
    config,
    context,
    actors,
    actions,
    events: {} as RequestEvents,
    types: {} as {
      [Request in AllRequests]: RequestMap[Request]["init"];
    },
  } as const;
}

type RequestLabeler<GivenRequest extends string> = {
  id: (request: GivenRequest) => RequestId<GivenRequest>;
} & {
  [GivenRequestEvent in RequestEvent]: (
    request: GivenRequest,
  ) => RequestEventLabel<GivenRequestEvent, GivenRequest>;
};

/** Thrown by request actors to carry HTTP-shaped failure metadata. */
export class RequestProblem<
  GivenError = unknown,
  ProblemResponse extends {
    error: GivenError;
    request: Request;
    response: Response;
  } = {
    error: GivenError;
    request: Request;
    response: Response;
  },
> {
  public status: number;
  public error: GivenError;

  public static from<
    GivenError,
    ProblemResponse extends {
      error: GivenError;
      request: Request;
      response: Response;
    },
  >(response: ProblemResponse) {
    return new RequestProblem(response);
  }

  constructor(public response: ProblemResponse) {
    this.status = response.response.status;
    this.error = response.error;
  }
}

export const requestLabeler: RequestLabeler<string> = {
  id: (request) => `request/${request}`,
  error: (request) => `@model.requests.${request}.error`,
  clear: (request) => `@model.requests.${request}.clear`,
  update: (request) => `@model.requests.${request}.update`,
  init: (request) => `@model.requests.${request}.init`,
  success: (request) => `@model.requests.${request}.success`,
  failure: (request) => `@model.requests.${request}.failure`,
};
