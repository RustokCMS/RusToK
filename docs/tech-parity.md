# Technical parity plan (Leptos + Next.js)

This document tracks **technical parity** between the Leptos and Next.js implementations (admin + storefront).
It focuses on shared integration points like REST/GraphQL, routing, SEO, auth, and error handling. The goal is
feature parity **in behavior and integration**, independent of UI design.

## Scope

- Admin: `apps/admin` (Leptos CSR) + `apps/next-admin` (Next.js).
- Storefront: `apps/storefront` (Leptos SSR) + `apps/next-frontend` (Next.js).

## Parity goals

- Same API capabilities and data contracts in both stacks.
- Same navigation flows, route guards, and URL structures.
- Same SEO strategy and metadata coverage where applicable.
- Same auth/session behavior and error handling.

## Integration parity matrix

| Area | Next.js (admin) | Leptos (admin) | Next.js (storefront) | Leptos (storefront) | Notes |
| --- | --- | --- | --- | --- | --- |
| REST integration | ✅ | ⚠️ | ✅ | ⚠️ | Align auth headers, error mapping, pagination.
| GraphQL integration | ✅ | ⚠️ | ✅ | ⚠️ | Align query/mutation names and filtering semantics.
| Routing structure | ✅ | ⚠️ | ✅ | ⚠️ | Match paths, dynamic segments, and redirects.
| Auth/session | ✅ | ⚠️ | ✅ | ⚠️ | Match token storage, expiry, refresh, and logout.
| Error handling | ✅ | ⚠️ | ✅ | ⚠️ | Standardize error codes → UI states.
| i18n | ✅ | ⚠️ | ✅ | ⚠️ | Same locale list, fallback order, and URL strategy.
| Telemetry | ✅ | ⚠️ | ✅ | ⚠️ | Same events, context payload, and sampling.
| Feature flags | ✅ | ⚠️ | ✅ | ⚠️ | Same flag sources and gating logic.
| Forms/validation | ✅ | ⚠️ | ✅ | ⚠️ | Same constraints, messages, and server mapping.
| SSR/CSR strategy | ✅ | ⚠️ | ✅ | ⚠️ | Align hydration and caching strategy.
| SEO | ✅ | ⚠️ | ✅ | ⚠️ | Same metadata, structured data, and robots rules.

Legend: ✅ implemented, ⚠️ pending, ❌ not planned.

## Implementation tables

### REST integration

| Item | Contract | Next.js | Leptos | Notes |
| --- | --- | --- | --- | --- |
| Base URL | `API_BASE_URL` | ✅ | ⚠️ | Centralize via config module.
| Auth header | `Authorization: Bearer` | ✅ | ⚠️ | Reuse token key names across stacks.
| Error mapping | 4xx/5xx → UI | ✅ | ⚠️ | Align with shared error codes.
| Pagination | `page/limit` | ✅ | ⚠️ | Match defaults and boundaries.

### GraphQL integration

| Item | Contract | Next.js | Leptos | Notes |
| --- | --- | --- | --- | --- |
| Client | GraphQL HTTP | ✅ | ⚠️ | Standardize endpoint + headers.
| Query naming | `users`/`user` | ✅ | ⚠️ | Keep identical filters and sort fields.
| Error mapping | GraphQL errors | ✅ | ⚠️ | Match user-facing messages.

### Routing

| Item | Contract | Next.js | Leptos | Notes |
| --- | --- | --- | --- | --- |
| Admin base | `/admin` | ✅ | ⚠️ | Align base routing and redirect rules.
| Users list | `/users` | ✅ | ⚠️ | Match query params for filters.
| User details | `/users/:id` | ✅ | ⚠️ | Keep ID parsing and breadcrumbs.

### SEO (storefront focus)

| Item | Contract | Next.js | Leptos | Notes |
| --- | --- | --- | --- | --- |
| Title/description | `meta` | ✅ | ⚠️ | Same defaults and overrides.
| OpenGraph | `og:*` | ✅ | ⚠️ | Same image sizing and fallbacks.
| Canonical | `link[rel=canonical]` | ✅ | ⚠️ | Same rules for pagination.
| Robots | `meta[name=robots]` | ✅ | ⚠️ | Same noindex rules for private pages.

## Review process

1. Update this document whenever a new integration is added or changed.
2. Require dual implementation PRs (Next.js + Leptos) for parity-sensitive work.
3. Add a snapshot or checklist per release to ensure parity stays intact.

## Next steps

- Create a shared API contract reference (REST + GraphQL) and link it here.
- Define shared route maps for admin and storefront.
- Add a standard error code map and i18n keys list.
