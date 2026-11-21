import { authorName, requirements, textField } from '@/lib/vpm/manifest';
import type { Manifest } from '@/lib/vpm/types';
import { createLink } from './dom';

type DetailValue = string | HTMLAnchorElement | undefined;

function linkField(manifest: Manifest, key: string, text: string): HTMLAnchorElement | undefined {
  const href = textField(manifest, key);
  return href === undefined ? undefined : createLink(text, href);
}

function detailValues(manifest: Manifest): [string, DetailValue][] {
  return [
    ['Unity', textField(manifest, 'unity')],
    ['Author', authorName(manifest)],
    ['License', textField(manifest, 'license')],
    ['Requires', requirements(manifest).join('\n')],
    ['Changelog', linkField(manifest, 'changelogUrl', 'Release notes')],
    ['Docs', linkField(manifest, 'documentationUrl', 'Documentation')],
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
