import http from 'k6/http';
import { check, group } from 'k6';

const baseURL = __ENV.BASE_URL || 'http://127.0.0.1:1378';

export const options = {
  vus: 10,
  duration: '30s',
  thresholds: {
    http_req_duration: ['p(95)<200'],
    http_req_failed: ['rate<0.01'],
  },
};

export default function () {
  group('healthz', () => {
    const res = http.get(`${baseURL}/healthz`);

    check(res, {
      'status is 204': (r) => r.status === 204,
    });
  });

  group('short', () => {
    let name = '';

    group('create', () => {
      const payload = JSON.stringify({
        url: 'https://elahe-dastan.github.io',
      });

      const res = http.post(`${baseURL}/api/urls`, payload, {
        headers: {
          'Content-Type': 'application/json',
        },
      });

      const success = check(res, {
        'status is 200': (r) => r.status === 200,
        'response time < 200ms': (r) => r.timings.duration < 200,
      });

      if (success) {
        name = res.json();
      }
    });

    if (!name) {
      return;
    }

    group('fetch', () => {
      const res = http.get(`${baseURL}/api/${name}`, {
        redirects: 0,
      });

      check(res, {
        'status is 307': (r) => r.status === 307,
        'has location header': (r) => r.headers['Location'] !== undefined,
      });
    });
  });
}
