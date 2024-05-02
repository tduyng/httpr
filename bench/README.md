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

Using tokio multithread:

```md
Running 5s test @ http://localhost:2024
6 threads and 200 connections
Thread Stats Avg Stdev Max +/- Stdev
Latency 18.77ms 46.66ms 396.86ms 96.04%
Req/Sec 1.35k 560.31 1.73k 83.61%
16456 requests in 5.04s, 1.05MB read
Socket errors: connect 0, read 16489, write 0, timeout 0
Requests/sec: 3263.56
Transfer/sec: 213.53KB
```
