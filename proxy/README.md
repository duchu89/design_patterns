# Proxy design pattern
Pattern type: Structural

## Definition
Provide a surrogate or placeholder for another object to control access to it.

## Details
Proxy is about controlling an access.

![Proxy diagram](proxy.gif)

We can define a diffrent types of proxes:
\* Virtual Proxy - controls access to a resource which is expensive to create. It delays creation of object until it's really required.
\* Protective Proxy - passes only requests from authorised clients.
\* Remote Proxy - used for resources which are "outside". Proxy is responsible for dealing with connection to that remote resource.
\* Smart Proxy - provides additional layer of security by interposing specific actions when the object is accessed. An example can be to check if the real object is locked before it is accessed to ensure that no other object can change it.

## Example
In example we've implemented a virtual proxy. `ExpensiveObjectProxy` is responsible for delaying creation of `ExpensiveObject` until client calls `process` method on it's instance.