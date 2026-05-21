import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Stack.module.css";

type StackGap =
  | "3xs"
  | "xxs"
  | "xs"
  | "sm"
  | "md"
  | "lg"
  | "xl"
  | "xxl"
  | "3xl";

interface StackProps extends ComponentPropsWithoutRef<"div"> {
  gap?: StackGap;
}

const Stack = forwardRef<HTMLDivElement, StackProps>(function Stack(
  { gap = "md", className, style, ...props },
  ref,
) {
  const classes = [styles.stack, className].filter(Boolean).join(" ");

  return (
    <div
      ref={ref}
      className={classes}
      style={
        {
          ...style,
          "--stack-gap": `var(--space-${gap})`,
        } as React.CSSProperties
      }
      {...props}
    />
  );
});

export { Stack };
export type { StackProps, StackGap };
