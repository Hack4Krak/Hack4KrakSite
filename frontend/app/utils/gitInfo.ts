import { execSync } from 'node:child_process'

export function getGitInfo() {
  const commitHash = execSync('git rev-parse --short HEAD').toString().trim()
  const branchName = execSync('git rev-parse --abbrev-ref HEAD').toString().trim()
  return { commitHash, branchName }
}
