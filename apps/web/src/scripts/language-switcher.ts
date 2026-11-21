function closeOnOutsideClick(switcher: HTMLDetailsElement, { target }: Event): void {
  if (target instanceof Node && !switcher.contains(target)) {
    switcher.open = false;
  }
}

function closeOnEscape(switcher: HTMLDetailsElement, { key }: KeyboardEvent): void {
  if (key !== 'Escape' || !switcher.open) {
    return;
  }
  switcher.open = false;
  const summary = switcher.querySelector('summary');
  if (summary instanceof HTMLElement) {
    summary.focus();
  }
}

export function setUpLanguageSwitcher(): void {
  const switcher = document.querySelector('[data-language-switcher]');
  if (!(switcher instanceof HTMLDetailsElement)) {
    return;
  }
  document.addEventListener('click', (event) => {
    closeOnOutsideClick(switcher, event);
  });
  document.addEventListener('keydown', (event) => {
    closeOnEscape(switcher, event);
  });
}
