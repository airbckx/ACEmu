import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  // Static export: `next build` writes a plain HTML/JS bundle to ./out,
  // which the Rust admin service serves on http://localhost:55990/.
  output: 'export',
  images: { unoptimized: true },
  trailingSlash: true,
};

export default nextConfig;
