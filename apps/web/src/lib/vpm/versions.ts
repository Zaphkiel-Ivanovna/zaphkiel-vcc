const PRERELEASE_MARKER = '-';

export function latestVersion(versions: readonly string[]): string | undefined {
  return versions.find((version) => !version.includes(PRERELEASE_MARKER)) ?? versions[0];
}
