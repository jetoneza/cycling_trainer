import tailwind from 'tailwindcss'
import tailwindConfig from './tailwind.config.js'
import autoprefixer from 'autoprefixer'
import nesting from 'tailwindcss/nesting'

export default {
  plugins: [nesting, tailwind(tailwindConfig), autoprefixer],
}
