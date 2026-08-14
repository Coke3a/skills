export default function Loading() {
  return (
    <main aria-busy="true" aria-live="polite">
      <h1>Loading...</h1>
      <div role="status">Loading content...</div>
    </main>
  );
}
