import plugin from 'tailwindcss/plugin'
/** @type {import('tailwindcss').Config} */
export const content = ['./src/**/*', './node_modules/shelf-ui/*.*']
export const theme = {
  extend: {},
}
export const plugins = [
  plugin(({ addVariant }) => {
    addVariant('em', ({ container }) => {
      container.walkRules(rule => {
        rule.selector = `.em\\:${rule.selector.slice(1)}`
        rule.walkDecls(decl => {
          decl.value = decl.value.replace('rem', 'em')
        })
      })
    })
  }),
]
