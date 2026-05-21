import {
  type ComponentPropsWithoutRef,
  type ElementType,
  forwardRef,
} from "react";
import styles from "./Text.module.css";

type TextSize =
  | "3xs"
  | "xxs"
  | "xs"
  | "sm"
  | "md"
  | "lg"
  | "xl"
  | "xxl"
  | "3xl";
type TextFont = "sans" | "serif" | "mono";
type TextWeight = "normal" | "medium" | "semibold" | "bold";
type TextColor = "default" | "muted" | "secondary";

const weightMap: Record<TextWeight, string> = {
  normal: "var(--font-weight-400)",
  medium: "var(--font-weight-500)",
  semibold: "var(--font-weight-600)",
  bold: "var(--font-weight-700)",
};

interface TextProps extends ComponentPropsWithoutRef<"span"> {
  size?: TextSize;
  font?: TextFont;
  weight?: TextWeight;
  color?: TextColor;
  italic?: boolean;
  uppercase?: boolean;
  as?: ElementType;
}

const Text = forwardRef<HTMLElement, TextProps>(function Text(
  {
    size,
    font,
    weight,
    color,
    italic = false,
    uppercase = false,
    as: Component = "span",
    className,
    style,
    ...props
  },
  ref,
) {
  const classes = [styles.text, className].filter(Boolean).join(" ");

  const textStyle: React.CSSProperties = { ...style };
  if (size) textStyle.fontSize = `var(--font-size-${size})`;
  if (font) textStyle.fontFamily = `var(--font-family-${font})`;
  if (weight) textStyle.fontWeight = weightMap[weight];
  if (color === "muted") textStyle.color = "var(--color-text-muted)";
  if (color === "secondary") textStyle.color = "var(--color-text-secondary)";
  if (italic) textStyle.fontStyle = "italic";
  if (uppercase) {
    textStyle.textTransform = "uppercase";
    textStyle.letterSpacing = "var(--letter-spacing-wide)";
  }

  return (
    <Component ref={ref} className={classes} style={textStyle} {...props} />
  );
});

export { Text };
export type { TextProps, TextSize, TextFont, TextWeight, TextColor };
