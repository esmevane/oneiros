import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Link.module.css";

interface LinkProps extends ComponentPropsWithoutRef<"a"> {}

interface NavProps extends ComponentPropsWithoutRef<"a"> {
  active?: boolean;
  sub?: boolean;
}

const Base = forwardRef<HTMLAnchorElement, LinkProps>(function Link(
  { className, ...props },
  ref,
) {
  const classes = [styles.link, className].filter(Boolean).join(" ");
  return <a ref={ref} className={classes} {...props} />;
});

const Nav = forwardRef<HTMLAnchorElement, NavProps>(function LinkNav(
  { active = false, sub = false, className, ...props },
  ref,
) {
  const classes = [
    styles.nav,
    active ? styles.active : "",
    sub ? styles.sub : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");
  return <a ref={ref} className={classes} {...props} />;
});

type LinkType = typeof Base & { Nav: typeof Nav };

export const Link = Base as LinkType;
Link.Nav = Nav;
