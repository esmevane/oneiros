import styles from "./Cards.module.css";

/**
 * Card layout for the homepage feature grid
 */
export function Cards({
  className,
  ...rest
}: React.DetailedHTMLProps<
  React.HTMLAttributes<HTMLDivElement>,
  HTMLDivElement
>) {
  // `not-content` opts the children out of Starlight's prose styling
  // (the @layer starlight.content rules in node_modules/@astrojs/starlight/style/markdown.css).
  const classes = ["not-content", styles.container, className]
    .filter(Boolean)
    .join(" ");
  return <div className={classes} {...rest} />;
}
