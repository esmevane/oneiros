import { fromPromise, type PromiseActorLogic } from "xstate";
import { formLabeler } from "./primitives/forms";

/** Runtime bindings — the actual IO implementations the shell provides. The
 *  shape mirrors the primitive envelopes: each domain gets its own bag of
 *  async functions. */
export interface Bindings {
  requests?: Record<string, (input?: unknown) => Promise<unknown>>;
  forms?: Record<string, (model: unknown) => Promise<unknown>>;
  workers?: Record<string, (input?: unknown) => Promise<unknown>>;
}

/** Turn a Bindings object into the xstate actor map the model's machine
 *  expects via `.provide({ actors: ... })`. */
export function fromConfig(bindings: Bindings) {
  const actors: Record<string, PromiseActorLogic<unknown, unknown>> = {};

  if (bindings.requests) {
    for (const key of Object.keys(bindings.requests)) {
      const fn = bindings.requests[key]!;
      actors[key] = fromPromise(async ({ input }) => fn(input));
    }
  }

  if (bindings.forms) {
    for (const key of Object.keys(bindings.forms)) {
      const fn = bindings.forms[key]!;
      const label = formLabeler.validate(key);
      actors[label] = fromPromise(async ({ input }) => fn(input));
    }
  }

  if (bindings.workers) {
    for (const key of Object.keys(bindings.workers)) {
      const fn = bindings.workers[key]!;
      actors[key] = fromPromise(async ({ input }) => fn(input));
    }
  }

  return { actors };
}
