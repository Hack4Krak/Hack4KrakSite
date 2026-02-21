const checkSvg = `<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>`

// Attaches copy handlers to server-rendered ProsePre buttons (not hydrated in islands)
export default defineNuxtPlugin(() => {
  function activate(wrapper: Element) {
    if (wrapper.hasAttribute('data-copy-active'))
      return

    const pre = wrapper.querySelector('pre')
    const btn = wrapper.querySelector<HTMLButtonElement>('button[tabindex="-1"]')
    if (!pre || !btn)
      return

    wrapper.setAttribute('data-copy-active', '')
    const originalHTML = btn.innerHTML
    let timer: number

    btn.addEventListener('click', () => {
      navigator.clipboard.writeText(pre.textContent ?? '')
      clearTimeout(timer)
      btn.innerHTML = checkSvg
      timer = window.setTimeout(() => { btn.innerHTML = originalHTML }, 2000)
    })
  }

  function scan(root: Element | Document) {
    for (const pre of root.querySelectorAll('pre')) {
      if (pre.parentElement)
        activate(pre.parentElement)
    }
  }

  scan(document)

  new MutationObserver((mutations) => {
    for (const { addedNodes } of mutations) {
      for (const node of addedNodes) {
        if (node instanceof Element)
          scan(node)
      }
    }
  }).observe(document.body, { childList: true, subtree: true })
})
