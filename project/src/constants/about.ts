export type AboutLinkKey = "website" | "github" | "releaseNotes" | "support";

export interface AboutLink {
  key: AboutLinkKey;
  url: string;
}

// 网址后续补充；为空时按钮保持禁用。
export const ABOUT_LINKS: readonly AboutLink[] = [
  {key: "website", url: ""},
  {key: "github", url: ""},
  {key: "releaseNotes", url: ""},
  {key: "support", url: ""},
];
