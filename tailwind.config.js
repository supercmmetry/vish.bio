/** @type {import('tailwindcss').Config} */


const colors = require('tailwindcss/colors')

module.exports = {
  content: [
    "./templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        primary: colors['slate']['800'],
        secondary: colors['indigo']['200'],
        foreground: colors.white,
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}

