import { model } from "@oneiros/loom";

export interface HostInfo {
  ok: boolean;
  version?: string;
}

/** Dashboard model — composed via loom from its primitives. The shape of
 *  this object IS the dashboard's behavioral surface. Adding a domain is
 *  a new key here. */
export const dashboardModel = model({
  id: "dashboard",
  requests: {
    hostInfo: { init: { ok: false } as HostInfo },
  },
});

export type DashboardModel = typeof dashboardModel;
