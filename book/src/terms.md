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

### E for Entity

An Entity is a unique identifier that represents a general-purpose object in the ECS. It acts as a pointer.

### C for Component

A Component is a data structure that holds information or attributes of an entity. Components are used to store the state and data of entities.

### S for System

A System is a function that operates on entities with specific components. Systems define the behavior and logic of the ECS by processing entities' components.

### Another way to think about ECS

It's a database!

Entities are the index, components are the columns and systems are procedural queries.


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
