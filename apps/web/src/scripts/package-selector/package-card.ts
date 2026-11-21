import { format } from '@/i18n/format';
import { textField } from '@/lib/vpm/manifest';
import type { Manifest, Versions } from '@/lib/vpm/types';
import { webUrl } from '@/lib/vpm/urls';
import { latestVersion, newerPrerelease } from '@/lib/vpm/versions';
import { detailRows } from './detail-rows';
import { createLink, requireElement } from './dom';
import { packageTemplate } from './elements';
import { homePath, messages } from './messages';
import { packageMeta } from './package-meta';

interface CardIdentity {
  id: string;
  latest: string;
  manifest: Manifest;
}

function setText(card: DocumentFragment, selector: string, text: string): void {
  requireElement(card, selector, HTMLElement).textContent = text;
}

function fillIdentity(card: DocumentFragment, { id, latest, manifest }: CardIdentity): void {
  const name = textField(manifest, 'displayName') ?? id;
  const description = textField(manifest, 'description') ?? '';
  requireElement(card, 'input', HTMLInputElement).value = id;
  requireElement(card, '.package', HTMLElement).dataset.search =
    `${name} ${id} ${description}`.toLowerCase();
  requireElement(card, '.page-link', HTMLAnchorElement).href =
    `${homePath}packages/${encodeURIComponent(id)}/`;
  setText(card, '.name', name);
  setText(card, '.version', latest);
  setText(card, '.id', id);
  setText(card, '.description', description);
  requireElement(card, 'dl', HTMLElement).append(...detailRows(manifest));
}

function fillVersions(card: DocumentFragment, versions: Versions, manifest: Manifest): void {
  const names = Object.keys(versions);
  const prerelease = newerPrerelease(names);
  const links = Object.entries(versions).map(([version, release]) => {
    const item = document.createElement('li');
    const downloadUrl = webUrl(textField(release, 'url'));
    item.append(createLink(version, downloadUrl));
    return item;
  });
  setText(card, '.meta', packageMeta(manifest, names.length));
  setText(
    card,
    '.prerelease',
    prerelease === undefined ? '' : format(messages.prerelease, { version: prerelease }),
  );
  requireElement(card, '.versions', HTMLElement).append(...links);
}

export function renderPackageCard(id: string, versions: Versions): Node | undefined {
  const latest = latestVersion(Object.keys(versions));
  const manifest = latest === undefined ? undefined : versions[latest];
  const card = packageTemplate.content.cloneNode(true);
  if (latest === undefined || manifest === undefined || !(card instanceof DocumentFragment)) {
    return undefined;
  }
  fillIdentity(card, { id, latest, manifest });
  fillVersions(card, versions, manifest);
  return card;
}
