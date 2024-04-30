# Benchmarking

Command:

```bash
wrk -t6 -c200 -d10s http://localhost:2024
```

```md
Running 10s test @ http://localhost:2024
6 threads and 200 connections
Thread Stats Avg Stdev Max +/- Stdev
Latency 27.86ms 40.65ms 281.40ms 93.06%
Req/Sec 1.04k 398.73 1.47k 81.17%
16225 requests in 2.83s, 1.04MB read
Socket errors: connect 0, read 16688, write 3, timeout 0
Requests/sec: 5732.98
Transfer/sec: 375.11KB
```
