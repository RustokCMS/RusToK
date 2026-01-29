/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./assets/**/*.css"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        rustok: {
          primary: "#3b82f6",
          "primary-content": "#ffffff",
          secondary: "#6366f1",
          "secondary-content": "#ffffff",
          accent: "#f59e0b",
          "accent-content": "#000000",
          neutral: "#1f2937",
          "neutral-content": "#ffffff",
          "base-100": "#ffffff",
          "base-200": "#f3f4f6",
          "base-300": "#e5e7eb",
          "base-content": "#1f2937",
          info: "#3abff8",
          success: "#22c55e",
          warning: "#f59e0b",
          error: "#ef4444",
        },
        rustok_dark: {
          primary: "#60a5fa",
          "primary-content": "#000000",
          secondary: "#818cf8",
          "secondary-content": "#000000",
          accent: "#fbbf24",
          "accent-content": "#000000",
          neutral: "#374151",
          "neutral-content": "#ffffff",
          "base-100": "#1f2937",
          "base-200": "#111827",
          "base-300": "#0f172a",
          "base-content": "#f3f4f6",
          info: "#3abff8",
          success: "#22c55e",
          warning: "#f59e0b",
          error: "#ef4444",
        },
      },
      "light",
      "dark",
    ],
    darkTheme: "rustok_dark",
  },
};
