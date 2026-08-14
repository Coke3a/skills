import type { ReactNode } from "react";

type NoticeVariant = "info" | "success" | "warning" | "danger";

type NoticeProps = {
  variant: NoticeVariant;
  title: string;
  children: ReactNode;
};

export function Notice({ variant, title, children }: NoticeProps) {
  return (
    <section data-variant={variant} aria-labelledby="notice-title">
      <h2 id="notice-title">{title}</h2>
      <div>{children}</div>
    </section>
  );
}

export function SuccessNotice({
  title,
  children,
}: Omit<NoticeProps, "variant">) {
  return (
    <Notice variant="success" title={title}>
      {children}
    </Notice>
  );
}
