import type { AddMethod } from '@/lib/analytics';
import { trackListingAdded } from '@/lib/analytics';
import { locale } from './messages';
import { selectedPackages } from './selection';

export function trackHomeListing(method: AddMethod): void {
  const { ids, total } = selectedPackages();
  trackListingAdded({
    locale,
    method,
    packages: ids,
    scope: ids.length === total ? 'all' : 'selection',
    source: 'home',
    totalPackages: total,
  });
}
