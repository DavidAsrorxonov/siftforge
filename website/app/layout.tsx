import "./globals.css";
import { Analytics } from "@vercel/analytics/next";

export { metadata } from "./metadata";

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className="h-full antialiased" suppressHydrationWarning>
      <body className="min-h-full flex flex-col">
        {children}

        <Analytics />
      </body>
    </html>
  );
}
