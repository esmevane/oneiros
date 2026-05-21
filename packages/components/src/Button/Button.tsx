import { Button as BaseButton } from "@base-ui/react/button";
import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Button.module.css";

type ButtonVariant = "primary" | "accent" | "ghost" | "danger";
type ButtonSize = "sm" | "md";

interface ButtonProps extends ComponentPropsWithoutRef<typeof BaseButton> {
  variant?: ButtonVariant;
  size?: ButtonSize;
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  { variant = "primary", size = "md", className, children, ...props },
  ref,
) {
  const classes = [
    styles.button,
    styles[variant],
    size === "sm" ? styles.sm : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <BaseButton ref={ref} className={classes} {...props}>
      {children}
    </BaseButton>
  );
});

export { Button };
export type { ButtonProps, ButtonVariant, ButtonSize };
