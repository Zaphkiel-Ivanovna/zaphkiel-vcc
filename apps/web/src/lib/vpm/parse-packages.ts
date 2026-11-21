import { isRecord } from './guards';
import type { Manifest, Packages, Versions } from './types';

function parseVersions(versions: Readonly<Record<string, unknown>>): Versions {
  const manifests = Object.entries(versions).filter((entry): entry is [string, Manifest] =>
    isRecord(entry[1]),
  );
  return Object.fromEntries(manifests);
}

export function parsePackages(listing: unknown): Packages {
  if (!isRecord(listing) || !isRecord(listing.packages)) {
    throw new TypeError('The listing has no packages object');
  }
  const packages: Record<string, Versions> = {};
  for (const [id, entry] of Object.entries(listing.packages)) {
    if (isRecord(entry) && isRecord(entry.versions)) {
      packages[id] = parseVersions(entry.versions);
    }
  }
  return packages;
}
