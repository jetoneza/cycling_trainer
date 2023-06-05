/** @type {import('tailwindcss').Config} */
export default {
  plugins: [],
  theme: {
    extend: {
      colors: {
        primary: '#00A3A3',
        secondary: '#171717',
        'secondary-500': '#737373',
      },
    },
  },
  content: ['./index.html', './src/**/*.{svelte,js,ts}'], // for unused CSS
  variants: {
    extend: {},
  },
}
