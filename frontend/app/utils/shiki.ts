import type { BundledLanguage } from 'shiki'
import BashLang from '@shikijs/langs/bash'
import CLang from '@shikijs/langs/c'
import CppLang from '@shikijs/langs/cpp'
import CssLang from '@shikijs/langs/css'
import HtmlLang from '@shikijs/langs/html'
import JsLang from '@shikijs/langs/javascript'
import JsonLang from '@shikijs/langs/json'
import ShellLang from '@shikijs/langs/shell'
import MdLang from '@shikijs/langs/markdown'
import MdcLang from '@shikijs/langs/mdc'
import PythonLang from '@shikijs/langs/python'
import SqlLang from '@shikijs/langs/sql'
import TsLang from '@shikijs/langs/typescript'
import YamlLang from '@shikijs/langs/yaml'
import GithubDark from '@shikijs/themes/github-dark'

export const shikiTheme = 'github-dark' as const

export const shikiBundledThemes = { [shikiTheme]: GithubDark }

export const shikiBundledLangs = {
  python: PythonLang,
  py: PythonLang,
  javascript: JsLang,
  js: JsLang,
  typescript: TsLang,
  ts: TsLang,
  json: JsonLang,
  bash: BashLang,
  sh: BashLang,
  shell: ShellLang,
  html: HtmlLang,
  css: CssLang,
  c: CLang,
  cpp: CppLang,
  yaml: YamlLang,
  yml: YamlLang,
  markdown: MdLang,
  md: MdLang,
  mdc: MdcLang,
  sql: SqlLang,
}

export const shikiLangNames = [...new Set(Object.keys(shikiBundledLangs))] as BundledLanguage[]
