import { type ComponentPropsWithoutRef, forwardRef } from "react";
import styles from "./Hero.module.css";

const Container = forwardRef<HTMLElement, ComponentPropsWithoutRef<"header">>(
  function HeroContainer({ className, ...props }, ref) {
    const classes = [styles.container, className].filter(Boolean).join(" ");
    return <header ref={ref} className={classes} {...props} />;
  },
);

const Eyebrow = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  function HeroEyebrow({ className, ...props }, ref) {
    const classes = [styles.eyebrow, className].filter(Boolean).join(" ");
    return <div ref={ref} className={classes} {...props} />;
  },
);

const Title = forwardRef<HTMLHeadingElement, ComponentPropsWithoutRef<"h1">>(
  function HeroTitle({ className, ...props }, ref) {
    const classes = [styles.title, className].filter(Boolean).join(" ");
    return <h1 ref={ref} className={classes} {...props} />;
  },
);

const Subtitle = forwardRef<
  HTMLParagraphElement,
  ComponentPropsWithoutRef<"p">
>(function HeroSubtitle({ className, ...props }, ref) {
  const classes = [styles.subtitle, className].filter(Boolean).join(" ");
  return <p ref={ref} className={classes} {...props} />;
});

export const Hero = { Container, Eyebrow, Title, Subtitle } as const;
