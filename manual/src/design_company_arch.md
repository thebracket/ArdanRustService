# Understanding Your Company Architecture

We've talked (and will keep talking) about your *program architecture*, but it's important not to miss the big picture. Your company also has an architecture, and your service(s) need to work for the company.

For example, adopting microservices is often a *political* not a *technical* decision. If your company is large enough that you have many teams who take responsibility over individual parts of your service architecture---and need the flexibility to deploy independently of one another---breaking your system up into team-sized chunks can make sense from a *political* perspective. Some of the original arguments for microservices included "overcoming fiefdoms" (a single IT group saying "this is ours"), scoping devops (so the developers can also be systems administrators, DBAs, etc.), keeping meetings to reasonable durations (not everyone has to discuss *everything*). Behind those, come arguments for scalability and resilience.

It's also really important to be realistic. If you're just starting out with a small team of plucky developers, and have 1-2 customers---there's a good chance you can build a monolith that runs the entire project on a $6-25/month Digital Ocean droplet. That's what LibreQoS did to get started! There are all manner of stories of "we built a huge Kubernetes architecture, with redundant servers on each coast. We were READY for web-scale --- and unfortunately, our 3 customers didn't pay enough to keep us afloat."

It's sadly less common to hear tales of "we launched and had a million customers in a week. Our infrastructure just couldn't keep up, and we failed because we couldn't scale". It does happen---but it's rare.

At the next level, if you're working for an established enterprise---you need to fit within whatever company architecture they are demanding. So make sure you understand what's expected, and tailor your solutions to fit the need.

And it's possible that you're working for an established "web scale" enterprise, and picked Rust because you need to replace a massive volume service with something highly performant, stable and safe. Once again, you need to work with what your enterprise demands.

Let's consider some *company architecture* decisions:

|Company|Description|Needs|Goal|
|--|--|--|--|
|Sole Developer|It's just you, you want to have fun and not get burned out by GitHub requests|Keep it Simple! Scale may not be the top priority, but avoiding spaghetti is.|Modular Monolith|
|Small Startup|You're just getting going and don't have many customers yet|Keep costs low. Keep it simple. Scale isn't a priority now, but you hope it will be. Telemetry.|Modular Monolith|
|Rapidly Expanding|You've got customers and need to scale to fit demand.|Telemetry, understand the pain points. Code that supports adding developers.|Modular Monolith that can be divided|
|Webscale!|You've made it! You're supporting Google-style load, and heating data-centers around the world.|Telemetry, Manageability, Supporting Diverse Teams|Readily divided services|

