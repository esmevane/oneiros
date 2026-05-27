/** xstate's reserved state-config keys. Excluded when deriving keypaths so
 *  that traversal only walks user-defined state nodes. */
type XStateReservedKeys =
  | "after"
  | "context"
  | "entry"
  | "always"
  | "exit"
  | "id"
  | "initial"
  | "invoke"
  | "on"
  | "onDone"
  | "onExit"
  | "type";

/** Strip xstate's reserved keys so a schema can be walked as a tree of
 *  states only. */
export type ProduceSchema<GivenType> = {
  [Key in keyof GivenType as Key extends XStateReservedKeys
    ? never
    : Key]: Key extends XStateReservedKeys
    ? never
    : ProduceSchema<GivenType[Key]>;
};

/** Build a union of dot-path strings into a (typed) tree. The `Filter`
 *  parameter lets you ask the walker to descend through one specific key
 *  per level (e.g. "states") without surfacing that key in the resulting
 *  paths. */
export type GetKeypaths<
  GivenType,
  Filter extends string = "__!!PLACEHOLDER!!__",
> = {
  [Key in keyof GivenType & string]: GivenType[Key] extends object
    ? Key extends Filter
      ? GetKeypaths<GivenType[Key], Filter>
      : Key | `${Key}.${GetKeypaths<GivenType[Key], Filter>}`
    : Key;
}[keyof GivenType & string];

/** All dot-paths reachable in a chart schema. Strips xstate's reserved keys
 *  first, then walks the user-defined state tree, descending silently
 *  through "states" so paths read like `requests.hostInfo.idle` rather than
 *  `requests.states.hostInfo.states.idle`. */
export type ModelPaths<GivenType> = GetKeypaths<
  ProduceSchema<GivenType>,
  "states"
>;
