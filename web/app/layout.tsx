import type { Metadata } from "next";
import "./globals.css";
import { Outfit } from 'next/font/google';
import { ThemeProvider } from "next-themes";

export const metadata: Metadata = {
  title: "Emittio — Anonymous Decentralized Email",
  description: "Emittio is a privacy-first decentralized email platform using IPFS, encryption, and smart contracts. Join the waitlist now!",
  keywords: [
    "Emittio",
    "decentralized email",
    "anonymous mail",
    "private email",
    "encrypted email",
    "IPFS mail",
    "privacy",
    "blockchain email",
    "secure messaging"
  ],
};

const outfit = Outfit({
  subsets: ['latin'],
  preload: true,
  // weight: ["200", "500"],
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="scroll-smooth" suppressHydrationWarning>
      <body className={`dark:bg-black bg-gray-50 dark:text-white ${outfit.className} min-w-screen min-h-svh flex flex-col`}>
        <svg id="filters"className="fixed h-0 w-0" preserveAspectRatio="xMidYMid slice">
          <defs>
            <filter id="morphing">
              <feColorMatrix
                  in="SourceGraphic"
                  type="matrix"
                  values="1 0 0 0 0
                          0 1 0 0 0
                          0 0 1 0 0
                          0 0 0 255 -140" />
            </filter>
          </defs>
        </svg>
        <ThemeProvider>
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}
