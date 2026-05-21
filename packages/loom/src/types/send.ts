/** A typed send function — bound to the events emitted by a particular
 *  primitive or model. Consumers pass a Send into createEvents to receive
 *  a bundle of typed dispatchers. */
export type Send<GivenEvent extends { type: string }> = (
  event: GivenEvent,
) => void;
