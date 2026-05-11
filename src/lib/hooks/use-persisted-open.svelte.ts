export function usePersistedOpen(key: string, defaultValue = true) {
  const stored = localStorage.getItem(key);
  let isOpen = $state(stored === null ? defaultValue : stored === "true");

  return {
    get isOpen() {
      return isOpen;
    },
    toggle() {
      isOpen = !isOpen;
      localStorage.setItem(key, String(isOpen));
    },
  };
}
