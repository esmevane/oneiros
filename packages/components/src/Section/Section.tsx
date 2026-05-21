import {
  createContext,
  forwardRef,
  useContext,
  useMemo,
  useState,
  type ComponentPropsWithoutRef,
} from "react";
import styles from "./Section.module.css";

type SectionState = "expanded" | "collapsed";

interface SectionContextValue {
  state: SectionState;
  toggle: () => void;
}

const SectionContext = createContext<SectionContextValue | null>(null);

function useSection(): SectionContextValue {
  const value = useContext(SectionContext);
  if (!value) {
    throw new Error(
      "Section.Header / Section.Body must be inside <Section.Container>",
    );
  }
  return value;
}

interface ContainerProps extends ComponentPropsWithoutRef<"div"> {
  /** Sets initial state to "collapsed". Default is "expanded". */
  collapsed?: boolean;
}

const Container = forwardRef<HTMLDivElement, ContainerProps>(
  function SectionContainer({ collapsed, className, ...props }, ref) {
    const [state, setState] = useState<SectionState>(
      collapsed ? "collapsed" : "expanded",
    );
    const toggle = () =>
      setState((current) =>
        current === "expanded" ? "collapsed" : "expanded",
      );
    const value = useMemo(() => ({ state, toggle }), [state]);

    const classes = [styles.container, className].filter(Boolean).join(" ");

    return (
      <SectionContext.Provider value={value}>
        <div ref={ref} className={classes} {...props} />
      </SectionContext.Provider>
    );
  },
);

const Header = forwardRef<
  HTMLButtonElement,
  ComponentPropsWithoutRef<"button">
>(function SectionHeader({ className, onClick, ...props }, ref) {
  const { state, toggle } = useSection();
  const classes = [styles.header, className].filter(Boolean).join(" ");
  return (
    <button
      ref={ref}
      type="button"
      className={classes}
      data-state={state}
      aria-expanded={state === "expanded"}
      onClick={(event) => {
        onClick?.(event);
        if (!event.defaultPrevented) {
          toggle();
        }
      }}
      {...props}
    />
  );
});

const Body = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  function SectionBody({ className, ...props }, ref) {
    const { state } = useSection();
    const classes = [styles.body, className].filter(Boolean).join(" ");
    return (
      <div
        ref={ref}
        className={classes}
        data-state={state}
        hidden={state === "collapsed"}
        {...props}
      />
    );
  },
);

export const Section = { Container, Header, Body } as const;
