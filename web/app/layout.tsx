import type { Metadata } from "next";
import "./globals.css";
import { Source_Serif_4 } from 'next/font/google';

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

const source_Serif_4 = Source_Serif_4();

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`dark:bg-black bg-gray-50 dark:text-white ${source_Serif_4.className}`}>
          {children}
      </body>
    </html>
  );
}
