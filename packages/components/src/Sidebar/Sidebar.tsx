import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Sidebar.module.css";

export const Sidebar = forwardRef<
  HTMLDivElement,
  ComponentPropsWithoutRef<"div">
>(function Sidebar({ className, ...props }, ref) {
  const classes = [styles.sidebar, className].filter(Boolean).join(" ");
  return <div ref={ref} className={classes} {...props} />;
});
