// Template: Go benchmark file. Replace names and workloads with the real
// target. Place next to the code under test as {name}_bench_test.go (any
// _test.go file works; the _bench suffix is a convention for discoverability).
//
// Run:
//   go test -bench=BenchmarkCreateExampleEntity -benchmem -count=10 ./... | tee old.txt
//   ... apply change ...
//   go test -bench=BenchmarkCreateExampleEntity -benchmem -count=10 ./... | tee new.txt
//   benchstat old.txt new.txt
package examplefeature

import (
	"encoding/json"
	"fmt"
	"testing"
)

// Go 1.24+ style: b.Loop() runs setup once and keeps benchmarked calls alive
// (no dead-code-elimination sink needed).
func BenchmarkCreateExampleEntityName(b *testing.B) {
	raw := "a realistic example name from production data"

	b.ReportAllocs()
	for b.Loop() {
		name, err := NewExampleEntityName(raw)
		if err != nil {
			b.Fatal(err)
		}
		_ = name
	}
}

// Pre-Go-1.24 style with explicit timer control and sink.
var sinkString string

func BenchmarkMapRowsToOutput(b *testing.B) {
	rows := makeRealisticRows(1000) // deterministic, production-shaped input

	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		out := mapRowsToOutput(rows)
		sinkString = out[0].Name
	}
}

// Table benchmarks compare input sizes or implementations side by side.
func BenchmarkSerializeResponse(b *testing.B) {
	for _, size := range []int{1, 100, 10_000} {
		b.Run(fmt.Sprintf("items=%d", size), func(b *testing.B) {
			payload := makePayload(size)

			b.ReportAllocs()
			for b.Loop() {
				if _, err := json.Marshal(payload); err != nil {
					b.Fatal(err)
				}
			}
		})
	}
}
