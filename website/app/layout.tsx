import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "SiftForge",
  description: "A safe, cross-platform CLI for organizing cluttered directories.",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className="h-full antialiased" suppressHydrationWarning>
      <body className="min-h-full flex flex-col">{children}</body>
    </html>
  );
}
