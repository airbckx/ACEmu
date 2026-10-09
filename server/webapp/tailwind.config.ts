import type { Config } from 'tailwindcss';
import colors from 'tailwindcss/colors';

const config: Config = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        // Single accent colour used across the dashboard.
        accent: colors.sky,
      },
    },
  },
  plugins: [],
};

export default config;
