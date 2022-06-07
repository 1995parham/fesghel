# Fesghel

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/1995parham/fesghel/ci?label=ci&logo=github&style=flat-square)

## Introduction

Here I am trying to implement an URL shortener with concepts that I use in Golang but this time in Rust.
It uses MongoDB as its storage backend.

## Nomenclature

Fesghel is a name of the following haapoo 🐶:

![haapoo](.img/haapoo.jpg)

## Load Testing

As you can see response time in Rust is much better than Golang.

```
    checks.....................: 99.60% ✓ 2988  ✗ 12
    data_received..............: 1.9 MB 62 kB/s
    data_sent..................: 516 kB 17 kB/s
    group_duration.............: avg=346.67ms min=234.35µs med=141.88ms max=30.76s   p(90)=229.84ms p(95)=685.55ms
    http_req_blocked...........: avg=21.66ms  min=0s       med=2µs      max=2.87s    p(90)=5µs      p(95)=24.09µs
    http_req_connecting........: avg=8.24ms   min=0s       med=0s       max=2.33s    p(90)=0s       p(95)=0s
    http_req_duration..........: avg=61.05ms  min=0s       med=14.22ms  max=605.39ms p(90)=169.05ms p(95)=194.65ms
    http_req_receiving.........: avg=52.36µs  min=0s       med=39µs     max=13.13ms  p(90)=76µs     p(95)=99.04µs
    http_req_sending...........: avg=24.95µs  min=0s       med=15µs     max=888µs    p(90)=46µs     p(95)=64µs
    http_req_tls_handshaking...: avg=8.81ms   min=0s       med=0s       max=468.61ms p(90)=0s       p(95)=0s
    http_req_waiting...........: avg=60.98ms  min=0s       med=14.15ms  max=605.26ms p(90)=168.95ms p(95)=194.42ms
    http_reqs..................: 4000   129.987115/s
    iteration_duration.........: avg=694.13ms min=131.95ms med=190.51ms max=30.77s   p(90)=748.75ms p(95)=1.51s
    iterations.................: 1000   32.496779/s
    vus........................: 100    min=100 max=100
    vus_max....................: 100    min=100 max=100
```
