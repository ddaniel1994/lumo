export type Theme = "light" | "dark" | "system";

export function prefersDark(): boolean {
  if (typeof window === "undefined") return false;
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

export function applyTheme(root: HTMLElement, theme: Theme, explicitDark?: boolean) {
  const effectiveDark =
    theme === "dark"
      ? true
      : theme === "light"
        ? false
        : prefersDark();

  root.classList.toggle("dark", effectiveDark);
  if (explicitDark !== undefined) {
    root.dataset.explicitDark = String(explicitDark);
  }
}
