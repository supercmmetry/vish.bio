/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: [
    './templates/**/*.html',
  ],
  future: {
    // Stops :hover styles sticking after a tap on touch devices.
    hoverOnlyWhenSupported: true,
  },
  theme: {
    extend: {
      // Tokens are CSS custom properties so the palette can be retuned in one place
      // and `/<alpha-value>` modifiers keep working (e.g. `bg-paper/60`).
      colors: {
        paper: 'rgb(var(--c-paper) / <alpha-value>)',
        'paper-sunk': 'rgb(var(--c-paper-sunk) / <alpha-value>)',
        ink: 'rgb(var(--c-ink) / <alpha-value>)',
        'ink-muted': 'rgb(var(--c-ink-muted) / <alpha-value>)',
        'ink-faint': 'rgb(var(--c-ink-faint) / <alpha-value>)',
        line: 'rgb(var(--c-line) / <alpha-value>)',
        accent: 'rgb(var(--c-accent) / <alpha-value>)',
        ember: 'rgb(var(--c-ember) / <alpha-value>)',
      },
      fontFamily: {
        display: ['"Instrument Serif"', 'Georgia', ...defaultTheme.fontFamily.serif],
        sans: ['"Instrument Sans Variable"', ...defaultTheme.fontFamily.sans],
        mono: ['"IBM Plex Mono"', ...defaultTheme.fontFamily.mono],
      },
      fontSize: {
        display: ['clamp(2.75rem, 1.2rem + 6vw, 5.5rem)', { lineHeight: '1.0', letterSpacing: '-0.03em' }],
        title: ['clamp(1.75rem, 1rem + 2.4vw, 2.75rem)', { lineHeight: '1.1', letterSpacing: '-0.02em' }],
        lede: ['clamp(1.0625rem, 0.95rem + 0.6vw, 1.375rem)', { lineHeight: '1.55', letterSpacing: '-0.005em' }],
        meta: ['0.75rem', { lineHeight: '1.4', letterSpacing: '0.16em' }],
      },
      maxWidth: {
        shell: '72rem',
        measure: '66ch',
      },
      transitionTimingFunction: {
        out: 'cubic-bezier(0.16, 1, 0.3, 1)',
      },
    },
  },
  plugins: [],
}
