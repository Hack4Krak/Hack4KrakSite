import { readdir } from 'node:fs/promises'
import { join } from 'node:path'

export async function getDocsPages() {
  const files = await readdir(join(process.cwd(), '/content/pages'))
  return files.map(file => `/docs/${file.replace('.md', '')}`)
}
