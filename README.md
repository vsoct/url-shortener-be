# url-shortener-be

Getting our hands wet by learning how to build a URL shortener with Rust🦀

## Available routes

| Route | Method | Description | Working |
| ---- | ----------- | ---- | ---- |
| / | GET | Returns server health status | ✅ |
| / | POST | Shortens a url. Body format: `{"url" : "https://villagesquare.com"}` | ✅ |
| /{id} | GET | Expands and redirects a shortened url | ✅ |
| /admin/urls | GET | Returns the current list of shortened urls | ✅ |
| /admin/urls/{id} | DELETE | Deletes a shortened url entry by id | ✅ |

## .env

`
BASE_URL=http://127.0.0.1:8080
SERVER_ADDRESS=127.0.0.1
SERVER_PORT=8080
`
