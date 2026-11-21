import { copyButton, urlInput } from './elements';
import { messages } from './messages';
import { trackHomeListing } from './tracking';

const COPIED_LABEL_DURATION_MS = 1500;

function showLabel(label: string): void {
  copyButton.textContent = label;
  setTimeout(() => {
    copyButton.textContent = messages.copy;
  }, COPIED_LABEL_DURATION_MS);
}

export async function copyUrl(): Promise<void> {
  try {
    await navigator.clipboard.writeText(urlInput.value);
    showLabel(messages.copied);
    trackHomeListing('copy_url');
  } catch {
    showLabel(messages.copyFailed);
  }
}
