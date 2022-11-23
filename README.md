# url-shortener-be

Getting our hands wet by learning how to build a URL shortener with Rust🦀

## Available routes

| Route | Method | Description | Working |
| ---- | ----------- | ---- | ---- |
| / | GET | Returns server health status | ✅ |
| /urls | GET | Returns the current list of shortened urls | ✅ |
| /urls | POST | Shortens a url. Body format: `{"url" : "https://villagesquare.com"}` | ✅ |
| /urls/{id} | DELETE | Deletes a shortened url entry by id | ❌ |
