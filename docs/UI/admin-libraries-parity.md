# Паритет библиотек админок (Next.js ↔ Leptos)

Этот документ фиксирует **известные** соответствия библиотек между админками и станет базой для унификации стека.

## Контекст админок

- **Описание:** сравнение библиотек и паттернов между админками, чтобы стек был единым и прогнозируемым.
- **Стек:** Leptos CSR (`apps/admin`) + Next.js App Router (`apps/next-admin`), TailwindCSS, shadcn/ui, leptos-shadcn-ui, общие дизайн‑токены.
- **Ссылки:** [UI документы](./) • [UI parity](./ui-parity.md) • [IU библиотеки](../../IU/README.md)

## Известные аналоги (подтверждено в коде/доках)

| Категория | Next.js admin | Leptos admin | Примечание |
| --- | --- | --- | --- |
| CSS/дизайн-токены | TailwindCSS | TailwindCSS (`tailwind-rs`) | Next.js uses TailwindCSS; Leptos uses the WASM-first, type-safe `tailwind-rs`. |
| CSS pipeline | PostCSS + Autoprefixer | PostCSS + Autoprefixer | Одинаковая цепочка сборки стилей. |
| UI контракт | shadcn/ui | shadcn-style components | В документации зафиксирован единый shadcn‑style подход для обеих админок. |
| Каталог аналогов | N/A | N/A | Список библиотек и адаптеров: https://github.com/leptos-rs/awesome-leptos |
| Метаданные (Next.js) | next/metadata | leptos-next-metadata | https://github.com/cloud-shuttle/leptos-next-metadata |
| Data fetching | @tanstack/react-query | leptos-query | https://github.com/cloud-shuttle/leptos-query |
| i18n | next-intl | leptos_i18n | https://github.com/Baptistemontan/leptos_i18n |
| GraphQL client | graphql-request (или fetch) | reqwest + обёртка в `apps/admin/src/api/mod.rs` | Leptos не использует `async-graphql` на клиенте; запросы идут по HTTP к `/api/graphql`. |

## Требуют поиска и подтверждения

- Формы/валидация (Next.js: react-hook-form + zod).
- Таблицы (Next.js: @tanstack/react-table).
- Data fetching (Next.js: @tanstack/react-query).
- State (Next.js: zustand).

## Принципы выбора библиотек

Наш приоритет — **максимальное использование готовых библиотек** для реализации функционала.
При создании нового функционала **нужно сначала найти и предложить** соответствующую библиотеку/интеграцию.
Иначе в конце мы получим неработающий самопис, который сложно поддерживать и масштабировать.

## Что можно взять из AvoRed Leptos admin (cargo)

Если ориентироваться на базовые зависимости из `leptop-admin`, можно заимствовать следующие практики:

- **console_error_panic_hook** — улучшает сообщения об ошибках в браузерной консоли для WASM-приложений.
- **console_log** — логирование в консоль браузера с форматированием (удобно для dev).
- **leptos (csr)** — базовый CSR-стек (у нас уже используется).
- **leptos_router** — маршрутизация (у нас уже используется).
- **leptos_image** — оптимизированные компоненты для изображений (может пригодиться для медиа в админке).

## Где искать активно развиваемые Leptos-админки

Пока в репозитории нет закреплённого списка конкретных админок, поэтому ориентируемся на:

- **Каталог awesome‑leptos** — основной индекс актуальных проектов и библиотек; искать по ключевым словам *admin*, *dashboard*, *cms*, *panel*, *template*.
- **GitHub‑поиск** по `leptos` + `admin`/`dashboard`/`cms` и фильтром по активности (last updated).
- **Issue/PR‑активность**: смотреть частоту релизов, активные issues и свежие коммиты, чтобы не брать «мертвые» шаблоны.

Если нужно, можем завести отдельный список кандидатов в этом документе и вести его как «матрицу заимствований».
