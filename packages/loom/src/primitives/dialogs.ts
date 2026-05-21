import type { ModelEvent, ModelNodeId, Send } from "../types";

type DialogId<DialogKey extends string> = ModelNodeId<"dialog", DialogKey>;
type DialogEventKind = "close" | "open" | "toggle";

/** Dialog primitive — each dialog is an independent open/closed substate
 *  inside a parallel root. Pure UI state; no actors. */
export function dialogs<DialogKey extends string>({
  dialogs = [],
}: Partial<{ dialogs: DialogKey[] }>) {
  type Dialogs = (typeof dialogs)[number];

  type DialogEventLabel<
    GivenEvent extends DialogEventKind,
    GivenDialog extends Dialogs,
  > = ModelEvent<"dialogs", GivenEvent, GivenDialog>;

  type DialogEventMap<GivenEvent extends DialogEventKind> = {
    [GivenDialog in Dialogs]: {
      type: DialogEventLabel<GivenEvent, GivenDialog>;
    };
  };

  type DialogEvents =
    DialogEventMap<DialogEventKind>[keyof DialogEventMap<DialogEventKind>];

  const labeler = {
    id: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): DialogId<GivenDialog> => `dialog/${dialog}`,
    close: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): DialogEventLabel<"close", GivenDialog> =>
      `@model.dialogs.close.${dialog}`,
    open: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): DialogEventLabel<"open", GivenDialog> => `@model.dialogs.open.${dialog}`,
    toggle: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): DialogEventLabel<"toggle", GivenDialog> =>
      `@model.dialogs.toggle.${dialog}`,
    opened: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): `#${DialogId<GivenDialog>}.open` => `#${labeler.id(dialog)}.open`,
    closed: <GivenDialog extends Dialogs>(
      dialog: GivenDialog,
    ): `#${DialogId<GivenDialog>}.closed` => `#${labeler.id(dialog)}.closed`,
  };

  const config = dialogs.reduce(
    (chart, dialog) => {
      const id = labeler.id(dialog);
      const toggle = labeler.toggle(dialog);
      const close = labeler.close(dialog);
      const open = labeler.open(dialog);
      const opened = labeler.opened(dialog);
      const closed = labeler.closed(dialog);

      return {
        type: "parallel" as const,
        states: {
          ...chart.states,
          [dialog]: {
            id,
            initial: "closed",
            states: {
              open: { on: { [close]: closed, [toggle]: closed } },
              closed: { on: { [open]: opened, [toggle]: opened } },
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

  const createEvents = (send: Send<DialogEvents>) =>
    dialogs.reduce(
      (mapSoFar, dialog) => ({
        ...mapSoFar,
        [dialog]: {
          close: () => send({ type: labeler.close(dialog) }),
          open: () => send({ type: labeler.open(dialog) }),
          toggle: () => send({ type: labeler.toggle(dialog) }),
        },
      }),
      {} as {
        [Dialog in Dialogs]: {
          open: () => void;
          close: () => void;
          toggle: () => void;
        };
      },
    );

  return {
    createEvents,
    config,
    context: {},
    actions: {},
    actors: {},
    events: {} as DialogEvents,
    types: {} as Record<Dialogs, never>,
  } as const;
}
