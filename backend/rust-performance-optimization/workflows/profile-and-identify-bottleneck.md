# Profile and Identify Bottleneck

## 1. Prepare

- [ ] Build release binary.
- [ ] Enable debug line info/frame pointers if needed.
- [ ] Choose realistic workload.

## 2. Profile

- [ ] Use appropriate profiler.
- [ ] Capture CPU profile, allocation profile, or DB profile.
- [ ] Record command and environment.

## 3. Analyze

- [ ] Identify hot function/path.
- [ ] Identify whether bottleneck is CPU, allocation, lock, IO, DB, serialization, or async scheduling.
- [ ] Confirm bottleneck is in scope.

## 4. Plan

- [ ] Propose smallest safe optimization.
- [ ] Identify affected layer.
- [ ] Confirm architecture boundaries remain intact.
