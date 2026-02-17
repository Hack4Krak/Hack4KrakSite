import {
  createMarkdownParser,
  createShikiHighlighter,
  rehypeHighlight,
} from '@nuxtjs/mdc/runtime'
import { shikiBundledLangs, shikiBundledThemes, shikiTheme } from '~/utils/shiki'

export default function useMarkdownParser() {
  let parser: Awaited<ReturnType<typeof createMarkdownParser>>

  const parse = async (markdown: string) => {
    if (!parser) {
      parser = await createMarkdownParser({
        rehype: {
          plugins: {
            highlight: {
              instance: rehypeHighlight,
              options: {
                theme: shikiTheme,
                highlighter: createShikiHighlighter({
                  bundledThemes: shikiBundledThemes,
                  bundledLangs: shikiBundledLangs,
                }),
              },
            },
          },
        },
      })
    }
    return parser(markdown)
  }

  return parse
}
