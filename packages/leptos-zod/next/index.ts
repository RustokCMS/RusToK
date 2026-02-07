export type ZodIssue = {
  path: Array<string | number>;
  message: string;
};

export type ZodError = {
  issues: ZodIssue[];
};

export const mapZodError = (error: ZodError): ZodError => ({
  issues: error.issues.map((issue) => ({
    path: issue.path,
    message: issue.message,
  })),
});
