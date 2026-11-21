export type AddMethod = 'vcc_link' | 'copy_url';
export type AddScope = 'all' | 'selection' | 'single';
export type AddSource = 'home' | 'package_page';

interface ListingAdded {
  locale: string;
  method: AddMethod;
  packages: readonly string[];
  scope: AddScope;
  source: AddSource;
  totalPackages?: number;
}

type Properties = Record<string, string | number>;

interface Rybbit {
  event: (name: string, properties: Properties) => void;
}

function isRybbit(value: unknown): value is Rybbit {
  return (
    typeof value === 'object' &&
    value !== null &&
    'event' in value &&
    typeof value.event === 'function'
  );
}

function send(name: string, properties: Properties): void {
  const rybbit: unknown = Reflect.get(globalThis, 'rybbit');
  if (isRybbit(rybbit)) {
    rybbit.event(name, properties);
  }
}

export function trackListingAdded(added: ListingAdded): void {
  const { locale, method, packages, scope, source, totalPackages } = added;
  const counts: Properties =
    totalPackages === undefined
      ? {}
      : { package_count: packages.length, total_packages: totalPackages };
  send('listing_added', Object.assign(counts, { locale, method, scope, source }));
  if (scope === 'all') {
    return;
  }
  for (const id of packages) {
    send('package_added', { locale, method, package: id, source });
  }
}
