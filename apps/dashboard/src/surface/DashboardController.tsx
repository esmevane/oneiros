import { Controller, bindings } from "../shell";

/** The controller island — the page's brain. Runs the dashboard model with
 *  the production bindings, publishes service + events to global signals.
 *  Exactly one per page. */
export function DashboardController() {
  return <Controller bindings={bindings} />;
}
