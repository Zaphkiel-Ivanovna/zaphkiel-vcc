import { toLocale } from '@/i18n/locales';
import type { AddScope } from '@/lib/analytics';
import { trackListingAdded } from '@/lib/analytics';

const SCOPES: readonly AddScope[] = ['all', 'single'];

function scopeOf(link: HTMLElement): AddScope | undefined {
  return SCOPES.find((scope) => scope === link.dataset.addScope);
}

export function setUpPackagePage(): void {
  const main = document.querySelector('main');
  const packageId = main instanceof HTMLElement ? main.dataset.packageId : undefined;
  const locale = toLocale(document.documentElement.lang);
  for (const link of document.querySelectorAll<HTMLElement>('[data-add-scope]')) {
    const scope = scopeOf(link);
    if (scope !== undefined && packageId !== undefined) {
      link.addEventListener('click', () => {
        trackListingAdded({
          locale,
          method: 'vcc_link',
          packages: [packageId],
          scope,
          source: 'package_page',
        });
      });
    }
  }
}
