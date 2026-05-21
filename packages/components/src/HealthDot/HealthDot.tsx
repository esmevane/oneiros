import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./HealthDot.module.css";

type HealthStatus = "current" | "drifting" | "critical" | "inactive";

interface HealthDotProps extends ComponentPropsWithoutRef<"span"> {
  status?: HealthStatus;
}

const HealthDot = forwardRef<HTMLSpanElement, HealthDotProps>(
  function HealthDot(
    { status = "inactive", className, children, ...props },
    ref,
  ) {
    const classes = [styles.health, className].filter(Boolean).join(" ");

    return (
      <span ref={ref} className={classes} {...props}>
        <span className={`${styles.indicator} ${styles[status]}`} />
        {children && <span className={styles.label}>{children}</span>}
      </span>
    );
  },
);

export { HealthDot };
export type { HealthDotProps, HealthStatus };
