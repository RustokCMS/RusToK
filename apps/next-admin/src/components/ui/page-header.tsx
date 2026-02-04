import { Breadcrumbs } from "./breadcrumbs";

type BreadcrumbItem = {
  label: string;
  href?: string;
};

type PageHeaderProps = {
  eyebrow?: string;
  title: string;
  subtitle?: string;
  actions?: React.ReactNode;
  breadcrumbs?: BreadcrumbItem[];
};

export function PageHeader({
  eyebrow,
  title,
  subtitle,
  actions,
  breadcrumbs,
}: PageHeaderProps) {
  return (
    <header className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
      <div className="space-y-2">
        {breadcrumbs && breadcrumbs.length ? (
          <Breadcrumbs items={breadcrumbs} />
        ) : null}
        {eyebrow ? (
          <p className="text-sm font-medium text-indigo-600">{eyebrow}</p>
        ) : null}
        <div>
          <h1 className="text-3xl font-semibold text-slate-900">{title}</h1>
          {subtitle ? (
            <p className="mt-2 max-w-2xl text-sm text-slate-500">{subtitle}</p>
          ) : null}
        </div>
      </div>
      {actions ? <div className="flex items-center gap-2">{actions}</div> : null}
    </header>
  );
}
