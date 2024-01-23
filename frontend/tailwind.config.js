/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        'pixel': ['Minecraft', 'monospace']
      },
      colors: {
        'white': 'var(--white)',
        'offwhite': 'var(--offwhite)',
        'blue': 'var(--blue)',
        'dark-blue': '#2f3b5e',
        'background': 'var(--background)',
        'background-dimmed': 'var(--background-dimmed)'
      }
    },
  },
  darkMode: 'class'
}
