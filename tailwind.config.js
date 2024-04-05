/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html", "./scripts/**/*.ts"],
  theme: {
    extend: {
      colors: {
        main: "#4F5356",
        button: "#FFB6EF",
        "button-hover": "#E6A3D7"
      },
    },
  },
  plugins: [],
};
