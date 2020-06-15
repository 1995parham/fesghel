# fesghel
[![Drone (cloud)](https://img.shields.io/drone/build/1995parham/fesghel.svg?style=flat-square)](https://cloud.drone.io/1995parham/fesghel)

## Introduction
Here I am trying to implement an URL shortener with concepts that I use in Golang.
It uses MongoDB as its storage.

## Nomenclature
Fesghel is a name of the following haapoo 🐶

![haapoo](.img/haapoo.jpg)

## Load Testing
```
    checks.....................: 99.80% ✓ 2994  ✗ 6
    data_received..............: 2.0 MB 58 kB/s
    data_sent..................: 520 kB 15 kB/s
    group_duration.............: avg=878.92ms min=299.68µs med=440.42ms max=34.21s  p(90)=1.97s   p(95)=3.23s
    http_req_blocked...........: avg=13.63ms  min=0s       med=3µs      max=2.75s   p(90)=12µs    p(95)=144.44µs
    http_req_connecting........: avg=5.81ms   min=0s       med=0s       max=2.39s   p(90)=0s      p(95)=4.64µs
    http_req_duration..........: avg=381.12ms min=0s       med=143.99ms max=6.32s   p(90)=1.13s   p(95)=1.73s
    http_req_receiving.........: avg=114.92µs min=0s       med=56µs     max=13.12ms p(90)=203µs   p(95)=323.19µs
    http_req_sending...........: avg=50.86µs  min=0s       med=22µs     max=6.26ms  p(90)=83.09µs p(95)=133µs
    http_req_tls_handshaking...: avg=7.76ms   min=0s       med=0s       max=1.4s    p(90)=0s      p(95)=0s
    http_req_waiting...........: avg=380.96ms min=0s       med=143.75ms max=6.32s   p(90)=1.13s   p(95)=1.73s
    http_reqs..................: 4000   116.781225/s
    iteration_duration.........: avg=1.76s    min=166.99ms med=1.23s    max=34.22s  p(90)=3.5s    p(95)=4.14s
    iterations.................: 1000   29.195306/s
    vus........................: 100    min=100 max=100
    vus_max....................: 100    min=100 max=100
 ```
