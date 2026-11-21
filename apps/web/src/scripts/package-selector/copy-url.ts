import { copyButton, urlInput } from './elements';

const COPIED_LABEL_DURATION_MS = 1500;
const DEFAULT_LABEL = 'Copy';
const COPIED_LABEL = 'Copied';
const FAILED_LABEL = 'Copy failed';

function showLabel(label: string): void {
  copyButton.textContent = label;
  setTimeout(() => {
    copyButton.textContent = DEFAULT_LABEL;
  }, COPIED_LABEL_DURATION_MS);
}

export async function copyUrl(): Promise<void> {
  try {
    await navigator.clipboard.writeText(urlInput.value);
    showLabel(COPIED_LABEL);
  } catch {
    showLabel(FAILED_LABEL);
  }
}
