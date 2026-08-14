"use client";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <main>
      <h1>Something went wrong</h1>
      <p>{error.message || "The page could not be loaded."}</p>
      <button type="button" onClick={reset}>
        Try Again
      </button>
    </main>
  );
}
