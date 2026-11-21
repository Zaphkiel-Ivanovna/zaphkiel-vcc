import { isRecord } from './guards';
import type { Manifest } from './types';

const DEPENDENCY_FIELDS = ['dependencies', 'vpmDependencies'] as const;

export function textField(manifest: Manifest, key: string): string | undefined {
  const value = manifest[key];
  return typeof value === 'string' && value !== '' ? value : undefined;
}

export function authorName(manifest: Manifest): string | undefined {
  const { author } = manifest;
  return isRecord(author) ? textField(author, 'name') : textField(manifest, 'author');
}

function dependencyRanges(dependencies: unknown): [string, string][] {
  if (!isRecord(dependencies)) {
    return [];
  }
  return Object.entries(dependencies).filter(
    (entry): entry is [string, string] => typeof entry[1] === 'string',
  );
}

export function requirements(manifest: Manifest): string[] {
  const ranges = new Map(DEPENDENCY_FIELDS.flatMap((field) => dependencyRanges(manifest[field])));
  return [...ranges].map(([name, range]) => `${name} ${range}`);
}
