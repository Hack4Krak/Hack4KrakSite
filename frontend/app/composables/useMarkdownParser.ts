import {
  createMarkdownParser,
  rehypeHighlight,
  createShikiHighlighter,
} from '@nuxtjs/mdc/runtime'
import GithubDark from '@shikijs/themes/github-dark'
import PythonLang from '@shikijs/langs/python'
import JsLang from '@shikijs/langs/javascript'
import TsLang from '@shikijs/langs/typescript'
import JsonLang from '@shikijs/langs/json'
import BashLang from '@shikijs/langs/bash'
import HtmlLang from '@shikijs/langs/html'
import CssLang from '@shikijs/langs/css'
import CLang from '@shikijs/langs/c'
import CppLang from '@shikijs/langs/cpp'
import YamlLang from '@shikijs/langs/yaml'
import MdLang from '@shikijs/langs/markdown'
import SqlLang from '@shikijs/langs/sql'

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
                theme: 'github-dark',
                highlighter: createShikiHighlighter({
                  bundledThemes: {
                    'github-dark': GithubDark,
                  },
                  bundledLangs: {
                    python: PythonLang,
                    py: PythonLang,
                    js: JsLang,
                    javascript: JsLang,
                    ts: TsLang,
                    typescript: TsLang,
                    json: JsonLang,
                    bash: BashLang,
                    sh: BashLang,
                    shell: BashLang,
                    html: HtmlLang,
                    css: CssLang,
                    c: CLang,
                    cpp: CppLang,
                    yaml: YamlLang,
                    yml: YamlLang,
                    md: MdLang,
                    markdown: MdLang,
                    sql: SqlLang,
                  },
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
