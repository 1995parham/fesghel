import http from 'k6/http';
import { check, group } from 'k6';

const baseURL = "http://127.0.0.1:1378"

export default function() {
  group("healthz", () => {
    let res = http.get(`${baseURL}/healthz`);

    check(res, {
      "success": (res) => res.status === 204,
    });
  });
}
