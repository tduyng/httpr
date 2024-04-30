# Benchmarking

Command:

```bash
wrk -t6 -c200 -d5s http://localhost:2024
```

```md
Running 5s test @ http://localhost:2024
6 threads and 200 connections
Thread Stats Avg Stdev Max +/- Stdev
Latency 24.65ms 27.38ms 257.99ms 96.97%
Req/Sec 0.92k 301.15 1.37k 84.48%
16345 requests in 5.02s, 1.04MB read
Socket errors: connect 0, read 16771, write 13, timeout 0
Requests/sec: 3259.19
Transfer/sec: 213.25KB
```
