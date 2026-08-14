import { Suspense } from "react";

import { FeatureSection } from "@/features/example/components/feature-section";
import { FeatureSectionSkeleton } from "@/features/example/components/feature-section-skeleton";
import { getExampleSummary } from "@/features/example/services/get-example-summary";

export const metadata = {
  title: "Example",
};

export default async function ExamplePage() {
  const summaryPromise = getExampleSummary();

  return (
    <main>
      <h1>Example</h1>
      <Suspense fallback={<FeatureSectionSkeleton />}>
        <FeatureSection summaryPromise={summaryPromise} />
      </Suspense>
    </main>
  );
}
