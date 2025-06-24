import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactStrictMode: false,
  output: "export",
  images: {
    unoptimized: true,
  },
  distDir: "dist",
  eslint: {
    ignoreDuringBuilds: true,
  },
  // Add experimental features for Turbopack compatibility with next/font
  experimental: {
    // This might help with font loading issues in Turbopack
    // You might need to adjust this based on your Next.js version and Turbopack setup
    // For example, `optimizePackageImports` might be relevant for font modules
    // optimizePackageImports: ['next/font'],
  },
};

export default nextConfig;
