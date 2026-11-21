const PRERELEASE_MARKER = '-';

export function latestVersion(versions: readonly string[]): string | undefined {
  return versions.find((version) => !version.includes(PRERELEASE_MARKER)) ?? versions[0];
}

export function newerPrerelease(versions: readonly string[]): string | undefined {
  const [newest] = versions;
  return newest !== undefined &&
    newest.includes(PRERELEASE_MARKER) &&
    newest !== latestVersion(versions)
    ? newest
    : undefined;
}
