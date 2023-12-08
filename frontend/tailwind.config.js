/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        'pixel': ['Minecraft', 'monospace']
      },
      colors: {
        'create-blue': '#7695EC',
        'create-blue-dark': '#2f3b5e',
        'minecraft-ui': {
          'light': '#d9d9db',
          'dark': '#101015'
        }
      }
    },
  },
  plugins: [],
  darkMode: 'class'
}
