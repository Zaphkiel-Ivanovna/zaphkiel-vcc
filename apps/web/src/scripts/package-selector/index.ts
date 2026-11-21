import { copyUrl } from './copy-url';
import {
  addLink,
  copyButton,
  filterInput,
  packageList,
  retryButton,
  selectedList,
  summary,
  toggleButton,
} from './elements';
import { applyFilter } from './filter';
import { loadPackages } from './load-packages';
import { messages } from './messages';
import { focusSelected, removeSelected } from './selected-list';
import { toggleAll, updateSelection } from './selection';
import { trackHomeListing } from './tracking';

async function start(): Promise<void> {
  retryButton.hidden = true;
  summary.textContent = messages.loading;
  try {
    await loadPackages();
  } catch {
    summary.textContent = messages.loadError;
    retryButton.hidden = false;
  }
}

packageList.addEventListener('change', updateSelection);
toggleButton.addEventListener('click', toggleAll);
filterInput.addEventListener('input', applyFilter);
copyButton.addEventListener('click', () => {
  void copyUrl();
});
addLink.addEventListener('click', () => {
  trackHomeListing('vcc_link');
});
retryButton.addEventListener('click', () => {
  void start();
});
selectedList.addEventListener('click', (event) => {
  if (removeSelected(event)) {
    updateSelection();
    focusSelected();
  }
});

await start();
