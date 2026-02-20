import type { MDCElement, MDCNode, MDCRoot } from '@nuxtjs/mdc'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import useMarkdownParser from '~/composables/useMarkdownParser'

function findNodes(node: MDCRoot | MDCNode | undefined, tag: string): MDCElement[] {
  const results: MDCElement[] = []
  if (!node)
    return results
  if (node.type === 'element' && node.tag === tag)
    results.push(node)
  if ('children' in node && Array.isArray(node.children)) {
    for (const child of node.children) {
      results.push(...findNodes(child, tag))
    }
  }
  return results
}

function textContent(node: MDCNode | undefined): string {
  if (!node)
    return ''
  if (node.type === 'text')
    return node.value ?? ''

  if ('children' in node && Array.isArray(node.children))
    return node.children.map(textContent).join('')
  return ''
}

describe('useMarkdownParser', () => {
  beforeEach(() => {
    vi.spyOn(console, 'warn').mockImplementation(() => {})
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('parses h1 with id attribute', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('# Hello World')

    const [h1] = findNodes(body, 'h1')
    expect(h1).toBeDefined()
    expect(h1!.props!.id).toBe('hello-world')
    expect(textContent(h1)).toBe('Hello World')
  })

  it('parses multiple heading levels with ids', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('# Title\n## Subtitle\n### Section')

    expect(findNodes(body, 'h1')[0]!.props!.id).toBe('title')
    expect(findNodes(body, 'h2')[0]!.props!.id).toBe('subtitle')
    expect(findNodes(body, 'h3')[0]!.props!.id).toBe('section')
  })

  it('parses inline code', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('Use `console.log()` here')

    const [code] = findNodes(body, 'code')
    expect(textContent(code)).toBe('console.log()')
  })

  it('parses code block into pre > code', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('```python\nprint("hello")\n```')

    const [pre] = findNodes(body, 'pre')
    expect(pre).toBeDefined()
    expect(findNodes(pre, 'code').length).toBeGreaterThanOrEqual(1)
  })

  it('applies language class on code block', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('```javascript\nconst x = 42;\n```')

    const [pre] = findNodes(body, 'pre')
    expect(pre?.props?.className).toContain('language-javascript')
  })

  it('applies shiki color styles on syntax tokens', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('```typescript\nconst msg: string = "hi";\n```')

    const spans = findNodes(body, 'span')
    const coloredSpans = spans.filter(n => /--shiki-default:#[0-9a-fA-F]/.test(n.props?.style ?? ''))
    expect(coloredSpans.length).toBeGreaterThan(0)
  })

  it('parses bold into strong node', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('Some **bold** text')

    const [strong] = findNodes(body, 'strong')
    expect(textContent(strong)).toBe('bold')
  })

  it('parses mixed content', async () => {
    const parse = useMarkdownParser()
    const { body } = await parse('# Title\n\nSome **bold** text.\n\n```python\nx = 1\n```')

    expect(findNodes(body, 'h1')[0]!.props!.id).toBe('title')
    expect(findNodes(body, 'strong')).toHaveLength(1)
    expect(findNodes(body, 'pre')).toHaveLength(1)
  })

  it('reuses parser across calls', async () => {
    const parse = useMarkdownParser()

    const r1 = await parse('# First')
    const r2 = await parse('# Second')

    expect(findNodes(r1.body, 'h1')[0]!.props!.id).toBe('first')
    expect(findNodes(r2.body, 'h1')[0]!.props!.id).toBe('second')
  })
})
