# Decorator design pattern
Pattern type: Structural

## Definition
Attach additional responsibilities to an object dynamically. Decorators provide a flexible alternative to subclassing for extending functionality.

## Details
Decorator pattern allows to add new funcionality to object without altering its structure. It is done by encapsulating original object inside an wrapper interface. Both the decorator object and the core object inherit from same interface.
The interface uses recursive composition to allow an unlimited number of decorator "layers" to be added to core object.

![Decorator diagram](decorator.png)

This pattern is an example or prefering composition over inheritance and in this case it allows us to greately decrease subclassing.

## Example
Example code provides a solution for having a different pizza toppings variants. We have a base version of pizza which can be decorated(wrapped) with different ingredients: cheese, chicken. The final price of pizza depends on pizza base price and prices of all ingredients that we added to it.