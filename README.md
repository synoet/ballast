
https://github.com/synoet/ballast/assets/10552019/89a35ad2-8408-468f-bdbc-fb0a15409be9
# ⚓ Ballast  
Ballast is a tool for snapshot load testing apis.



https://github.com/synoet/ballast/assets/10552019/25572acb-1f67-49fc-b88a-97d6a02be671




## Introduction
#### Installation
```bash
cargo install ballast
```

#### Setup
Create a `ballast.json` config file in your directory.
```json
{
  "endpoints": [
    {
      "name": "GET test",
      "url":"http://localhost:8080/test",
      "method": "GET",
      "concurrent_requests": 5,
      "cycles": 10,
      "threshold": 100,
      "expected_status": 500,
    },
    {
      "name": "POSR test",
      "url":"http://localhost:8080/test",
      "method": "POST",
      "concurrent_requests": 5,
      "cycles": 10,
      "threshold": 100,
      "expected_status": 200,
      "headers": {
        "Content-Type": "application/json"
      },
      "body": {
        "payload": {}
      }
    },
  ]
}
```

#### Run
```bash
ballast # in the directory with ballast.json
```

## Why?
**What is snapshot testing?**

Snapshot testing is a commonly used technique for testing UI frameworks such as Jest + React. It involves capturing a "snapshot" of the DOM at a specific point and using it as a reference for comparison after making changes.

**How does this apply to APIs?**

Applying a similar approach, when running a load test, ballast automatically generates a snapshot of your test. By comparing performance to a snapshot after making changes, API developers can assess how these changes affect performance.

## Configuration
**Configuring Tests**   
`name`: The name you are giving your test.   
`url`: HTTP endpoint you are testing.   
`method`: `GET` | `POST` | `PUT` | `DELETE` | `PATCH` | `OPTIONS`   
`concurrent_requests`: how many concurrent requests to run per testing cycle, you can think of total requests in a test as `concurrent_requests * cycles`   
`cycles`: number of cycles in a test   
`threshold`: This is the acceptable deviation of average response time for a test to be successful. by default it is `250ms`. Response time is used to measure the requests success if all other expected values match.   
`headers` *(optional)*: A map of headings to include on the request   
`body` *(optional)*: some json payload to include in your request   
`expected_status` *(optional)*: the status you're expecting the endpoint to return if it functions correctly   
`expected_body` *(optional)*: the expected response for the endpoint   
`expected_headers` *(optional)*: the expected response headers for the endpoint   
