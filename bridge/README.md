# Bridge design pattern
Pattern type: Structural

## Definition
Decouple an abstraction from its implementation so that the two can vary independently.

## Details
This pattern lets you split a large class or a set of closely related classes into two separate hierarchies—abstraction and implementation—which can be developed independently of each other.

![Bridge diagram](bridge.png)

There are 2 parts in Bridge design pattern :
\* Abstraction
\* Implementation

The bridge pattern allows the Abstraction and the Implementation to be developed independently and the client code can access only the Abstraction part without being concerned about the Implementation part.
The abstraction is an interface or abstract class and the implementor is also an interface or abstract class.
The abstraction contains a reference to the implementor. Children of the abstraction are referred to as refined abstractions, and children of the implementor are concrete implementors. Since we can change the reference to the implementor in the abstraction, we are able to change the abstraction’s implementor at run-time. Changes to the implementor do not affect client code.
It increases the loose coupling between class abstraction and it’s implementation.

It's similar to Strategy pattern though Strategy is behavioral pattern and Bridge is Structural. While Strategy is used to switch between separate implementations (e.g of algorithm) within some context (abstraction), Bridge allows to change interfaces and abstractions independently.

## Example
We are building an universal pilot which can be used to control various devices (TV, Radio). Our abstraction is `Remote` which has two concrete variants: `BasicRemote` and `ProRemote`. Implementor is `Device` which is implmeneted by specific device like: `TV` and `Radio`. `Remote` contains an reference to implementation of `Device`.