import { Link, Section } from "@oneiros/components";

type SidebarEntry =
  | {
      type: "link";
      href: string;
      label: string;
      isCurrent: boolean;
    }
  | {
      type: "group";
      label: string;
      entries: SidebarEntry[];
      collapsed: boolean;
    };

interface DocsSidebarProps {
  entries: SidebarEntry[];
}

/** Render Starlight's sidebar data through our design system. Each top-level
 *  group becomes a Section; each link becomes a Link.Nav. Sub-groups render
 *  as sub-sections (collapsed by default). */
export function DocsSidebar({ entries }: DocsSidebarProps) {
  return <Entries entries={entries} />;
}

function Entries({ entries }: { entries: SidebarEntry[] }) {
  return entries.map((entry) =>
    entry.type === "group" ? (
      <Section.Container key={entry.label} collapsed={entry.collapsed}>
        <Section.Header>{entry.label}</Section.Header>
        <Section.Body>
          <Entries entries={entry.entries} />
        </Section.Body>
      </Section.Container>
    ) : (
      <Link.Nav key={entry.href} href={entry.href} active={entry.isCurrent} sub>
        {entry.label}
      </Link.Nav>
    ),
  );
}
