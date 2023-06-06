import tailwind from 'tailwindcss'
import tailwindConfig from './tailwind.config.js'
import autoprefixer from 'autoprefixer'
import nested from 'postcss-nested'

export default {
  plugins: [tailwind(tailwindConfig), autoprefixer, nested],
}
