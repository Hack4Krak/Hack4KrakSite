import { readdir } from 'node:fs/promises'

export async function getDocsPages() {
  const files = await readdir(`${process.cwd()}/content/pages`)
  return files.map(file => `/docs/${file.replace('.md', '')}`)
}
