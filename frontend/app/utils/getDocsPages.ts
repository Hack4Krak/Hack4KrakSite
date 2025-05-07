import { readdir } from 'node:fs/promises'
import { join } from 'node:path'

/**
 * This is a temporary solution for retrieving documentation pages.
 */
export async function getDocsPages() {
  const files = await readdir(join(process.cwd(), '/content/pages'))
  return files.map(file => `/docs/${file.replace('.md', '')}`)
}
