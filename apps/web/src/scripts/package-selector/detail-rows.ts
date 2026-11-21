import { authorName, requirements, textField } from '@/lib/vpm/manifest';
import type { Manifest } from '@/lib/vpm/types';
import { webUrl } from '@/lib/vpm/urls';
import { createLink } from './dom';
import { messages } from './messages';

type DetailValue = string | HTMLAnchorElement | undefined;

function linkField(manifest: Manifest, key: string, text: string): HTMLAnchorElement | undefined {
  const href = webUrl(textField(manifest, key));
  return href === undefined ? undefined : createLink(text, href);
}

function detailValues(manifest: Manifest): [string, DetailValue][] {
  return [
    [messages.author, authorName(manifest)],
    [messages.requires, requirements(manifest).join('\n')],
    [messages.changelog, linkField(manifest, 'changelogUrl', messages.releaseNotes)],
    [messages.docs, linkField(manifest, 'documentationUrl', messages.documentation)],
  ];
}

export function detailRows(manifest: Manifest): HTMLElement[] {
  return detailValues(manifest).flatMap(([term, value]) => {
    if (value === undefined || value === '') {
      return [];
    }
    const title = document.createElement('dt');
    const description = document.createElement('dd');
    title.textContent = term;
    description.append(value);
    return [title, description];
  });
}
