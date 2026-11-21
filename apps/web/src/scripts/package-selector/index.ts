import { copyUrl } from './copy-url';
import { copyButton, packageList, summary, toggleButton } from './elements';
import { loadPackages } from './load-packages';
import { toggleAll, updateSelection } from './selection';

packageList.addEventListener('change', updateSelection);
toggleButton.addEventListener('click', toggleAll);
copyButton.addEventListener('click', () => {
  void copyUrl();
});

try {
  await loadPackages();
} catch {
  summary.textContent = 'Could not load the package list. Try again in a moment.';
}
