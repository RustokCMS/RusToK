"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import Link from "next/link";

import { Button } from "@/components/ui/button";
import { locales } from "@/i18n";

type LoginFormProps = {
  locale: string;
};

type AuthResponse = {
  access_token: string;
};

const apiBaseUrl =
  process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:3000";

export default function LoginForm({ locale }: LoginFormProps) {
  const t = useTranslations("auth");
  const tErrors = useTranslations("errors");
  const router = useRouter();
  const [tenant, setTenant] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const onSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);

    if (!tenant || !email || !password) {
      setError(t("errorRequired"));
      return;
    }

    setIsLoading(true);

    try {
      const response = await fetch(`${apiBaseUrl}/api/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-Slug": tenant,
        },
        body: JSON.stringify({ email, password }),
      });

      if (!response.ok) {
        if (response.status === 401) {
          setError(tErrors("auth.invalid_credentials"));
        } else {
          setError(tErrors("http"));
        }
        return;
      }

      const payload = (await response.json()) as AuthResponse;
      document.cookie = `rustok-admin-token=${payload.access_token}; path=/`;
      document.cookie = `rustok-admin-tenant=${tenant}; path=/`;
      router.push(`/${locale}`);
    } catch {
      setError(tErrors("network"));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main className="min-h-screen bg-slate-50">
      <section className="mx-auto flex min-h-screen max-w-5xl flex-col gap-8 px-6 py-12 lg:flex-row lg:items-center">
        <aside className="flex-1">
          <span className="badge badge-outline">{t("badge")}</span>
          <h1 className="mt-3 text-3xl font-semibold text-slate-900">
            {t("heroTitle")}
          </h1>
          <p className="mt-3 text-sm text-slate-500">{t("heroSubtitle")}</p>
          <div className="mt-6 rounded-2xl border border-slate-200 bg-white p-5 text-sm text-slate-600">
            <p className="font-semibold text-slate-900">{t("heroListTitle")}</p>
            <p className="mt-2">{t("heroListSubtitle")}</p>
          </div>
        </aside>

        <div className="flex-1">
          <form
            className="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm"
            onSubmit={onSubmit}
          >
            <div>
              <h2 className="text-xl font-semibold text-slate-900">
                {t("title")}
              </h2>
              <p className="mt-2 text-sm text-slate-500">{t("subtitle")}</p>
            </div>

            <div className="mt-4 flex flex-wrap gap-2">
              {locales.map((targetLocale) => (
                <Button
                  asChild
                  key={targetLocale}
                  size="md"
                  variant={targetLocale === locale ? "primary" : "outline"}
                >
                  <Link
                    aria-current={targetLocale === locale ? "page" : undefined}
                    href={`/${targetLocale}/login`}
                  >
                    {targetLocale.toUpperCase()}
                  </Link>
                </Button>
              ))}
            </div>

            {error ? (
              <div className="mt-4 rounded-xl border border-rose-200 bg-rose-50 p-3 text-sm text-rose-600">
                {error}
              </div>
            ) : null}

            <div className="mt-5 grid gap-4">
              <label className="text-sm font-medium text-slate-700">
                {t("tenantLabel")}
                <input
                  className="mt-2 w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:outline-none"
                  name="tenant"
                  placeholder="demo"
                  value={tenant}
                  onChange={(event) => setTenant(event.target.value)}
                />
              </label>
              <label className="text-sm font-medium text-slate-700">
                {t("emailLabel")}
                <input
                  className="mt-2 w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:outline-none"
                  name="email"
                  placeholder="admin@rustok.io"
                  type="email"
                  value={email}
                  onChange={(event) => setEmail(event.target.value)}
                />
              </label>
              <label className="text-sm font-medium text-slate-700">
                {t("passwordLabel")}
                <input
                  className="mt-2 w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:outline-none"
                  name="password"
                  placeholder="••••••••"
                  type="password"
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                />
              </label>
            </div>

            <Button
              className="mt-6 w-full"
              size="md"
              type="submit"
              variant="primary"
              disabled={isLoading}
            >
              {isLoading ? `${t("submit")}…` : t("submit")}
            </Button>

            <p className="mt-4 text-xs text-slate-500">{t("footer")}</p>
          </form>
        </div>
      </section>
    </main>
  );
}
