export function selectionUrl(
  listingUrl: string,
  selected: readonly string[],
  total: number,
): string {
  return selected.length === total ? listingUrl : `${listingUrl}?p=${selected.join(',')}`;
}

export function addRepoUrl(listingUrl: string): string {
  return `vcc://vpm/addRepo?url=${encodeURIComponent(listingUrl)}`;
}
