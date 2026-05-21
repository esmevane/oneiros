import { Separator as BaseSeparator } from "@base-ui/react/separator";
import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Separator.module.css";

interface SeparatorProps extends ComponentPropsWithoutRef<
  typeof BaseSeparator
> {
  orientation?: "horizontal" | "vertical";
}

const Separator = forwardRef<HTMLDivElement, SeparatorProps>(function Separator(
  { className, ...props },
  ref,
) {
  return (
    <BaseSeparator
      ref={ref}
      className={[styles.separator, className].filter(Boolean).join(" ")}
      {...props}
    />
  );
});

export { Separator };
export type { SeparatorProps };
