import { textField } from '@/lib/vpm/manifest';
import type { Manifest, Versions } from '@/lib/vpm/types';
import { latestVersion } from '@/lib/vpm/versions';
import { detailRows } from './detail-rows';
import { createLink, requireElement } from './dom';
import { packageTemplate } from './elements';

interface CardIdentity {
  id: string;
  latest: string;
  manifest: Manifest;
}

function setText(card: DocumentFragment, selector: string, text: string): void {
  requireElement(card, selector, HTMLElement).textContent = text;
}

function fillIdentity(card: DocumentFragment, { id, latest, manifest }: CardIdentity): void {
  requireElement(card, 'input', HTMLInputElement).value = id;
  setText(card, '.name', textField(manifest, 'displayName') ?? id);
  setText(card, '.version', latest);
  setText(card, '.id', id);
  setText(card, '.description', textField(manifest, 'description') ?? '');
  requireElement(card, 'dl', HTMLElement).append(...detailRows(manifest));
}

function fillVersions(card: DocumentFragment, versions: Versions): void {
  const links = Object.entries(versions).map(([version, manifest]) => {
    const item = document.createElement('li');
    item.append(createLink(version, textField(manifest, 'url') ?? '#'));
    return item;
  });
  setText(card, 'summary', links.length === 1 ? '1 version' : `${links.length} versions`);
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
  fillVersions(card, versions);
  return card;
}
