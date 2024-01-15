# Ballast  
Ballast is a tool for snapshot load testing apis.

**What is snapshot testing?**

Snapshot testing is a commonly used technique for testing UI frameworks such as Jest + React. It involves capturing a "snapshot" of the DOM at a specific point and using it as a reference for comparison after making changes.

**How does this apply to APIs?**

Applying a similar approach, when running a load test, ballast automatically generates a snapshot of your test. By comparing performance to a snapshot after making changes, API developers can assess how these changes affect performance.

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

**Example**
```json
{
  "endpoints": [
    {
      "name": "Endpoint 1",
      "url":"https://www.google.com",
      "method": "GET",
      "concurrent_requests": 10,
      "cycles": 20,
      "threshold": 100,
      "expected_status": 500,
      "expected_body": {
        "value": "hello"
      },
      "expected_headers": {
        "Content-Type": "application/json"
      }
    },
  ]
}
```

We define a test named "Endpoint 1" for the URL "https://www.google.com". The method used is "GET". The test consists of 20 cycles with 10 concurrent requests in each cycle.   

We set a threshold of 100ms, which means that it is acceptable for the average response time to deviate by 100ms since the previous test. If there is no previous request, this is not considered a metric for success.
We also define our expected response body and headers.   

Running ballast will yield the following result.   
![CleanShot 2024-01-15 at 15 19 08](https://github.com/synoet/ballast/assets/10552019/0806f796-8c98-4aa1-b571-442a5658c2bd)


The test failed because we didn't match three expected values: [body, status, headers]. Below are the stats for the entire test.   

As this is our first time running this test, there is no threshold check.   

When we run it again (with the threshold set to 1ms for testing purposes), we receive the threshold message.      
![CleanShot 2024-01-15 at 15 21 02](https://github.com/synoet/ballast/assets/10552019/6477d546-3be2-4dd0-9544-78d6d368dde2)


Here we have the message stating that we were expecting an average response time of 687 (according to the last test snapshot) with a threshold value.   

However, it is important to note that this was only for demonstration purposes. We do not actually expect such performance from google.com. Let's switch back to a more realistic scenario.   
```json
{
  "endpoints": [
    {
      "name": "Endpoint 1",
      "url":"https://www.google.com",
      "method": "GET",
      "concurrent_requests": 10,
      "cycles": 20,
      "threshold": 100,
      "expected_status": 200
    }
  ]
}
```

I removed the body and headers, and set the expected status to 200. I also reverted the threshold to 100ms.  
![CleanShot 2024-01-15 at 15 24 36](https://github.com/synoet/ballast/assets/10552019/52379db2-754b-4aaf-bff4-b82ac13af241)  


When running this, we obtain a pass! We can observe the deviation in each statistic from the previous run. However, since the average is within the 100ms threshold, it is considered a pass. Happy testing!
