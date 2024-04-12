/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        primary: {
          "50": "#e8edf2",
          "100": "#d1dbe4",
          "200": "#a3b7ca",
          "300": "#7592af",
          "400": "#476e95",
          "500": "#194a7a",
          "600": "#143b62",
          "700": "#0f2c49",
          "800": "#0a1e31",
          "900": "#050f18",
        },
      },
    },
  },
  plugins: [],
};
