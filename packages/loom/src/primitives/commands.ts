import { assign, type AnyEventObject } from "xstate";
import type { ModelEvent, ModelNodeId, Send } from "../types";

type CommandsId = ModelNodeId<"commands", "registry">;

/** A registered command — metadata plus an invocation function. The `run`
 *  function is closed over at registration time; the chart emits an event
 *  whenever a command is invoked so consumers can observe (and replay)
 *  command activity. */
export interface CommandDefinition {
  label: string;
  description?: string;
  keybind?: string;
  run: () => void | Promise<void>;
}

/** Commands primitive — hybrid registry + chart. Commands are looked up by
 *  id at runtime; invoking a command sends an event the chart records as
 *  `lastInvoked` in context. Useful for command palettes, keybindings, and
 *  programmatic invocation. */
export function commands<
  CommandKey extends string,
  CommandMap extends Record<CommandKey, CommandDefinition>,
>({ commands = {} as CommandMap }: Partial<{ commands: CommandMap }>) {
  type AllCommands = keyof CommandMap & string;

  type CommandEventLabel<GivenCommand extends string> = ModelEvent<
    "commands",
    "invoke",
    GivenCommand
  >;

  type CommandEvents = {
    [Command in AllCommands]: { type: CommandEventLabel<Command> };
  }[AllCommands];

  type CommandsContext = {
    lastInvoked: AllCommands | null;
  };

  const labeler = {
    id: (): CommandsId => "commands/registry",
    invoke: <Command extends AllCommands>(
      command: Command,
    ): CommandEventLabel<Command> => `@model.commands.invoke.${command}`,
  };

  type ActionArgs = {
    context: { commands: CommandsContext };
    event: AnyEventObject;
  };

  const actions: Record<string, unknown> = {};
  const stateOn: Record<string, unknown> = {};

  for (const command of Object.keys(commands) as AllCommands[]) {
    const invoke = labeler.invoke(command);
    actions[invoke] = assign({
      commands: ({ context: ctx }: ActionArgs) => ({
        ...ctx.commands,
        lastInvoked: command,
      }),
    });
    stateOn[invoke] = { actions: [invoke] };
  }

  const config = {
    id: labeler.id(),
    initial: "active" as const,
    states: { active: { on: stateOn } },
  };

  const context: CommandsContext = { lastInvoked: null };

  const createEvents = (send: Send<CommandEvents>) =>
    (Object.keys(commands) as AllCommands[]).reduce(
      (mapSoFar, command) => ({
        ...mapSoFar,
        [command]: {
          invoke: () => {
            send({ type: labeler.invoke(command) } as CommandEvents);
            const definition = commands[command];
            void definition.run();
          },
        },
      }),
      {} as { [Command in AllCommands]: { invoke: () => void } },
    );

  /** Lookup table for the registered commands. Loom-react surfaces this
   *  via useCommands() / useCommand(id). */
  const registry = commands as Readonly<CommandMap>;

  return {
    createEvents,
    config,
    context,
    actions,
    actors: {},
    registry,
    events: {} as CommandEvents,
    types: {} as Record<AllCommands, CommandMap[AllCommands]>,
  } as const;
}
