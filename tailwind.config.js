/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        fm: {
          bg: "#08111f",
          panel: "#101d2f",
          panel2: "#172840",
          border: "#263a55",
          accent: "#22d3c5",
          accent2: "#60a5fa",
          warn: "#fbbf24",
          danger: "#fb7185",
          text: "#f8fafc",
          dim: "#94a3b8"
        }
      }
    }
  },
  plugins: []
};
