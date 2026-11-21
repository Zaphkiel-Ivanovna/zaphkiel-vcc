import { parsePackages } from '@/lib/vpm/parse-packages';
import type { Packages } from '@/lib/vpm/types';
import { packageList, summary, toggleButton } from './elements';
import { renderPackageCard } from './package-card';
import { listingUrl, updateSelection } from './selection';

async function fetchPackages(): Promise<Packages> {
  const response = await fetch(listingUrl);
  if (!response.ok) {
    throw new Error(`The listing responded with HTTP ${response.status}`);
  }
  const listing: unknown = await response.json();
  return parsePackages(listing);
}

export async function loadPackages(): Promise<void> {
  const packages = await fetchPackages();
  const cards = Object.entries(packages)
    .map(([id, versions]) => renderPackageCard(id, versions))
    .filter((card) => card !== undefined);
  if (cards.length === 0) {
    summary.textContent = 'No packages published yet.';
    return;
  }
  packageList.append(...cards);
  toggleButton.hidden = false;
  updateSelection();
}
