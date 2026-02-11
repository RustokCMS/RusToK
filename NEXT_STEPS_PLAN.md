# RusToK - Next Steps Plan
## After Code Audit (February 11, 2026)

> **Context**: Code audit completed, backend compiles, frontend temporarily disabled, test infrastructure needs fixes.

---

## 🎯 Immediate Priorities (This Week)

### 1. Fix Test Infrastructure ⏱️ 2-3 hours
**Priority**: CRITICAL  
**Blocker**: Test suite cannot run

**Tasks**:
- [ ] Review current `DomainEvent` enum variants in `rustok-core/src/events/types.rs`
- [ ] Clean up `rustok-test-utils/src/events.rs`:
  - Remove references to deleted event variants (Shipment*, Payment*, Inventory*, etc.)
  - Ensure all match arms cover current variants only
  - Add `_ => ` catch-all for forward compatibility
- [ ] Fix `UserRole` clone issue in `rustok-test-utils/src/fixtures.rs:163`
- [ ] Verify test suite compiles: `cargo test --workspace --no-run`
- [ ] Run existing tests: `cargo test --workspace`
- [ ] Document any newly discovered test failures

**Success Criteria**:
- ✅ `rustok-test-utils` compiles without errors
- ✅ Test suite runs (даже если некоторые тесты падают)
- ✅ Coverage report генерируется

---

### 2. Resolve Frontend Build Issue ⏱️ 3-4 hours
**Priority**: HIGH  
**Blocker**: Cannot build admin/storefront apps

**Investigation Steps**:
1. [ ] Check for `tailwind-rs` updates:
   ```bash
   cargo search tailwind-rs
   ```

2. [ ] Check `parcel_css` compatibility:
   - Current: v1.0.0-alpha.32 (broken)
   - Required: version compatible with parcel_selectors 0.24.9+

3. [ ] Evaluate alternatives:

**Option A: Update Dependencies**
- [ ] Try updating `tailwind-rs` in `Cargo.toml`
- [ ] Test if newer version fixes `parcel_css` issue
- [ ] If yes: re-enable admin/storefront in workspace

**Option B: Fork and Patch**
- [ ] Fork `tailwind-rs` repository
- [ ] Update `parcel_css` dependency to newer alpha/beta
- [ ] Test compilation
- [ ] Use forked version via git dependency

**Option C: Alternative Approach**
- [ ] Research Leptos-specific Tailwind solutions
- [ ] Consider `stylex` or other CSS-in-Rust approaches
- [ ] Evaluate build-time Tailwind CLI integration
- [ ] Document trade-offs

**Success Criteria**:
- ✅ Frontend apps compile OR
- ✅ Clear path forward documented with timeline

---

## 📋 Short-term Tasks (Next 1-2 Weeks)

### 3. Continue Phase 3 Implementation ⏱️ 10-12 days
**Priority**: MEDIUM  
**Context**: Phase 1 & 2 complete (11/22 tasks done, 50% overall)

**Reference**: `PROJECT_STATUS.md` - Phase 3: Production Ready

#### 3.1 Error Handling Standardization (2 days)
- [ ] Audit all `.unwrap()` calls (use `rg "\.unwrap\(\)"`)
- [ ] Replace with proper error handling + context
- [ ] Use `thiserror` in library crates
- [ ] Use `anyhow` in application crates
- [ ] Document error hierarchy in `docs/ERROR_HANDLING.md`

#### 3.2 API Documentation (2 days)
- [ ] Add OpenAPI examples for all REST endpoints
- [ ] Document GraphQL schema with examples
- [ ] Create error response catalog
- [ ] Add authentication flow documentation
- [ ] Generate Swagger UI at `/swagger`

#### 3.3 Pre-commit Hooks (1 day)
- [ ] Set up `pre-commit` framework
- [ ] Add `cargo fmt --check` hook
- [ ] Add `cargo clippy -- -D warnings` hook
- [ ] Add fast test suite hook
- [ ] Document setup in `CONTRIBUTING.md`

#### 3.4 Database Optimization (3 days)
- [ ] Analyze slow queries with `EXPLAIN ANALYZE`
- [ ] Add missing indexes (especially on foreign keys)
- [ ] Optimize list queries with proper pagination
- [ ] Tune connection pool settings
- [ ] Add slow query logging configuration

#### 3.5 Logging Configuration (1 day)
- [ ] Configure JSON output for production
- [ ] Add correlation ID tracking across services
- [ ] Set up log levels per module
- [ ] Configure log rotation
- [ ] Update `docs/structured-logging.md`

#### 3.6 Security Hardening (2 days)
- [ ] Implement Content Security Policy (CSP)
- [ ] Fine-tune rate limits by endpoint
- [ ] Configure CORS properly for production
- [ ] Add security headers middleware
- [ ] Run `cargo audit` and address vulnerabilities

---

## 🚀 Medium-term Goals (Month 2)

### 4. Phase 4: Advanced Features ⏱️ 2-3 weeks

**Reference**: `PROJECT_STATUS.md` - Phase 4: Advanced Features

#### 4.1 Read Model Optimization (3 days)
- [ ] Analyze index query patterns
- [ ] Add materialized views for common queries
- [ ] Implement incremental index updates
- [ ] Add index rebuild CLI command
- [ ] Document read model architecture

#### 4.2 Event Replay System (3 days)
- [ ] Implement replay from offset
- [ ] Add read model rebuild from events
- [ ] Create replay CLI commands
- [ ] Add replay progress tracking
- [ ] Document replay procedures

#### 4.3 Advanced Caching (2 days)
- [ ] Add Redis integration option
- [ ] Implement cache warming strategies
- [ ] Add cache invalidation on events
- [ ] Configure multi-level caching
- [ ] Add cache metrics to Prometheus

#### 4.4 Performance Benchmarks (2 days)
- [ ] Set up `criterion` benchmarks
- [ ] Benchmark critical paths (node CRUD, catalog search)
- [ ] Create performance regression tests
- [ ] Document performance targets
- [ ] Add CI performance checks

#### 4.5 Multi-region Support (5 days)
- [ ] Design multi-region architecture
- [ ] Implement region-aware routing
- [ ] Add cross-region event replication
- [ ] Configure regional databases
- [ ] Document deployment topology

---

## 📚 Documentation Tasks (Ongoing)

### 5. Clarify Architecture Patterns ⏱️ 1-2 days

#### 5.1 TransactionalEventBus Pattern
- [ ] Document location (`rustok-outbox`, not `rustok-core`)
- [ ] Explain when to use `EventBus` vs `TransactionalEventBus`
- [ ] Add code examples to `RUSTOK_MANIFEST.md`
- [ ] Consider re-export from `rustok-core` for discoverability

#### 5.2 Service Layer Guidelines
- [ ] Document standard service constructor pattern
- [ ] Explain dependency injection approach (via constructor)
- [ ] Show examples of transactional operations
- [ ] Add testing patterns for services

#### 5.3 Event System Documentation
- [ ] Document `EventTransport` trait evolution
- [ ] Explain `as_any()` downcasting pattern
- [ ] Show examples of custom transports
- [ ] Add integration test patterns

---

## 🔍 Technical Debt Items

### Priority 1 (Fix Soon)
- [ ] ❌ rustok-test-utils event references
- [ ] ⚠️ Frontend build (parcel_css issue)
- [ ] [ ] All `.unwrap()` calls in production code

### Priority 2 (This Month)
- [ ] [ ] Slow queries without indexes
- [ ] [ ] Missing OpenAPI documentation
- [ ] [ ] No pre-commit hooks

### Priority 3 (Nice to Have)
- [ ] [ ] Event schema validation automation
- [ ] [ ] Performance benchmarks in CI
- [ ] [ ] Multi-region documentation

---

## 📊 Success Metrics

### Phase 3 Completion (Target: End of Month 1)
- [ ] Zero `.unwrap()` in production code
- [ ] 100% API endpoints documented
- [ ] Database p95 < 100ms
- [ ] All security headers configured
- [ ] Pre-commit hooks enforced

### Phase 4 Completion (Target: End of Month 2)
- [ ] Event replay functional
- [ ] Read model rebuild < 5 min
- [ ] Cache hit rate > 90%
- [ ] Performance benchmarks in CI
- [ ] Multi-region docs complete

### Overall Project Health
- [ ] Backend: ✅ Compiles cleanly
- [ ] Frontend: ✅ Compiles cleanly (currently ⚠️)
- [ ] Tests: ✅ 40%+ coverage (currently ~31%)
- [ ] CI/CD: ✅ All checks pass
- [ ] Security: ✅ Zero critical vulnerabilities

---

## 🎓 Lessons from Audit

### What Went Well
✅ Clean architecture with good separation of concerns  
✅ Consistent service patterns across modules  
✅ Strong typing with minimal `Any` usage  
✅ No cyclic dependencies

### Areas for Improvement
⚠️ Test infrastructure lagged behind code changes  
⚠️ External dependency (tailwind-rs) blocks frontend  
⚠️ Import confusion around TransactionalEventBus  
⚠️ Some documentation lags implementation

### Action Items
1. **Keep test-utils in sync** - Consider code generation or CI check
2. **Pin critical dependencies** - Avoid alpha versions in production path
3. **Document non-obvious patterns** - Like EventBus location
4. **Regular dependency audits** - Monthly `cargo outdated` + `cargo audit`

---

## 📅 Timeline Summary

**Week 1** (Current):
- Fix test infrastructure ✅
- Resolve frontend build issue 🔄
- Begin error handling audit

**Weeks 2-3**:
- Complete Phase 3 tasks
- Add comprehensive API docs
- Set up pre-commit hooks

**Week 4**:
- Database optimization sprint
- Security hardening
- Logging improvements

**Months 2**:
- Phase 4 implementation
- Advanced features
- Performance optimization

**Total Estimated Time**: 6-8 weeks to complete Phase 3 & 4

---

## 🤝 Next Session Checklist

Before starting next development session:

1. [ ] Review this plan
2. [ ] Check if test infrastructure is fixed
3. [ ] Verify backend still compiles: `cargo check --workspace`
4. [ ] Pull latest changes: `git pull origin main`
5. [ ] Review open issues/PRs
6. [ ] Pick highest-priority task from this document
7. [ ] Update `PROJECT_STATUS.md` with progress

---

**Last Updated**: February 11, 2026  
**Plan Version**: 1.0  
**Next Review**: After completing test infrastructure fixes
