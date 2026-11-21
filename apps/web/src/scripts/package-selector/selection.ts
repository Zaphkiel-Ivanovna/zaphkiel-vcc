import { plural } from '@/i18n/format';
import { addRepoUrl, selectionUrl } from '@/lib/vpm/urls';
import { addLink, copyButton, packageList, summary, toggleButton, urlInput } from './elements';
import { locale, messages } from './messages';
import { renderSelected } from './selected-list';

export const listingUrl = new URL('/index.json', location.origin).href;

function checkboxes(): HTMLInputElement[] {
  return [...packageList.querySelectorAll<HTMLInputElement>('input[type=checkbox]')];
}

function showUrl(url: string | undefined): void {
  urlInput.value = url ?? '';
  copyButton.disabled = url === undefined;
  if (url === undefined) {
    addLink.removeAttribute('href');
  } else {
    addLink.href = addRepoUrl(url);
  }
}

export function selectedPackages(): { ids: string[]; total: number } {
  const boxes = checkboxes();
  const ids = boxes.filter((box) => box.checked).map((box) => box.value);
  return { ids, total: boxes.length };
}

export function updateSelection(): void {
  const boxes = checkboxes();
  const selected = boxes.filter((box) => box.checked);
  const hasSelection = selected.length > 0;
  const ids = selected.map((box) => box.value);

  showUrl(hasSelection ? selectionUrl(listingUrl, ids, boxes.length) : undefined);
  renderSelected(selected);
  summary.textContent = hasSelection
    ? plural(locale, messages.selected, { count: selected.length, total: boxes.length })
    : messages.noneSelected;
  toggleButton.textContent = hasSelection ? messages.deselectAll : messages.selectAll;
}

export function toggleAll(): void {
  const boxes = checkboxes();
  const shouldCheck = boxes.every((box) => !box.checked);
  for (const box of boxes) {
    box.checked = shouldCheck;
  }
  updateSelection();
}
