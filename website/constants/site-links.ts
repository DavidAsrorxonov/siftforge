export type SiteLink = {
  href: string;
  label: string;
  icon: "github" | "package" | "docs" | "releases";
};

export const siteLinks: SiteLink[] = [
  {
    href: "https://github.com/DovudxonN/siftforge",
    label: "GitHub",
    icon: "github",
  },
  {
    href: "https://crates.io/crates/siftforge",
    label: "crates.io",
    icon: "package",
  },
  {
    href: "https://github.com/DovudxonN/siftforge#readme",
    label: "Docs",
    icon: "docs",
  },
  {
    href: "https://github.com/DovudxonN/siftforge/releases/latest",
    label: "Releases",
    icon: "releases",
  },
];
