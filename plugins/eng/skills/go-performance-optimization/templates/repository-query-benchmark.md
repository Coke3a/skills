# Repository Query Benchmark

Repository method / query:

- ...

Data shape (row counts, index state, production-likeness):

- ...

Go benchmark (integration, real or containerized PostgreSQL):

```sh
go test -bench=BenchmarkExampleRepositoryFindByID -benchmem -count=10 ./internal/infra/postgres | tee old.txt
```

Query plan evidence:

```sql
EXPLAIN (ANALYZE, BUFFERS) <the actual generated query with real parameters>;
```

Results:

| Metric            | Baseline | After |
| ----------------- | -------- | ----- |
| time/op           |          |       |
| allocs/op         |          |       |
| DB execution time |          |       |
| Rows / buffers    |          |       |

Change made (index, query shape, batch method, mapping):

- ...

Migration required:

- ...

Notes / variance:

- ...
