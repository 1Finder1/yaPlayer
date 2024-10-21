import {fontFamily} from "tailwindcss/defaultTheme";

const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
const config = {
    darkMode: ["class"],
    content: ["./src/**/*.{html,js,svelte,ts}"],
    safelist: ["dark"],
    theme: {
        container: {
            center: true,
            padding: "2rem",
            screens: {
                "2xl": "1400px"
            }
        },
        extend: {
            colors: {
                border: "hsl(var(--border) / <alpha-value>)",
                input: "hsl(var(--input) / <alpha-value>)",
                ring: "hsl(var(--ring) / <alpha-value>)",
                background: "hsl(var(--background) / <alpha-value>)",
                foreground: "hsl(var(--foreground) / <alpha-value>)",
                "yellow": {
                    50: "#FCF9F2",
                    100: "#FAF3E5",
                    200: "#F5E7CC",
                    300: "#EFD8AE",
                    400: "#EACC94",
                    500: "#E5C07A",
                    600: "#D9A33F",
                    700: "#AF7E22",
                    800: "#775617",
                    900: "#3C2B0C",
                    950: "#1E1506"
                },
                "danger": {
                    50: "#FDEDED",
                    100: "#FBDBDB",
                    200: "#F6B6B6",
                    300: "#F29292",
                    400: "#ED6E6E",
                    500: "#E94C4C",
                    600: "#DA1B1B",
                    700: "#A31414",
                    800: "#6D0D0D",
                    900: "#360707",
                    950: "#1B0303"
                },
                "success": {
                    50: "#F3FBEF",
                    100: "#E7F7DE",
                    200: "#CFEFBE",
                    300: "#B7E69D",
                    400: "#9CDD79",
                    500: "#85D559",
                    600: "#62BF30",
                    700: "#4B9325",
                    800: "#326219",
                    900: "#19310C",
                    950: "#0D1806"
                },

                "gray": {
                    50: "#F2F2F2",
                    100: "#E6E6E6",
                    200: "#CCCCCC",
                    300: "#B3B3B3",
                    400: "#999999",
                    500: "#808080",
                    600: "#666666",
                    700: "#4D4D4D",
                    800: "#333333",
                    900: "#1A1A1A",
                    950: "#0D0D0D"
                },
                "bg": "#0e0e0e",
                primary: {
                    DEFAULT: "hsl(var(--primary) / <alpha-value>)",
                    foreground: "hsl(var(--primary-foreground) / <alpha-value>)"
                },
                secondary: {
                    DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
                    foreground: "hsl(var(--secondary-foreground) / <alpha-value>)"
                },
                destructive: {
                    DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
                    foreground: "hsl(var(--destructive-foreground) / <alpha-value>)"
                },
                muted: {
                    DEFAULT: "hsl(var(--muted) / <alpha-value>)",
                    foreground: "hsl(var(--muted-foreground) / <alpha-value>)"
                },
                accent: {
                    DEFAULT: "hsl(var(--accent) / <alpha-value>)",
                    foreground: "hsl(var(--accent-foreground) / <alpha-value>)"
                },
                popover: {
                    DEFAULT: "hsl(var(--popover) / <alpha-value>)",
                    foreground: "hsl(var(--popover-foreground) / <alpha-value>)"
                },
                card: {
                    DEFAULT: "hsl(var(--card) / <alpha-value>)",
                    foreground: "hsl(var(--card-foreground) / <alpha-value>)"
                }
            },
            borderRadius: {
                lg: "var(--radius)",
                md: "calc(var(--radius) - 2px)",
                sm: "calc(var(--radius) - 4px)"
            },
            fontFamily: {
                sans: [...fontFamily.sans]
            }
        }
    },
};

export default config;
