/** @type {import('tailwindcss').Config} */
export default {
  plugins: [],
  theme: {
    extend: {
      colors: {
        'primary-100': '#e7f1fd',
        'primary-200': '#87b9f6',
        'primary-300': '#3e8ff1',
        'primary-400': '#0f73ee',
        'secondary-100': '#737373',
        'secondary-200': '#171717',
      },
    },
  },
  content: ['./index.html', './src/**/*.{svelte,js,ts}'], // for unused CSS
  variants: {
    extend: {},
  },
}
