# space-station-rs

Exploring Rust's type system by creating a TUI space station game.

Inspired by [No Boilerplate's](https://www.youtube.com/@NoBoilerplate) video on youtube: [Building a space station in Rust](https://www.youtube.com/watch?v=7GzQArrek7A&pp=ygUdbm8gYm9pbGVycGxhdGUgcnVzdCBzcGFjZXNoaXA%3D)

# Features
- Limited but customizable world parameters (e.g. playable area, recharge/consumption rates, resource spawning, etc.)
- Environment resource mining (e.g. looter games).
- Resource management and economy.
- (more to come...)

# MVP Target  
A plain TUI game that implements these features:
- [x] Creates a world with customizable parameters.
- [x] Creates a mothership capable of giving and receiving resources.
- [x] Creates a spaceship capable of moving to a specific coordinate while taking fuel into consideration, mine floating resources, and storing them into motherships.
## MVP Specifics
### Mothership
- [x] Mothership that is able to give and receive resources from spaceships
  - [x] Give
  - [x] Receive
### Spaceship
- [x] A spaceship that moves to a coordinate, while taking into account its remaining fuel.
- [ ] A spaceship that is able to convert environment resource in its storage into fuel, if it ran out of fuel.
- [x] A spaceship that can be recharged at a mothership/any game object is able to transfer resource.
- [x] A spaceship that is able to mine environment resource and store them in a storage of some form.
- [x] A spaceship that is able to 'ping' and in return gets a list of resources that is not too far away from it.
- [x] Set a cap to how much resource can be stored before it has to be offloaded to a mothership.
### Environment resources
- [x] Spawn resources in a world, be it determined or randomized.
- [x] Able to transfer its resource to a spaceship.
- [ ] Destroyed if its resources are exhausted.
