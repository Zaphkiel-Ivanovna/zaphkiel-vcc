export function selectionUrl(
  listingUrl: string,
  selected: readonly string[],
  total: number,
): string {
  return selected.length === total ? listingUrl : `${listingUrl}?p=${selected.join(',')}`;
}

const WEB_PROTOCOLS = new Set(['http:', 'https:']);

export function webUrl(value: string | undefined): string | undefined {
  if (value === undefined || !URL.canParse(value)) {
    return undefined;
  }
  const url = new URL(value);
  return WEB_PROTOCOLS.has(url.protocol) ? url.href : undefined;
}

export function addRepoUrl(listingUrl: string): string {
  return `vcc://vpm/addRepo?url=${encodeURIComponent(listingUrl)}`;
}
