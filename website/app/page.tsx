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
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;

    return (
      storedTheme === "light" || storedTheme === "dark"
        ? storedTheme
        : prefersDark
          ? "dark"
          : "light"
    );
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

      setTheme((currentTheme) =>
        currentTheme === "dark" ? "light" : "dark",
      );
    };

    window.addEventListener("keydown", handleKeyDown);

    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const themeToggleLabel = useMemo(
    () => `Switch to ${theme === "dark" ? "light" : "dark"} mode`,
    [theme],
  );

  return (
    <main className="flex min-h-dvh items-center justify-center overflow-hidden bg-[var(--terminal-bg)] px-4 py-4 font-mono text-[var(--terminal-fg)] sm:px-6">
      <section className="grid h-[calc(100dvh-2rem)] w-full max-w-6xl grid-rows-[auto_1fr_auto] border border-[var(--terminal-border)] bg-[var(--terminal-panel)]">
        <header className="flex items-center justify-between border-b border-[var(--terminal-border)] px-3 py-2 text-xs text-[var(--terminal-muted)] sm:px-4">
          <div className="flex min-w-0 items-center gap-2">
            <span className="h-3 w-3 shrink-0 bg-[var(--terminal-accent)]" />
            <span className="truncate">siftforge://site</span>
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
            className="inline-flex h-8 w-8 items-center justify-center border border-[var(--terminal-border)] text-[var(--terminal-strong)] transition-colors hover:bg-[var(--terminal-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--terminal-accent)]"
          >
            {theme === "dark" ? (
              <Sun aria-hidden="true" className="h-4 w-4" />
            ) : (
              <Moon aria-hidden="true" className="h-4 w-4" />
            )}
          </button>
        </header>

        <div className="grid min-h-0 items-center gap-6 px-4 py-5 sm:px-8 lg:grid-cols-[0.86fr_1.14fr] lg:gap-10 lg:px-12">
          <div className="flex min-h-0 items-center justify-center">
            <div className="relative h-[6rem] w-[16rem] sm:h-[8rem] sm:w-[21rem] lg:h-[11rem] lg:w-[28rem]">
              <Image
                src="/logo/siftforge-light.png"
                alt="SiftForge"
                fill
                priority
                sizes="(min-width: 1024px) 448px, (min-width: 640px) 336px, 256px"
                className="object-contain dark:hidden"
              />
              <Image
                src="/logo/siftforge-dark.png"
                alt="SiftForge"
                fill
                priority
                sizes="(min-width: 1024px) 448px, (min-width: 640px) 336px, 256px"
                className="hidden object-contain dark:block"
              />
            </div>
          </div>

          <div className="mx-auto flex w-full max-w-2xl flex-col gap-5 text-left">
            <div className="space-y-3">
              <p className="text-xs uppercase text-[var(--terminal-accent)]">
                &gt; Forge order from clutter
              </p>
              <h1 className="text-3xl font-semibold leading-tight tracking-normal text-[var(--terminal-strong)] sm:text-5xl">
                Safe file organization for your terminal.
              </h1>
              <p className="max-w-xl text-sm leading-6 text-[var(--terminal-muted)] sm:text-base">
                SiftForge previews every move, applies changes only when asked,
                saves local history, and can undo the latest operation.
              </p>
            </div>

            <div className="border border-[var(--terminal-border)] bg-[var(--terminal-code)] px-4 py-3 text-sm text-[var(--terminal-strong)]">
              <span className="text-[var(--terminal-accent)]">$</span> cargo
              install siftforge
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
                    className="inline-flex h-11 items-center justify-center gap-2 border border-[var(--terminal-border)] px-3 text-sm font-medium text-[var(--terminal-strong)] transition-colors hover:bg-[var(--terminal-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--terminal-accent)] sm:justify-start"
                  >
                    <Icon aria-hidden="true" className="h-4 w-4" />
                    {label}
                  </a>
                );
              })}
            </nav>
          </div>
        </div>

        <footer className="flex flex-wrap items-center gap-x-4 gap-y-1 border-t border-[var(--terminal-border)] px-3 py-2 text-xs text-[var(--terminal-muted)] sm:px-4">
          <span>preview first</span>
          <span>apply explicitly</span>
          <span>undo locally</span>
        </footer>
      </section>
    </main>
  );
}
