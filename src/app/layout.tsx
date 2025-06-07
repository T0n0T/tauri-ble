"use client";
import { Geist, Geist_Mono } from "next/font/google";
import "@/styles/globals.css";
import MainLayout from "@/view/main-layout"; // 导入 MainLayout

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <MainLayout>{children}</MainLayout> {/* 使用 MainLayout 包裹 children */}
      </body>
    </html>
  );
}
