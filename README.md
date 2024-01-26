# shout
A URL shortner

## Generate Short Url
- send post request with full url and email which you want to verify
```
  method: POST 
  body: { "email": "<your@gmail.com>", "url": "<full_url_you_want_to_short>"}
```

## Verify and Get full Url
- just type `localhost:3000/<short_url>` in browser to recieve full url on email or
- send get request with short url in address and then you will recieve full url on email
```
  method: GET
  address: http://localhost:3000/<short_url>
```
