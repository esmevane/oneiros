import {
  assign,
  fromPromise,
  type AnyEventObject,
  type PromiseActorLogic,
} from "xstate";
import type { ModelEvent, ModelNodeId, Send } from "../types";

type FormId<GivenForm extends string> = ModelNodeId<"form", GivenForm>;

type FormEvent = "error" | "clear" | "commit" | "update" | "validate";

type FormEventLabel<
  GivenEvent extends FormEvent,
  GivenForm extends string,
> = ModelEvent<"forms", GivenEvent, GivenForm>;

/** Forms primitive — each form is a parallel substate tracking activity
 *  (active/idle) and validation (pristine/validating/valid/invalid). The
 *  validate actor is the consumer's async predicate. */
export function forms<
  FormKey extends string,
  FormMap extends Record<
    FormKey,
    { init: unknown; validate: (model: unknown) => Promise<unknown> }
  >,
>({ forms = {} as FormMap }: Partial<{ forms: FormMap }>) {
  type AllForms = keyof FormMap & string;

  type ChangeFormField<
    GivenEvent extends FormEvent,
    GivenForm extends AllForms,
    GivenValues extends FormMap[GivenForm]["init"] = FormMap[GivenForm]["init"],
    GivenField extends keyof GivenValues = keyof GivenValues,
  > = {
    type: FormEventLabel<GivenEvent, GivenForm>;
    field: GivenField;
    value: GivenValues[GivenField];
  };

  type FormEventMap<GivenEvent extends FormEvent> = {
    [GivenForm in AllForms]: GivenEvent extends "commit" | "update"
      ? ChangeFormField<GivenEvent, GivenForm>
      : { type: FormEventLabel<GivenEvent, GivenForm> };
  };

  type FormEvents = FormEventMap<FormEvent>[keyof FormEventMap<FormEvent>];

  type FormContext = {
    [Form in AllForms]: {
      values: FormMap[Form]["init"];
      errors: string | string[];
    };
  };

  const labeler: FormLabeler<AllForms> = {
    id: (form) => `form/${form}`,
    error: (form) => `@model.forms.error.${form}`,
    clear: (form) => `@model.forms.clear.${form}`,
    validate: (form) => `@model.forms.validate.${form}`,
    update: (form) => `@model.forms.update.${form}`,
    commit: (form) => `@model.forms.commit.${form}`,
  };

  type ActionArgs = {
    context: { forms: FormContext };
    event: AnyEventObject;
  };

  const actions = (Object.keys(forms) as AllForms[]).reduce(
    (mapSoFar, givenForm) => {
      const clear = labeler.clear(givenForm);
      const error = labeler.error(givenForm);
      const update = labeler.update(givenForm);

      const block = {
        [error]: assign({
          forms: ({ context, event }: ActionArgs) => ({
            ...context.forms,
            [givenForm]: {
              ...context.forms[givenForm],
              errors: Reflect.get(event, "error") as string | string[],
            },
          }),
        }),
        [clear]: assign({
          forms: ({ context }: ActionArgs) => ({
            ...context.forms,
            [givenForm]: { ...context.forms[givenForm], errors: [] },
          }),
        }),
        [update]: assign({
          forms: ({ context, event }: ActionArgs) => {
            const field = Reflect.get(event, "field");
            const value = Reflect.get(event, "value");
            return {
              ...context.forms,
              [givenForm]: {
                ...context.forms[givenForm],
                values: {
                  ...(context.forms[givenForm].values as object),
                  [field]: value,
                },
              },
            };
          },
        }),
      } as const;

      return { ...mapSoFar, ...block };
    },
    {} as Record<string, unknown>,
  );

  const context = (Object.keys(forms) as AllForms[]).reduce(
    (mapSoFar, givenForm) => ({
      ...mapSoFar,
      [givenForm]: { values: forms[givenForm].init, errors: [] },
    }),
    {} as FormContext,
  );

  const actors = (Object.keys(forms) as AllForms[]).reduce(
    (mapSoFar, givenForm) => {
      const validate = labeler.validate(givenForm);
      return {
        ...mapSoFar,
        [validate]: fromPromise<unknown, unknown>(({ input }) =>
          forms[givenForm].validate(input),
        ),
      };
    },
    {} as Record<string, PromiseActorLogic<unknown, unknown, AnyEventObject>>,
  );

  const config = (Object.keys(forms) as AllForms[]).reduce(
    (mapSoFar, givenForm) => {
      const id = labeler.id(givenForm);
      const clear = labeler.clear(givenForm);
      const error = labeler.error(givenForm);
      const update = labeler.update(givenForm);
      const commit = labeler.commit(givenForm);
      const validate = labeler.validate(givenForm);

      return {
        type: "parallel" as const,
        states: {
          ...mapSoFar.states,
          [givenForm]: {
            id,
            type: "parallel" as const,
            states: {
              activity: {
                initial: "idle" as const,
                states: {
                  active: {
                    after: { 16: `#${id}.validation.validating` },
                    on: { [commit]: { target: "active", actions: [update] } },
                  },
                  idle: {
                    on: {
                      [commit]: { target: "active", actions: [update] },
                    },
                  },
                },
              },
              validation: {
                id: "validation" as const,
                initial: "pristine" as const,
                states: {
                  pristine: {},
                  valid: {},
                  invalid: {},
                  validating: {
                    invoke: {
                      src: validate,
                      input: ({ context }: ActionArgs) =>
                        context.forms[givenForm].values,
                      onDone: { target: "valid", actions: clear },
                      onError: { target: "invalid", actions: error },
                    },
                  },
                },
              },
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

  const createEvents = (send: Send<FormEvents>) =>
    (Object.keys(forms) as AllForms[]).reduce(
      (mapSoFar, givenForm) => ({
        ...mapSoFar,
        [givenForm]: {
          clear: () => send({ type: labeler.clear(givenForm) } as FormEvents),
          commit: ({
            field,
            value,
          }: Pick<
            ChangeFormField<"commit", typeof givenForm>,
            "field" | "value"
          >) =>
            send({
              type: labeler.commit(givenForm),
              field,
              value,
            } as FormEvents),
        },
      }),
      {} as {
        [Form in AllForms]: {
          clear: () => void;
          commit: (
            payload: Pick<ChangeFormField<"commit", Form>, "field" | "value">,
          ) => void;
        };
      },
    );

  return {
    createEvents,
    config,
    context,
    actions,
    actors,
    events: {} as FormEvents,
    types: {} as Record<AllForms, FormMap[AllForms]>,
  } as const;
}

type FormLabeler<GivenForm extends string> = {
  id: (form: GivenForm) => FormId<GivenForm>;
} & {
  [GivenFormEvent in FormEvent]: (
    form: GivenForm,
  ) => FormEventLabel<GivenFormEvent, GivenForm>;
};

export const formLabeler: FormLabeler<string> = {
  id: (form) => `form/${form}`,
  error: (form) => `@model.forms.error.${form}`,
  clear: (form) => `@model.forms.clear.${form}`,
  validate: (form) => `@model.forms.validate.${form}`,
  update: (form) => `@model.forms.update.${form}`,
  commit: (form) => `@model.forms.commit.${form}`,
};
