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

### Application

The Application is the main entry point of Bevy. It manages the game loop, schedules systems, and handles resources and events. It exists only during setup, and is not available once the game loop started.

### Plugin

A Plugin is a modular piece of functionality that can be added to a Bevy app. It encapsulates systems, resources, and configuration. It exists only at build time.

### World

The World is a data structure that stores all entities and their components. It provides methods to query and manipulate entities and components.

### Query

A Query is used to access entities and their components in a World. It allows systems to filter and iterate over entities with specific component sets.

### Commands

Commands are used to schedule changes to the World, such as adding or removing entities and components. They are executed later during the same frame, after the system that generated them ended.

### Resource

A Resource is a globally accessible data structure that is not tied to any specific entity. It is used to store shared data and state.

### Event

An Event is a message that can be sent and received by systems. Events are used to communicate between systems and decouple their logic.

### Observer

An Observer is a system that reacts to changes in the World, such as component modifications or entity creation. It is used to implement reactive behavior.

### Relations

Relations are a way to link entities together. The most common is the parent / children relation, which propagates things like position and visibility.
