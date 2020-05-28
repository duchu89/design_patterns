# Template method design pattern
Pattern type: Behavioral

## Definition
Define the skeleton of an algorithm in an operation, deferring some steps to subclasses. Template Method lets subclasses redefine certain steps of an algorithm without changing the algorithm’s structure.

## Details

![Template method diagram](template_method.png)

Template Method is a behavioral design pattern that lets one define an “invariant” steps or program flow in an algorithm. Invariant means cannot be changed, it is the same. The subclasses of the template can redefine the behavior of the algorithm.

## Example
