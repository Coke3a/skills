"use server";

import { revalidateTag } from "next/cache";

import { saveExample } from "@/features/example/services/save-example";
import { exampleSchema } from "@/features/example/schemas/example-schema";

export type ExampleActionState = {
  ok: boolean;
  fieldErrors?: Record<string, string>;
  formError?: string;
};

export async function saveExampleAction(
  _previousState: ExampleActionState,
  formData: FormData,
): Promise<ExampleActionState> {
  const parsed = exampleSchema.safeParse({
    title: formData.get("title"),
  });

  if (!parsed.success) {
    return {
      ok: false,
      fieldErrors: {
        title: parsed.error.flatten().fieldErrors.title?.[0] ?? "Invalid title.",
      },
    };
  }

  try {
    await saveExample(parsed.data);
    revalidateTag("examples");
    return { ok: true };
  } catch {
    return {
      ok: false,
      formError: "The example could not be saved.",
    };
  }
}
