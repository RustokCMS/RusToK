# Мини-киты для админки и фронтенда

Цель: держать **минимальные** внутренние библиотеки для Leptos-приложений,
чтобы не дублировать код между админкой и фронтендом.

Охват приложений (обязательно учитываем при разработке, чтобы не плодить лишний самопис):
- `apps/admin` (Leptos CSR)
- `apps/storefront` (Leptos SSR)

## Уже есть

### 1) Leptos Auth
- Контракт для `/api/auth/*`
- Ключи хранения, маппинг ошибок
- Rust реализации

### 2) Leptos GraphQL
- Контракт для `/api/graphql`
- Заголовки tenant/auth
- Rust реализации

### 3) Leptos Hook Form
- Контракт для состояния формы (submitting/errors)
- Единая структура `field_errors`
- Rust реализации

### 4) Leptos Zod
- Контракт для ошибок валидации (zod‑style)
- Стабильная форма ошибок для UI
- Rust реализации

### 5) Leptos Struct Table
- Библиотека `leptos-struct-table` как основной слой таблиц для Leptos
- Типобезопасное описание колонок и рендера
- Rust реализация

### 6) Leptos Zustand
- Контракт для стор‑снимков и обновлений
- Лёгкий state-management слой для Leptos
- Rust реализации

## Что ещё можно закрывать мини-китами (по мере надобности)

### Формы и валидация
- **Leptos:** покрыто `leptos-hook-form` + `leptos-zod` (контракт FormState + errors)

### Таблицы
- **Leptos:** `leptos-struct-table`

### State
- **Leptos:** покрыто `leptos-zustand` (контракт store snapshots/updates)

### Утилиты для реактивности и браузерных API
- **Leptos:** `leptos-use` можно подключать точечно (debounce/throttle, media queries, storage, events).
- В базовый обязательный стек не входит: добавляем только при явной потребности экрана, чтобы не раздувать зависимости.

## Принцип расширения

1. Сначала фиксируем контракт и минимальный API.
2. Реализуем его в Rust.
3. Подключаем на 1–2 экранах, только затем масштабируем.

This is an alpha version and requires clarification. Be careful, there may be errors in the text. So that no one thinks that this is an immutable rule.
