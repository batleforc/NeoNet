# NeoNet Tracing

What's mean by [tracing](https://docs.rs/tracing/latest/tracing/) in NeoNet is the hability to debug what happend and extract anonymized metrics.

## Kind

There is 3 kind of tracing provided in the app

### File

Allow the user to store the trace log json formated in a file, this file would look closer to a log file.

### Console

This kind will output the event created by the app directly inside the console.

### Otel or OpenTelemetry

My personal favourite. [Otel](https://opentelemetry.io/) allow the push of each event directly in a subscribe like [Jagger](https://www.jaegertracing.io/) or [Tempo](https://grafana.com/oss/tempo/). On the long run it will alow an easy monitoring of the app. With the goal of maintaining the app on the long run including Tracing and Otel will make the app more debug ready.

## Configuration

In order to setup your tracing endpoint, please set it up in the config.yaml file. You will be hable to setup the three kind at once but it's really needed.