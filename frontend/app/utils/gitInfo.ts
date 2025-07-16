export function getGitInfo() {
  const commitHash = Bun.spawnSync(['git', 'rev-parse', '--short', 'HEAD']).stdout.toString().trim()
  const branchName = Bun.spawnSync(['git', 'rev-parse', '--abbrev-ref', 'HEAD']).stdout.toString().trim()
  return { commitHash, branchName }
}
