import { getExample } from "@/features/example/services/get-example";

type ExampleServerSectionProps = {
  id: string;
};

export async function ExampleServerSection({ id }: ExampleServerSectionProps) {
  const example = await getExample(id);

  if (!example) {
    return <p>No example found.</p>;
  }

  return (
    <section aria-labelledby="example-section-heading">
      <h2 id="example-section-heading">{example.title}</h2>
      <p>{example.description}</p>
    </section>
  );
}
