# Handling Service Configuration

Configuration in a service-oriented environment can be quite complicated. Depending upon your setup, you may want to receive configuration from one or more of these targets:

* **Environment Variables** --- particularly in Kubernetes and Docker based systems, passing configuration by environment variable is very common, often required.
* **Configuration Files** --- you may want to read a configuration file and obtain settings from there.
* **HTTP** --- some orchestration systems provide a unified configuration management setup, expecting your application to retrieve configuration over HTTP.
* **Command Line** --- and you may just want to configure parts of your application from the command-line. If there is setup involved in bootstrapping your service (for example, adding first users to an authentication stack), you may even require support for this.

On top of that, services need to be able to access their active configuration---both during setup, and at runtime. Just in case you weren't worried yet, you may also be hosting multiple services together in a modular monolith---and want them to each handle much of their own configuration!

In this section, we're going to dive through many of the configuration options you have at your disposal.
