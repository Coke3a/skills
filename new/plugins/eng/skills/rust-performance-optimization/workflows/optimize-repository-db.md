# Optimize Repository and Database

## 1. Confirm DB bottleneck

- [ ] Query timing or profile indicates DB/repository bottleneck.
- [ ] N+1, missing index, broad transaction, or excessive rows/columns are suspected.

## 2. Inspect

- [ ] Repository trait method.
- [ ] Diesel query.
- [ ] Query filters/order/limit.
- [ ] Index/migration needs.
- [ ] Transaction scope.
- [ ] Row -> domain mapping cost.
- [ ] DB pool usage.

## 3. Optimize safely

- [ ] Keep Diesel code in infra.
- [ ] Keep row structs private.
- [ ] Add repository trait method if batch query is needed.
- [ ] Add migration for index/schema change if needed.
- [ ] Keep usecase orchestration clean.
- [ ] Do not query DB from handler.

## 4. Verify

- [ ] Add/update repository integration test if query behavior changed.
- [ ] Run DB benchmark or query timing.
- [ ] Run correctness tests.
