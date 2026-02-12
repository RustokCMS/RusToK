# Admin ↔ Server Connection Quickstart

Практическая инструкция для сценария:

- backend (Loco server) установлен в одной папке;
- admin UI установлен в другой папке;
- нужно быстро и без боли подключить админку к серверу.

---

## 1) Самый простой и рекомендуемый вариант: **один домен через reverse proxy**

Идея: браузер открывает админку и отправляет API-запросы на **тот же origin**.
Прокси уже пересылает `/api/*` в backend.

Это избавляет от CORS-проблем и сложной ручной настройки.

### 1.1 Пример структуры на сервере

```text
/opt/rustok/
  server/      # Loco backend
  admin/       # собранная админка (static/SSR)
```

### 1.2 Nginx конфиг (базовый)

```nginx
server {
    listen 80;
    server_name admin.example.com;

    # Админка (статика)
    root /opt/rustok/admin;
    index index.html;

    location / {
        try_files $uri /index.html;
    }

    # API -> backend
    location /api/ {
        proxy_pass http://127.0.0.1:5150/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 1.3 Что указывать в админке

В этом режиме обычно достаточно:

- `GRAPHQL_ENDPOINT=/api/graphql`
- `AUTH_BASE_URL=/api/auth`

`api_base_url` можно не задавать, если UI и API идут через один host.

---

## 2) Вариант "разные домены": admin отдельно, API отдельно

Пример:

- admin: `https://admin.example.com`
- backend: `https://api.example.com`

Тогда в конфиге админки нужно явно задать:

- `api_base_url=https://api.example.com`
- `graphql_endpoint=/api/graphql`
- `auth_base_url=/api/auth`

И на backend обязательно:

- настроить CORS для `https://admin.example.com`;
- разрешить заголовки `Authorization`, `X-Tenant-Slug`;
- разрешить `credentials`, если используете cookie-based auth.

---

## 3) Минимальный runtime config (что должно быть у админки)

Сводка полей:

- `api_base_url` — базовый URL backend (опционально при same-origin)
- `graphql_endpoint` — обычно `/api/graphql`
- `auth_base_url` — обычно `/api/auth`
- `tenant_slug` — опционально, если нужно предзаполнить tenant
- `app_env` — `local` / `staging` / `production`

---

## 4) Что сделать "тупому пользователю" пошагово (чеклист)

1. Поднять backend и убедиться, что доступен `GET /api/health` (или ваш health endpoint).
2. Положить сборку админки в папку веб-сервера.
3. Включить reverse proxy `/api/* -> backend`.
4. Открыть админку в браузере.
5. Проверить вход (login) и запрос `me`.
6. Проверить GraphQL-запрос к `/api/graphql`.

Если login проходит, `me` работает, и GraphQL отвечает — подключение выполнено корректно.

---

## 5) Диагностика (если не работает)

### Симптом: `401 Unauthorized`

Проверьте:

- отправляется ли `Authorization: Bearer <token>`;
- отправляется ли `X-Tenant-Slug` (если обязателен);
- не протух ли токен.

### Симптом: CORS error

Почти всегда это из-за cross-origin режима.

Быстрый фикс: перейти на same-origin через reverse proxy.

### Симптом: `404 /api/graphql`

Проверьте:

- что прокси действительно пробрасывает `/api/`;
- что backend слушает правильный порт;
- что endpoint именно `/api/graphql`.

---

## 6) Рекомендация для production

Используйте **same-origin схему** (admin + `/api/*` за одним доменом).

Плюсы:

- проще эксплуатация;
- меньше проблем с CORS/cookies;
- предсказуемая конфигурация для пользователей.
