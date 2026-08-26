"use client";

import Image from "next/image";
import {
  Check,
  Copy,
  ExternalLink,
  Moon,
  Package,
  ScrollText,
  Sun,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { GitHub } from "@/components/icons/github";
import { installCommands } from "@/constants/install-commands";
import { siteLinks } from "@/constants/site-links";

type Theme = "light" | "dark";
type InstallMethod = (typeof installCommands)[number]["id"];

const linkIcons = {
  github: GitHub,
  package: Package,
  docs: ScrollText,
  releases: ExternalLink,
};

export default function Home() {
  const [copied, setCopied] = useState(false);
  const [installMethod, setInstallMethod] = useState<InstallMethod>("cargo");
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

  useEffect(() => {
    if (!copied) {
      return;
    }

    const timeout = window.setTimeout(() => setCopied(false), 1500);

    return () => window.clearTimeout(timeout);
  }, [copied]);

  const activeInstallCommand = useMemo(
    () =>
      installCommands.find((command) => command.id === installMethod) ??
      installCommands[0],
    [installMethod],
  );

  const themeToggleLabel = useMemo(
    () => `Switch to ${theme === "dark" ? "light" : "dark"} mode`,
    [theme],
  );

  const handleCopyCommand = async () => {
    try {
      if (!navigator.clipboard) {
        throw new Error("Clipboard API is not available");
      }

      await navigator.clipboard.writeText(activeInstallCommand.command);
    } catch {
      const textArea = document.createElement("textarea");
      textArea.value = activeInstallCommand.command;
      textArea.setAttribute("readonly", "");
      textArea.style.position = "fixed";
      textArea.style.opacity = "0";
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand("copy");
      document.body.removeChild(textArea);
    }

    setCopied(true);
  };

  return (
    <main className="terminal-shell flex min-h-dvh items-center justify-center overflow-x-hidden px-3 py-3 font-mono text-(--terminal-fg) sm:px-6 sm:py-4">
      <section className="terminal-window grid min-h-[calc(100dvh-1.5rem)] w-full max-w-6xl grid-rows-[auto_1fr_auto] border border-(--terminal-border) bg-(--terminal-panel) sm:h-[calc(100dvh-2rem)] sm:min-h-0">
        <header className="flex h-11 items-center justify-between border-b border-(--terminal-border) bg-(--terminal-code) px-3 text-[11px] text-(--terminal-code-fg) sm:h-auto sm:px-4 sm:py-2 sm:text-xs">
          <div className="flex min-w-0 items-center gap-2">
            <span className="h-3 w-3 shrink-0 bg-(--terminal-accent) sm:h-3 sm:w-3" />
            <span className="truncate">https://siftforge.dovudkhon.com</span>
          </div>
          <div className="hidden items-center gap-3 opacity-75 sm:flex">
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
            className="ml-2 inline-flex h-8 w-8 shrink-0 items-center justify-center border border-(--terminal-border) text-(--terminal-code-fg) transition-colors hover:border-(--terminal-accent) hover:text-(--terminal-accent) focus:outline-none focus:ring-2 focus:ring-(--terminal-accent)"
          >
            {theme === "dark" ? (
              <Sun aria-hidden="true" className="h-4 w-4" />
            ) : (
              <Moon aria-hidden="true" className="h-4 w-4" />
            )}
          </button>
        </header>

        <div className="grid min-h-0 items-center gap-4 px-3 py-3 sm:gap-5 sm:px-8 sm:py-6 lg:grid-cols-[0.92fr_1.08fr] lg:gap-10 lg:px-12">
          <div className="terminal-readout flex min-h-0 h-58 flex-col justify-between border border-(--terminal-border) p-3 sm:h-auto sm:p-4">
            <div className="flex items-center justify-between border-b border-(--terminal-border) pb-2 text-[9px] uppercase text-(--terminal-muted) sm:text-xs">
              <span>artifact preview</span>
              <span className="flex items-center gap-2">
                <span className="h-2 w-2 bg-(--terminal-accent)" />
                active
              </span>
            </div>

            <div className="flex flex-1 items-center justify-center py-2 sm:py-4">
              <div className="relative h-14 w-48 sm:h-30 sm:w-88 lg:h-40 lg:w-116">
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

            <div className="space-y-1 border-t border-(--terminal-border) pt-2 text-[9px] text-(--terminal-muted) sm:text-xs">
              <p>
                <span className="text-(--terminal-accent)">ok</span> scan:
                preview-only by default
              </p>
              <p>
                <span className="text-(--terminal-accent)">ok</span> history:
                local undo records
              </p>
              <p className="h-3 border border-(--terminal-border) bg-(--terminal-accent-soft) sm:h-4" />
            </div>
          </div>

          <div className="mx-auto flex min-w-0 w-full max-w-2xl flex-col gap-4 text-left sm:gap-5">
            <div className="space-y-2 sm:space-y-3">
              <p className="text-[11px] uppercase text-(--terminal-accent) sm:text-xs">
                &gt; Forge order from clutter
              </p>
              <h1 className="text-[2.35rem] font-semibold leading-[1.08] tracking-normal text-(--terminal-strong) sm:text-5xl">
                Safe file organization for your terminal.
              </h1>
              <p className="max-w-xl text-[13px] leading-6 text-(--terminal-muted) sm:text-base">
                SiftForge previews every move, applies changes only when asked,
                saves local history, and can undo the latest operation.
              </p>
            </div>

            <div className="min-w-0 border border-(--terminal-border) bg-(--terminal-code) text-[13px] text-(--terminal-code-fg) sm:text-sm">
              <div className="flex items-center justify-between gap-2 border-b border-(--terminal-border) px-2 py-2 text-[10px] uppercase">
                <div className="flex min-w-0 gap-1">
                  {installCommands.map((command) => {
                    const isActive = command.id === installMethod;

                    return (
                      <button
                        key={command.id}
                        type="button"
                        aria-pressed={isActive}
                        onClick={() => {
                          setInstallMethod(command.id);
                          setCopied(false);
                        }}
                        className={`border px-2 py-1 transition-colors focus:outline-none focus:ring-2 focus:ring-(--terminal-accent) ${
                          isActive
                            ? "border-(--terminal-accent) bg-(--terminal-accent) text-(--terminal-code)"
                            : "border-(--terminal-border) text-(--terminal-code-fg) hover:border-(--terminal-accent)"
                        }`}
                      >
                        {command.label}
                      </button>
                    );
                  })}
                </div>
                <button
                  type="button"
                  aria-label={`Copy ${activeInstallCommand.label} install command`}
                  title="Copy command"
                  onClick={handleCopyCommand}
                  className="inline-flex h-8 w-8 shrink-0 items-center justify-center border border-(--terminal-border) text-(--terminal-code-fg) transition-colors hover:border-(--terminal-accent) hover:text-(--terminal-accent) focus:outline-none focus:ring-2 focus:ring-(--terminal-accent)"
                >
                  {copied ? (
                    <Check aria-hidden="true" className="h-4 w-4" />
                  ) : (
                    <Copy aria-hidden="true" className="h-4 w-4" />
                  )}
                </button>
              </div>
              <div className="min-w-0 px-3 py-3 sm:px-4">
                <code className="block min-h-5 whitespace-pre-wrap break-all leading-6">
                  <span className="text-(--terminal-accent)">$</span>{" "}
                  {activeInstallCommand.command}
                  <span className="blink-cursor" />
                </code>
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
                    className="inline-flex h-10 items-center justify-center gap-2 border border-(--terminal-border) bg-(--terminal-panel) px-3 text-sm font-medium text-(--terminal-strong) transition-colors hover:bg-(--terminal-hover) hover:text-(--terminal-accent) focus:outline-none focus:ring-2 focus:ring-(--terminal-accent) sm:h-11 sm:justify-start"
                  >
                    <Icon aria-hidden="true" className="h-4 w-4" />
                    {label}
                  </a>
                );
              })}
            </nav>
          </div>
        </div>

        <footer className="grid grid-cols-3 border-t border-(--terminal-border) bg-(--terminal-code) px-3 py-2 text-center text-[10px] text-(--terminal-code-fg) sm:flex sm:flex-wrap sm:items-center sm:gap-x-4 sm:gap-y-1 sm:px-4 sm:text-xs">
          <span>preview first</span>
          <span>apply explicitly</span>
          <span>undo locally</span>
        </footer>
      </section>
    </main>
  );
}
