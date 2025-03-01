# ⚓ Ballast

Ballast is a tool for snapshot load testing apis.

![output](https://github.com/synoet/ballast/assets/10552019/498c9ada-4d55-4074-8f82-d6c32c6f558b)

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
      "url": "http://localhost:8080/test",
      "method": "GET",
      "concurrent_requests": 5,
      "cycles": 10,
      "threshold": 100,
      "expected_status": 500
    },
    {
      "name": "POST test",
      "url": "http://localhost:8080/test",
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
    }
  ]
}
```

_**Note:** For more configuration options, see the [Configuration](#configuration) section._

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

Below are the available configuration options for use in `ballast.json` for configuring test(s).

| **key**               | **required** | **default** | **description**                                                                                                                                                          |
|:----------------------|:------------:|:-----------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `name`                |              |             | The name you are giving your test.                                                                                                                                       |   
| `url`                 |              |             | The HTTP endpoint you are testing.                                                                                                                                       |   
| `method`              |              |             | The HTTP method. e.g. `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `OPTIONS`                                                                                                 |   
| `concurrent_requests` |              |             | The number of concurrent requests to run per testing cycle. You can think of total requests in a test as `concurrent_requests * cycles`.                                 |   
| `cycles`              |              |             | The number of cycles in a test.                                                                                                                                          |   
| `threshold`           |              |    250ms    | The acceptable deviation of average response time for a test to be successful. Response time is used to measure the requests success if all other expected values match. |   
| `headers`             | *(optional)* |             | A map of headings to include on the request.                                                                                                                             |   
| `body`                | *(optional)* |             | A json payload to include in your request.                                                                                                                               |   
| `expected_status`     | *(optional)* |             | The status you're expecting the endpoint to return if it functions correctly.                                                                                            |   
| `expected_body`       | *(optional)* |             | The expected response for the endpoint.                                                                                                                                  |   
| `expected_headers`    | *(optional)* |             | The expected response headers for the endpoint.                                                                                                                          |   
| `ramp`                | *(optional)* |    true     | Should the request be ramped up, or not.                                                                                                                                 |
