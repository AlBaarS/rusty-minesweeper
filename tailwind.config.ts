import type { Config } from "tailwindcss";

export default {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
      },
    },
  },
  safelist: [
    "hover:border-red-400",
    "hover:bg-red-400",
    "hover:border-green-400",
    "hover:bg-green-400",
    "hover:border-blue-400",
    "hover:bg-blue-400",
    "hover:border-yellow-400",
    "hover:bg-yellow-400",
  ],
  plugins: [],
} satisfies Config;
