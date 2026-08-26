"use client";

import Image from "next/image";
import { ExternalLink, Moon, Package, ScrollText, Sun } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { GitHub } from "@/components/icons/github";
import { siteLinks } from "@/constants/site-links";

type Theme = "light" | "dark";

const linkIcons = {
  github: GitHub,
  package: Package,
  docs: ScrollText,
  releases: ExternalLink,
};

export default function Home() {
  const [theme, setTheme] = useState<Theme>(() => {
    if (typeof window === "undefined") {
      return "dark";
    }

    const storedTheme = window.localStorage.getItem("siftforge-theme");
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;

    return storedTheme === "light" || storedTheme === "dark"
      ? storedTheme
      : prefersDark
        ? "dark"
        : "light";
  });

  useEffect(() => {
    document.documentElement.classList.toggle("dark", theme === "dark");
    window.localStorage.setItem("siftforge-theme", theme);
  }, [theme]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const tagName = target?.tagName.toLowerCase();

      if (
        event.key.toLowerCase() !== "d" ||
        event.metaKey ||
        event.ctrlKey ||
        event.altKey ||
        tagName === "input" ||
        tagName === "textarea" ||
        tagName === "select"
      ) {
        return;
      }

      setTheme((currentTheme) => (currentTheme === "dark" ? "light" : "dark"));
    };

    window.addEventListener("keydown", handleKeyDown);

    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const themeToggleLabel = useMemo(
    () => `Switch to ${theme === "dark" ? "light" : "dark"} mode`,
    [theme],
  );

  return (
    <main className="terminal-shell flex min-h-dvh items-center justify-center overflow-hidden px-3 py-3 font-mono text-(--terminal-fg) sm:px-6 sm:py-4">
      <section className="terminal-window grid h-[calc(100dvh-1.5rem)] w-full max-w-6xl grid-rows-[auto_1fr_auto] border border-(--terminal-border) bg-(--terminal-panel) sm:h-[calc(100dvh-2rem)]">
        <header className="flex items-center justify-between border-b border-(--terminal-border) bg-(--terminal-code) px-3 py-2 text-xs text-(--terminal-muted) sm:px-4">
          <div className="flex min-w-0 items-center gap-2">
            <span className="blink-led h-3 w-3 shrink-0 bg-(--terminal-accent)" />
            <span className="truncate">siftforge://site</span>
          </div>
          <div className="hidden items-center gap-3 text-(--terminal-muted) sm:flex">
            <span>mode:{theme}</span>
            <span>key:D</span>
          </div>
          <button
            type="button"
            aria-label={themeToggleLabel}
            title={`${themeToggleLabel} (D)`}
            onClick={() =>
              setTheme((currentTheme) =>
                currentTheme === "dark" ? "light" : "dark",
              )
            }
            className="inline-flex h-8 w-8 items-center justify-center border border-(--terminal-border) text-(--terminal-strong) transition-colors hover:bg-(--terminal-hover) focus:outline-none focus:ring-2 focus:ring-(--terminal-accent)"
          >
            {theme === "dark" ? (
              <Sun aria-hidden="true" className="h-4 w-4" />
            ) : (
              <Moon aria-hidden="true" className="h-4 w-4" />
            )}
          </button>
        </header>

        <div className="grid min-h-0 items-center gap-5 px-3 py-4 sm:px-8 sm:py-6 lg:grid-cols-[0.92fr_1.08fr] lg:gap-10 lg:px-12">
          <div className="terminal-readout flex min-h-0 flex-col justify-between border border-(--terminal-border) p-3 sm:p-4">
            <div className="flex items-center justify-between border-b border-(--terminal-border) pb-2 text-[10px] uppercase text-(--terminal-muted) sm:text-xs">
              <span>artifact preview</span>
              <span className="flex items-center gap-2">
                <span className="blink-led h-2 w-2 bg-(--terminal-accent)" />
                active
              </span>
            </div>

            <div className="flex flex-1 items-center justify-center py-4">
              <div className="relative h-20 w-60 sm:h-30 sm:w-88 lg:h-40 lg:w-116">
                <Image
                  src="/logo/siftforge-light-no-bg.png"
                  alt="SiftForge"
                  fill
                  priority
                  sizes="(min-width: 1024px) 464px, (min-width: 640px) 352px, 240px"
                  className="terminal-logo-glow object-contain dark:hidden"
                />
                <Image
                  src="/logo/siftforge-dark-no-bg.png"
                  alt="SiftForge"
                  fill
                  priority
                  sizes="(min-width: 1024px) 464px, (min-width: 640px) 352px, 240px"
                  className="terminal-logo-glow hidden object-contain dark:block"
                />
              </div>
            </div>

            <div className="space-y-1 border-t border-(--terminal-border) pt-2 text-[10px] text-(--terminal-muted) sm:text-xs">
              <p>
                <span className="text-(--terminal-accent)">ok</span> scan:
                preview-only by default
              </p>
              <p>
                <span className="text-(--terminal-accent)">ok</span> history:
                local undo records
              </p>
              <p className="signal-line h-4 border border-(--terminal-border)" />
            </div>
          </div>

          <div className="mx-auto flex w-full max-w-2xl flex-col gap-5 text-left">
            <div className="space-y-3">
              <p className="text-xs uppercase text-(--terminal-accent)">
                &gt; Forge order from clutter
                <span className="blink-cursor" />
              </p>
              <h1 className="text-3xl font-semibold leading-tight tracking-normal text-(--terminal-strong) sm:text-5xl">
                Safe file organization for your terminal.
              </h1>
              <p className="max-w-xl text-sm leading-6 text-(--terminal-muted) sm:text-base">
                SiftForge previews every move, applies changes only when asked,
                saves local history, and can undo the latest operation.
              </p>
            </div>

            <div className="border border-(--terminal-border) bg-(--terminal-code) text-sm text-(--terminal-strong)">
              <div className="border-b border-(--terminal-border) px-3 py-1 text-[10px] uppercase text-(--terminal-muted)">
                install
              </div>
              <div className="px-4 py-3">
                <span className="text-(--terminal-accent)">$</span> cargo
                install siftforge
                <span className="blink-cursor" />
              </div>
            </div>

            <nav
              aria-label="SiftForge links"
              className="grid grid-cols-2 gap-2 sm:flex sm:flex-wrap"
            >
              {siteLinks.map(({ href, label, icon }) => {
                const Icon = linkIcons[icon];

                return (
                  <a
                    key={href}
                    href={href}
                    target="_blank"
                    rel="noreferrer"
                    className="inline-flex h-11 items-center justify-center gap-2 border border-(--terminal-border) bg-(--terminal-panel) px-3 text-sm font-medium text-(--terminal-strong) transition-colors hover:bg-(--terminal-hover) hover:text-(--terminal-accent) focus:outline-none focus:ring-2 focus:ring-(--terminal-accent) sm:justify-start"
                  >
                    <Icon aria-hidden="true" className="h-4 w-4" />
                    {label}
                  </a>
                );
              })}
            </nav>
          </div>
        </div>

        <footer className="flex flex-wrap items-center gap-x-4 gap-y-1 border-t border-(--terminal-border) bg-(--terminal-code) px-3 py-2 text-xs text-(--terminal-muted) sm:px-4">
          <span>preview first</span>
          <span>apply explicitly</span>
          <span>undo locally</span>
        </footer>
      </section>
    </main>
  );
}
