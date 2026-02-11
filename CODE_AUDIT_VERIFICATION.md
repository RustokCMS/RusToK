# RusToK — Проверка реализации после внешнего ревью

Дата: 2026-02-11

## Что проверено

Сверены:
- `IMPLEMENTATION_CHECKLIST.md`
- `PROGRESS_TRACKER.md`
- ключевые участки кода по Event versioning, Transactional publishing, test-utils и cache stampede protection.

## Подтверждено в коде

1. **Event schema versioning реализован**:
   - `EventEnvelope` содержит `event_type` и `schema_version`.
   - `DomainEvent` поддерживает `schema_version()`.
   - Outbox транспорт сохраняет `event_type` и `schema_version`.
   - Миграция `m20260211_000001_add_event_versioning` подключена в migrator.

2. **Transactional event publishing реализован**:
   - Есть `TransactionalEventBus`.
   - Есть `OutboxTransport::write_to_outbox`.
   - `NodeService` использует `publish_in_tx`.

3. **`rustok-test-utils` crate присутствует**:
   - Есть модули `db`, `events`, `fixtures`, `helpers`.

4. **Tenant cache stampede protection присутствует**:
   - В middleware есть `in_flight` и `get_or_load_with_coalescing`.
   - Добавлена метрика `coalesced_requests`.

## Найденные проблемы / несоответствия

1. **Чеклист не синхронизирован с трекером прогресса**.
   - В `PROGRESS_TRACKER.md` отмечены завершенными 4 critical issues (4/6).
   - В `IMPLEMENTATION_CHECKLIST.md` по факту отмечен только блок Tenant Cache Stampede Protection (3 отмеченных чекбокса в этом блоке), тогда как блоки Event Schema Versioning / Transaction Safety / Test Utilities остаются не отмеченными.

2. **`setup_test_db()` в `rustok-test-utils` не выполняет миграции, хотя документация функции утверждает обратное**.
   - В комментарии написано, что миграции должны запускаться.
   - В текущей реализации миграции не запускаются (оставлен комментарий-заглушка).
   - Это может давать ложное чувство готовности тестовой инфраструктуры.

3. **Тест cache stampede protection частично демонстрационный и не валидирует реальную middleware-инфраструктуру**.
   - Первый тест явно демонстрирует проблему «без коалесинга» и ожидает 100 запросов в БД.
   - Тест не подключает реальный `TenantCacheInfrastructure`, поэтому не доказывает работоспособность интеграционного сценария в приложении.

## Рекомендации (следующий шаг)

1. Синхронизировать `IMPLEMENTATION_CHECKLIST.md` с фактическим статусом из `PROGRESS_TRACKER.md`.
2. Для `setup_test_db()`:
   - либо реально запускать миграции,
   - либо изменить документацию функции на корректную (без заявления о миграциях).
3. Для cache stampede:
   - добавить минимум один интеграционный тест, который использует реальный middleware + test DB и проверяет снижение числа запросов к БД при конкурентной нагрузке.


## Статус после исправлений

- ✅ Синхронизирован `IMPLEMENTATION_CHECKLIST.md` для закрытых critical tasks (Issue #1-#4).
- ✅ Исправлена документация `setup_test_db()` (без ложного утверждения о запуске миграций).
- ✅ Уточнена документация и тесты по cache stampede: сохранены concurrency-тесты singleflight, но убраны чрезмерные claims об интеграционном покрытии.
