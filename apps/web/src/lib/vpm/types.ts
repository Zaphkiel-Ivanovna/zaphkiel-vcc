export type Manifest = Readonly<Record<string, unknown>>;
export type Versions = Readonly<Record<string, Manifest>>;
export type Packages = Readonly<Record<string, Versions>>;
