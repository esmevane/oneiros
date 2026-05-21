import { type ComponentPropsWithoutRef, forwardRef } from "react";

import styles from "./Badge.module.css";

export type BadgeVariant =
  | "primary"
  | "accent"
  | "success"
  | "warn"
  | "info"
  | "error"
  | "muted";

export interface BadgeProps extends ComponentPropsWithoutRef<"span"> {
  variant?: BadgeVariant;
}

export const Badge = forwardRef<HTMLSpanElement, BadgeProps>(function Badge(
  { variant = "muted", className, children, ...props },
  ref,
) {
  const classes = [styles.badge, styles[variant], className]
    .filter(Boolean)
    .join(" ");

  return (
    <span ref={ref} className={classes} {...props}>
      {children}
    </span>
  );
});
