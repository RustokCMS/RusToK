import { getLocale } from "next-intl/server";

import LoginForm from "./view";

export default async function LoginPage() {
  const locale = await getLocale();

  return <LoginForm locale={locale} />;
}
