import tailwind from 'prettier-plugin-tailwindcss'

/** @type {import("prettier").Config} */
export default {
  plugins: [
    tailwind
  ],
  arrowParens: "avoid",
  semi: false,
  singleQuote: true,
  trailingComma: "all",
  jsxSingleQuote: false,
  useTabs: false,
  tabWidth: 2
}
