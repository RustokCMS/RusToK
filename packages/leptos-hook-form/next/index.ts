export type FieldError = {
  field: string;
  message: string;
};

export type FormStateShape = {
  isSubmitting: boolean;
  formError?: string | null;
  fieldErrors: FieldError[];
};

export class FormState {
  static idle(): FormStateShape {
    return { isSubmitting: false, formError: null, fieldErrors: [] };
  }

  static submitting(): FormStateShape {
    return { isSubmitting: true, formError: null, fieldErrors: [] };
  }

  static withFormError(message: string): FormStateShape {
    return { isSubmitting: false, formError: message, fieldErrors: [] };
  }

  static withFieldErrors(fieldErrors: FieldError[]): FormStateShape {
    return { isSubmitting: false, formError: null, fieldErrors };
  }
}

export type ValidationIssue = {
  path: Array<string | number>;
  message: string;
};

export const issuesToFieldErrors = (
  issues: ValidationIssue[],
): FieldError[] =>
  issues.map((issue) => ({
    field: issue.path.map(String).join("."),
    message: issue.message,
  }));
