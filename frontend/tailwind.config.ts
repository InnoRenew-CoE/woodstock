import typography from "@tailwindcss/typography";
import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
  content: ["./src/**/*.{html,js,svelte,ts,css}"],

  theme: {
    extend: {
      colors: {
        accent: "#f31559",
        "selection-color": "#f31559",
        secondary: "#818cf8",
        error: "#ff002b",
        "dark-background": "#fbfbfb",
        "light-background": "#ffffff",
        success: "#09bc8a",
        "border-color": "#e8e8e8",
        primary: "#000000",
      },
      fontFamily: {
        nunito: "Nunito",
      },
      borderRadius: {
        "rounded-0": "0rem",
        "rounded-1": "0.4166666666666667rem",
        "rounded-2": "0.75rem",
        "rounded-3": "0.8333333333333334rem",
        "rounded-4": "1.25rem",
        "rounded-5": "3.1666666666666665rem",
      },
    },
  },

  plugins: [typography],
} satisfies Config;
