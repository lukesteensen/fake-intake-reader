# `fake-intake-reader`

This is a little program that polls the `fakeintake` service, collects all the
requests that have been sent to it, and looks for gaps in the contained metric
points.

When using this with the Datadog Agent, make sure to send requests in JSON so
that we can parse them easier:

```yaml
use_v2_api.series: false
```
