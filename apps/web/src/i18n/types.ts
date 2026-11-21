export interface PluralForms {
  one: string;
  other: string;
}

export interface PageMessages {
  packageTitle: string;
  packagePage: string;
  allPackages: string;
  repository: string;
  download: string;
  aboutHeading: string;
  unity: string;
  license: string;
  packageAuthor: string;
  requires: string;
  addAllToVcc: string;
  addOnlyThisPackage: string;
  title: string;
  socialImageAlt: string;
  notFoundHeading: string;
  notFoundText: string;
  notFoundLink: string;
  description: string;
  author: string;
  lead: string;
  language: string;
  listingHeading: string;
  selectedLabel: string;
  urlLabel: string;
  addToVcc: string;
  howToHeading: string;
  howToSteps: readonly string[];
  howToHelp: string;
  packagesHeading: string;
  filterLabel: string;
  details: string;
  allVersions: string;
  retry: string;
  noScriptBefore: string;
  noScriptLink: string;
  noScriptAfter: string;
  footerNote: string;
  footerSource: string;
}

export interface ClientMessages {
  loading: string;
  loadError: string;
  empty: string;
  selected: PluralForms;
  noneSelected: string;
  selectAll: string;
  deselectAll: string;
  copy: string;
  copied: string;
  copyFailed: string;
  remove: string;
  noMatch: string;
  packages: PluralForms;
  versions: PluralForms;
  dependencies: PluralForms;
  prerelease: string;
  unity: string;
  author: string;
  requires: string;
  changelog: string;
  docs: string;
  releaseNotes: string;
  documentation: string;
}

export interface Messages {
  page: PageMessages;
  client: ClientMessages;
}
