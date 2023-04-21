# Custom Log Provider

Goal of this project is to learn how to implement my own logger than can be used as a provider for the Log crate. 

The secondary goal was to figure out how to use dependency injection with a "LogProvider" trait that I can implement on N number of concrete LogProviders (db logger, axiom, datadog, etc).

### Concerns

1. Given my use of traits, I had to use a lot of Box<dyn > types as well as Box::new() in my main.rs binary file. I don't fully understand why this was necessary and am working to better my understanding here. I'm also generally unsure if this was the best approach for this implementation. 

2. I implemented Send and Sync traits using unsafe. I'm also unsure both why this was necessary and if it will lead to issues in the future.
