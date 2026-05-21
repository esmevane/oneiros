import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Layout.module.css";

const Container = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  function LayoutContainer({ className, ...props }, ref) {
    const classes = [styles.container, className].filter(Boolean).join(" ");
    return <div ref={ref} className={classes} {...props} />;
  },
);

const Aside = forwardRef<HTMLElement, ComponentPropsWithoutRef<"aside">>(
  function LayoutAside({ className, ...props }, ref) {
    const classes = [styles.aside, className].filter(Boolean).join(" ");
    return <aside ref={ref} className={classes} {...props} />;
  },
);

const Main = forwardRef<HTMLElement, ComponentPropsWithoutRef<"main">>(
  function LayoutMain({ className, ...props }, ref) {
    const classes = [styles.main, className].filter(Boolean).join(" ");
    return <main ref={ref} className={classes} {...props} />;
  },
);

export const Layout = { Container, Aside, Main } as const;
