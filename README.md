# url-shortener-be

Getting our hands wet by learning how to build a URL shortener with Rust🦀

## API Endpoint

<https://url-shortener-vs.herokuapp.com>

## Available routes

| Route | Method | Description | Working |
| ---- | ----------- | ---- | ---- |
| / | GET | Returns server health status | ✅ |
| / | POST | Shortens a url. Body format: `{"url" : "https://villagesquare.com"}` | ✅ |
| /{id} | GET | Expands and redirects a shortened url | ✅ |
| /admin/urls | GET | Returns the current list of shortened urls | ✅ |
| /admin/urls/{id} | DELETE | Deletes a shortened url entry by id | ✅ |

## .env

```.env
BASE_URL=http://127.0.0.1:8080
HOST=127.0.0.1
PORT=8080
```
