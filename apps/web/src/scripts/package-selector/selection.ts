import { addRepoUrl, selectionUrl } from '@/lib/vpm/urls';
import { addLink, copyButton, packageList, summary, toggleButton, urlInput } from './elements';

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

export function updateSelection(): void {
  const boxes = checkboxes();
  const selected = boxes.filter((box) => box.checked).map((box) => box.value);
  const hasSelection = selected.length > 0;

  showUrl(hasSelection ? selectionUrl(listingUrl, selected, boxes.length) : undefined);
  summary.textContent = `${selected.length} of ${boxes.length} packages selected`;
  toggleButton.textContent = hasSelection ? 'Deselect all' : 'Select all';
}

export function toggleAll(): void {
  const boxes = checkboxes();
  const shouldCheck = boxes.every((box) => !box.checked);
  for (const box of boxes) {
    box.checked = shouldCheck;
  }
  updateSelection();
}
