export function useGitInfo() {
  const runtimeConfig = useRuntimeConfig()

  const commitHash = runtimeConfig.public.gitCommitSha
  const branchName = runtimeConfig.public.gitBranch

  return {
    commitHash,
    branchName,
  }
}
