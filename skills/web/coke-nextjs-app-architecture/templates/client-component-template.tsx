"use client";

import { startTransition, useState } from "react";

type ExampleClientPanelProps = {
  initialValue: string;
  onSave?: (value: string) => Promise<void>;
};

export function ExampleClientPanel({
  initialValue,
  onSave,
}: ExampleClientPanelProps) {
  const [value, setValue] = useState(initialValue);
  const [isPending, setIsPending] = useState(false);

  function handleSave() {
    if (!onSave) return;

    setIsPending(true);
    startTransition(async () => {
      try {
        await onSave(value);
      } finally {
        setIsPending(false);
      }
    });
  }

  return (
    <section aria-labelledby="example-client-panel-heading">
      <h2 id="example-client-panel-heading">Example Panel</h2>
      <label htmlFor="example-value">Value</label>
      <input
        id="example-value"
        name="exampleValue"
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />
      <button type="button" onClick={handleSave} disabled={isPending}>
        {isPending ? "Saving..." : "Save"}
      </button>
    </section>
  );
}
