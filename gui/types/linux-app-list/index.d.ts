// Implemented in accordance with this specification:
// https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
declare module 'linux-app-list' {
  export interface AppData {
    absolutepath: string;
    Name: string;
    Icon?: string;
    Exec?: string;
    lang?: Record<string, { Name: string; Icon: string }>;
    Terminal?: string;
    NoDisplay?: string;
    Hidden?: string;
    OnlyShowIn?: string | string[];
    NotShowIn?: string | string[];
    TryExec?: string;
  }

  export interface AppList {
    list(): string[];
    data(app: string): AppData;
  }

  export default function indexItems(): AppList;
}
