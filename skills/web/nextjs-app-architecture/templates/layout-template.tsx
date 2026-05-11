import type { ReactNode } from "react";

export const metadata = {
  title: {
    default: "Example",
    template: "%s | Example",
  },
};

export default function ExampleLayout({ children }: { children: ReactNode }) {
  return (
    <section aria-labelledby="example-heading">
      <div>
        <h1 id="example-heading">Example</h1>
      </div>
      {children}
    </section>
  );
}
