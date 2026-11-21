import { plural } from '@/i18n/format';
import { parsePackages } from '@/lib/vpm/parse-packages';
import type { Packages } from '@/lib/vpm/types';
import { filterField, packageList, stats, summary, toggleButton } from './elements';
import { locale, messages } from './messages';
import { renderPackageCard } from './package-card';
import { listingUrl, updateSelection } from './selection';

const FILTER_THRESHOLD = 5;

async function fetchPackages(): Promise<Packages> {
  const response = await fetch(listingUrl);
  if (!response.ok) {
    throw new Error(`The listing responded with HTTP ${response.status}`);
  }
  const listing: unknown = await response.json();
  return parsePackages(listing);
}

function showStats(packages: Packages): void {
  const versionCount = Object.values(packages).flatMap((versions) => Object.keys(versions)).length;
  const packageCount = Object.keys(packages).length;
  stats.textContent = [
    plural(locale, messages.packages, { count: packageCount }),
    plural(locale, messages.versions, { count: versionCount }),
  ].join(' · ');
  filterField.hidden = packageCount < FILTER_THRESHOLD;
}

export async function loadPackages(): Promise<void> {
  const packages = await fetchPackages();
  const cards = Object.entries(packages)
    .map(([id, versions]) => renderPackageCard(id, versions))
    .filter((card) => card !== undefined);
  if (cards.length === 0) {
    summary.textContent = messages.empty;
    return;
  }
  packageList.append(...cards);
  toggleButton.hidden = false;
  showStats(packages);
  updateSelection();
}
