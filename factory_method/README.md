# Factory method design pattern
Pattern type: Creational

## Definition
The factory method design pattern defines interface for creating an object. But lets subclasses decide which class to instantiate. Factory method lets the class defer instantiation to subclasses.

## Details
Factory method pattern allows the object creation logic to be hidden. Two interfaces are defined: *Creator* and *Product*.
*Creator* contains method which return object implementing *Product* interface. Concrete Creators implement *Creator* interface, while concrete Product implement *Product* interface.

![Factory method diagram](factory_method.png)

## Example
In example we have a `Factory` trait which has a method that creates a new phone struct implementing `Phone` trait,  which variant is basing on provided `Specification` object. There are two implementations of `Factory` trait: `AppleFactory` and `GoogleFactory`. There are several types of phones, all of which implement `Phone` trait.