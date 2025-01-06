import typography from "@tailwindcss/typography";
import type { Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {
      colors: {
        accent: "#f31559",
        "selection-color": "#f31559",
        secondary: "#d0bfff",
        error: "#ff002b",
        "dark-background": "#fbfbfb",
        "light-background": "#ffffff",
        success: "#09bc8a",
        "border-color": "#e8e8e8",
        primary: "#000000",
      },
      borderRadius: {
        none: "0",
      },
    },
  },

  plugins: [typography],
} satisfies Config;
