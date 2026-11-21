import { format, plural } from '@/i18n/format';
import { requirements, textField } from '@/lib/vpm/manifest';
import type { Manifest } from '@/lib/vpm/types';
import { locale, messages } from './messages';

const SEPARATOR = ' · ';

export function packageMeta(manifest: Manifest, versionCount: number): string {
  const unity = textField(manifest, 'unity');
  const dependencyCount = requirements(manifest).length;
  const parts = [
    unity === undefined ? undefined : format(messages.unity, { version: unity }),
    textField(manifest, 'license'),
    plural(locale, messages.versions, { count: versionCount }),
    dependencyCount === 0
      ? undefined
      : plural(locale, messages.dependencies, { count: dependencyCount }),
  ];
  return parts.filter((part) => part !== undefined).join(SEPARATOR);
}
