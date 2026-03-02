function removeDataTestAttrs(node: { type: number, props?: { type: number, name: string }[] }) {
  if (node.type === 1 && node.props) {
    node.props = node.props.filter(
      prop => !(prop.type === 6 && prop.name === 'data-testid'),
    )
  }
}

export function getNodeTransforms() {
  return process.env.NODE_ENV === 'production'
    ? [removeDataTestAttrs]
    : []
}
