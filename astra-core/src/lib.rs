/*
export function createSignal<T>(
  value: T,
  options?: { equals?: false | ((prev: T, next: T) => boolean); name?: string }
): [get: () => T, set: (v: (T extends Function ? never : T) | ((prev: T) => T)) => T] {
  return [
    () => value as T,
    v => {
      return (value = typeof v === "function" ? (v as (prev: T) => T)(value) : v);
    }
  ];
}
 */
