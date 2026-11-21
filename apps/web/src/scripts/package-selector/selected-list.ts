import { format } from '@/i18n/format';
import { packageList, selectedList } from './elements';
import { messages } from './messages';

function packageName(box: HTMLInputElement): string {
  const row = box.closest('.package');
  const name = row instanceof HTMLElement ? row.querySelector('.name') : undefined;
  return name instanceof HTMLElement ? name.textContent : box.value;
}

function selectedItem(box: HTMLInputElement): HTMLLIElement {
  const name = packageName(box);
  const item = document.createElement('li');
  const remove = document.createElement('button');
  remove.type = 'button';
  remove.dataset.id = box.value;
  remove.textContent = '×';
  remove.setAttribute('aria-label', format(messages.remove, { name }));
  item.append(name, remove);
  return item;
}

export function renderSelected(boxes: readonly HTMLInputElement[]): void {
  selectedList.replaceChildren(...boxes.map((box) => selectedItem(box)));
}

export function removeSelected(event: Event): boolean {
  const { target } = event;
  if (!(target instanceof HTMLButtonElement) || target.dataset.id === undefined) {
    return false;
  }
  const box = packageList.querySelector(`input[value="${CSS.escape(target.dataset.id)}"]`);
  if (!(box instanceof HTMLInputElement)) {
    return false;
  }
  box.checked = false;
  return true;
}

export function focusSelected(): void {
  const next = selectedList.querySelector('button');
  if (next instanceof HTMLButtonElement) {
    next.focus();
  }
}
