# OpenTelemetry

OpenTelemetry is becoming a standard for tracing distributed applications. The Axum stack supports it nicely.

Let's get started by creating a Dockerized `SigNoz` server to gather and display OpenTelemetry data.

The quick way to install it is to clone the GitHUb repo and run the install script to setup a Dockerized server:

```bash
git clone -b main https://github.com/SigNoz/signoz.git && cd signoz/deploy/
./install.sh
```

This can take a while, so I'm not going to make you watch! As they say on kids TV shows, "here's one I made earlier". You will see the following information dump:

```
++++++++++++++++++ SUCCESS ++++++++++++++++++++++

🟢 Your installation is complete!

🟢 Your frontend is running on http://localhost:3301

ℹ️  By default, retention period is set to 15 days for logs and traces, and 30 days for metrics.
To change this, navigate to the General tab on the Settings page of SigNoz UI. For more details, refer to https://signoz.io/docs/userguide/retention-period 

ℹ️  To bring down SigNoz and clean volumes : sudo docker-compose -f ./docker/clickhouse-setup/docker-compose.yaml down -v
```

Go to [http://localhost:3301/](http://localhost:3301/) in a browser to view the newly installed SigNoz OpenTelemetry system.

The system is now listening for telemetry data on: `http://localhost:4317`