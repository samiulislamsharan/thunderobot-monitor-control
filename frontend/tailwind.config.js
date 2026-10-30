/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        cyber: {
          bg: '#080c14',
          card: '#0f172a',
          cardHover: '#1e293b',
          border: '#1e293b',
          borderGlow: '#00f0ff',
          neonCyan: '#00f0ff',
          neonPurple: '#a855f7',
          neonAmber: '#f59e0b',
          neonGreen: '#10b981',
          neonRed: '#ef4444'
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace']
      },
      boxShadow: {
        'cyan-glow': '0 0 20px -3px rgba(0, 240, 255, 0.25)',
        'purple-glow': '0 0 20px -3px rgba(168, 85, 247, 0.25)',
      }
    },
  },
  plugins: [],
}
