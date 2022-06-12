module.exports = {
  content: ['./src/**/*.{ts,tsx,html}'],
  plugins: [require('daisyui')],
  daisyui: {
    styled: true,
    themes: ['pastel'],
    rtl: false
  }
};
