/** State-node id for a primitive's substate. Used inside chart configs to
 *  give nodes stable string ids that can be referenced from dot-paths. */
export type ModelNodeId<
  GivenRoot extends string,
  GivenString extends string,
> = `${GivenRoot}/${GivenString}`;

/** Conventional event label for a primitive's emitted event.
 *  Shape: `@model.{root}.{target}.{event}`. */
export type ModelEvent<
  GivenRoot extends string,
  GivenTarget extends string,
  GivenEvent extends string,
> = `@model.${GivenRoot}.${GivenTarget}.${GivenEvent}`;
