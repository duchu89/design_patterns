# Abstract factory pattern
Pattern type: Creational

## Definition
Provide an interface for creating families of related or dependent objects without specifying their concrete classes.

## Details
Abstract factory pattern is used to create a set of related or dependant objects. This pattern enforces behaviour of using only related products together, which allows multiple families or products to be switched between easily.

![Abstract factory diagram](abstract_factory.gif)

Client software creates a concrete implementation of the abstract factory and then uses the generic interfaces to create the concrete objects that are part of the family of objects.
The client does not know or care which concrete objects it gets from each of these concrete factories since it uses only the generic interfaces of their products.

## Example
Example is focused around families of UI widgets (buttons, progress bars) for different operating systems. We define trait `GUIFactory` which has following methods: `create_button` and `create_progress_bar`. We also define basic traits for UI widgets: `Button` and `ProgressBar`. We create an implementation of this traits both for Material and Cupertino designs. We define two factories: `AndroidFactory` and `IOSFactory`. Using factories to get UI widgets prevents us from mixing UI elements from different systems.
