# Service Deployment

In this section, we're going to talk about different ways to deploy your Rust services. We won't dive into Kubernetes---let's leave that for the subject experts. We will discuss:

| Deployment Type | Description | Comment |
|--|--|--|
| Direct to OS | Installed on a host with an OS, no virtualization | The fastest without also writing an operating system |
| Direct to VM | Installed on a host that is running a virtual machine system. Installed inside a VM on the VM system. | VMs impose overhead, so not as fast as native. Depending upon your VM system, your VM may now be relocatable. |
| Docker | Docker, either on a physical host or a VM | Docker takes the overhead of the parent system (physical host or VM), and adds a little more as it runs inside a container (or a VM on Windows/Mac). |
| Docker Compose | Multiple Docker Images | Same as Docker |

If you're using Rust for its high-performance and low-overhead, there are definite benefits to running as close to the metal as possible. Conversely, there are management benefits from virtualizing and/or containerizing. As with many things in IT, it's a trade-off. I can't give you the right answer for your needs!