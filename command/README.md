# Command pattern
Pattern type: Behavioral

## Definition
Encapsulate a request as an object, thereby letting you parameterize clients with different requests, queue or log requests, and support undoable operations.

## Details
This pattern allows us to decouple objects that produce the commands from their consumers, so that's why the pattern is commonly known as the producer-consumer pattern.

![Command diagram](command_uml.gif)

Command declares an interface for all commands, providing a simple *execute()* method which asks the *Receiver* of the command to carry out an operation. The *Receiver* has the knowledge of what to do to carry out the request. The *Invoker* holds a command and can get the *Command* to execute a request by calling the execute method. The Client creates *ConcreteCommands* and sets a *Receiver* for the command. The *ConcreteCommand* defines a binding between the action and the receiver. When the *Invoker* calls execute the *ConcreteCommand* will run one or more actions on the *Receiver*.

## Example
Let's think about Stock Exchange - we can have a different Stocks on our Broker account for which we can send command either for selling them or buying more.
Having a `Command` trait defined, we create a two concrete commands - `BuyStock` and `SellStock`. Each of them takes an concrete `Stock` in constructor. Then we have an `Broker` which stores a list of Commands and executes them. 
