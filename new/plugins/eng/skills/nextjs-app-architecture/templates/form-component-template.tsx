"use client";

import { useActionState } from "react";

import {
  saveExampleAction,
  type ExampleActionState,
} from "@/features/example/actions/save-example-action";

const initialState: ExampleActionState = { ok: false };

export function ExampleForm() {
  const [state, formAction, isPending] = useActionState(
    saveExampleAction,
    initialState,
  );

  return (
    <form action={formAction} aria-describedby="example-form-status">
      <label htmlFor="example-title">Title</label>
      <input
        id="example-title"
        name="title"
        type="text"
        autoComplete="off"
        aria-invalid={Boolean(state.fieldErrors?.title)}
        aria-describedby={
          state.fieldErrors?.title ? "example-title-error" : undefined
        }
      />
      {state.fieldErrors?.title ? (
        <p id="example-title-error">{state.fieldErrors.title}</p>
      ) : null}

      {state.formError ? (
        <p id="example-form-status" role="alert">
          {state.formError}
        </p>
      ) : (
        <p id="example-form-status" aria-live="polite">
          {state.ok ? "Saved." : ""}
        </p>
      )}

      <button type="submit" disabled={isPending}>
        {isPending ? "Saving..." : "Save"}
      </button>
    </form>
  );
}
