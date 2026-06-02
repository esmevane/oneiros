import { createShell } from "@oneiros/loom-react";
import type { Bindings } from "@oneiros/loom";
import { dashboardModel } from "./machine";
import { api, client } from "@oneiros/client";

/** Weave the dashboard model into a React shell. One call per app. The
 *  result holds the Controller component, the signal-backed hooks, and
 *  the underlying signal store. */
export const shell = createShell({ model: dashboardModel });

export const { Controller, useEvents, useSelector, useMatches, useRegistries } =
  shell;

/** Production bindings — the actual IO the controller wires into the
 *  machine at runtime. Tests pass a different Bindings shape to swap
 *  every domain at once. */
export const bindings: Bindings = {
  requests: {
    hostInfo: async () => {
      const response = await fetch("/v1/health");
      if (!response.ok) {
        throw new Error(`Host responded ${response.status}`);
      }
      const body = (await response.json()) as { version?: string };
      return { ok: true, version: body.version };
    },
  },
};
