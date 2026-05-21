import {
  type ComponentPropsWithoutRef,
  type ReactNode,
  forwardRef,
} from "react";
import styles from "./Card.module.css";

interface CardProps extends ComponentPropsWithoutRef<"div"> {
  title?: string;
  subtitle?: string;
  header?: ReactNode;
}

const Card = forwardRef<HTMLDivElement, CardProps>(function Card(
  { title, subtitle, header, className, children, ...props },
  ref,
) {
  const classes = [styles.card, className].filter(Boolean).join(" ");
  const showHeader = title || subtitle || header;

  return (
    <div ref={ref} className={classes} {...props}>
      {showHeader && (
        <div className={styles.header}>
          {header ?? (
            <>
              {title && <div className={styles.title}>{title}</div>}
              {subtitle && <div className={styles.subtitle}>{subtitle}</div>}
            </>
          )}
        </div>
      )}
      <div className={styles.body}>{children}</div>
    </div>
  );
});

export { Card };
export type { CardProps };
