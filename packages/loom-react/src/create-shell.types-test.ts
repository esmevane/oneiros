/** Type-level assertions for `useMatches` path-narrowing through a real
 *  shell. Purely compile-time. */

import { model } from "@oneiros/loom";
import { createShell } from "./create-shell";

const dashboardLike = model({
  id: "dashboard",
  requests: { hostInfo: { init: { ok: false } } },
});

const shell = createShell({ model: dashboardLike });

/** Valid path — should compile. */
shell.useMatches("requests.hostInfo.idle");
shell.useMatches("requests.hostInfo.failure");
shell.useMatches("requests");
shell.useMatches("requests.hostInfo");

/** Invalid path — should be a compile error. The `@ts-expect-error` is the
 *  assertion: if this becomes valid, typecheck fails. */
// @ts-expect-error — "active" is not a substate of hostInfo
shell.useMatches("requests.hostInfo.active");

// @ts-expect-error — typo in primitive name
shell.useMatches("forms.hostInfo.idle");

// @ts-expect-error — unconfigured primitive
shell.useMatches("dialogs.confirm");

// @ts-expect-error — totally invalid
shell.useMatches("nonsense");
