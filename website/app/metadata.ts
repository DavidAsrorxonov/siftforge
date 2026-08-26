import type { Metadata } from "next";

const siteName = "SiftForge";
const siteDescription =
  "A safe, cross-platform CLI for organizing cluttered directories with preview, apply, history, and undo.";

const defaultSiteUrl = "https://siftforge.dovudkon.com";

export const siteUrl = new URL(
  process.env.NEXT_PUBLIC_SITE_URL ?? defaultSiteUrl,
);

export const metadata: Metadata = {
  metadataBase: siteUrl,
  title: {
    default: siteName,
    template: `%s | ${siteName}`,
  },
  description: siteDescription,
  applicationName: siteName,
  keywords: [
    "SiftForge",
    "file organizer",
    "Rust CLI",
    "terminal tool",
    "directory cleanup",
    "safe file organization",
  ],
  authors: [{ name: "Dovudxon Asrorxonov" }],
  creator: "Dovudxon Asrorxonov",
  publisher: "SiftForge",
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    url: "/",
    title: siteName,
    description: siteDescription,
    siteName,
  },
  twitter: {
    card: "summary",
    title: siteName,
    description: siteDescription,
  },
  robots: {
    index: true,
    follow: true,
  },
};
