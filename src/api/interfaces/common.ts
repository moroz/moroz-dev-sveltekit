export type ID = string | number;

export interface MutationResult<T, E = Partial<T>> {
  success: boolean;
  errors: Record<keyof E, string[]>;
  data: T | null;
}
