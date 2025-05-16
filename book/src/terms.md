# Terms

This page explores key terms and concepts used in Bevy.
We'll delve deeper into how they are applied throughout the tutorial.

Feel free to regularly come back here to re-read it if something seems unclear.

## ECS

Bevy is fundamentally a data-driven game framework.
As such, understanding how it achieves this through implementing an ECS pattern
is necessary to use bevy.

Luckily, getting started only needs surface-level understanding.
Further progress will then be achieved by getting more familiar with
structuring your logic around the ECS paradigm.

*Note: The initials are not in order, to make it easier to understand/follow.*

### C for Component

*A Component is a piece of data.*

Most data is fairly simple: integers, strings. Representing Health, positions,
cooldowns etc.
But you are not limited to those. You can use most Rust structures and use them
as components.

This seems fairly restrictive at first, but it will make sense later on when
using Entities and Systems to compose more complex behavior.

### E for Entity

*An Entity is a set of Components.*

The **Entity** is the container through which the an ECS is organized.
It allows to group components, and what you will work with in systems.

As an example, a `Player` entity might contain the following **Components**:

- `Position`
- `Health`
- `KeyboardController`

As you can see, it might have what one considers 'simple data' but also
components that traditionally would be considered 'logic'.

If this is not clear how bevy achieves this, read on and it will make sense to
you throughout the tutorial chapters.

### S for System

*A system is a function that uses entities and their components.*

The **System** is what glues different Components together.
For example, you might have a system for:

- updating the position of an entity based on its velocity. Both `Position` and
  `Velocity` would be components.
- updating the `Health` component based on `FireResistance` and a `OnFire` component.

## Bevy Concepts

While bevy uses the ECS paradigm, it is a **game engine**.
As such it comes with its own abstractions to save you time implementing them
yourself.
Be sure to familiarize with them, as they will used throughout this tutorial.

### Application

The Application is the main entry point of Bevy.
It serves as the glue between all parts that your game needs:

- managing the game loop and scheduling systems
- coordinating the lifecycles of your game states
- handling resources and events

You will create one application, usually in your `main` method.
You then add all the different parts of your game you need to it.

### Plugin

Games, or in general most applications, can have *a lot* of different parts.
Too many to list them out in a single place.
It would get unwieldy, and potentially introduce bugs by being hard to read.

Plugins are bevy's way of abstracting pieces of your game into smaller pieces.
Instead of adding 10 systems, 5 resources, and 6 events.
You add a `HealthPlugin`.

Composing your app this way, also allows you to use rust's visibility system to
ensure invariants of your implementation.

### World

The World is the central data structure that stores all entities and their components.
It provides methods to query and manipulate entities and components.

You generally don't have to access the World directly.
Some use-cases still require you to.
Keep in mind, that running a system with access to a mutable World means that
no-one else can run at the same time.
We are using Rust after all!

### Query

A Query is how to access entities and their components.
As the name suggests, they can 'query': You choose what data, and under which
conditions you would want it.

For example, you might want to query for "All entities with a Health component,
that do not have a Position component". You use a `Query` for that!

### Commands

Commands are used to schedule changes to the World.
You can for example add or remove components from entities, or even remove
whole entities.
They are executed later during the same frame, after the system that generated
them ended.

### Resource

Resources are globally accessible data structure that are not tied to any specific entity.
They are used to store shared data and state.

You can think of them as Singletons, and allow to store long-lived data.

### Event

Events are messages that can be sent and received by systems.
Events are used to communicate between systems and decouple their logic.

### Observer

Observer are systems that react to changes in the World, such as component
modifications or entity creation.
It is used to implement reactive behavior, allowing for tightly coupled
behavior.

### Relations

Relations are a way to link entities together.
The most common is the `ChildOf` / `Children` relation, which propagates things
like position and visibility.

You can create your own, allowing you to represent what your game needs.
