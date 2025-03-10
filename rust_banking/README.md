# Ownership, Borrowing, Lifecycle

## Ownership

The goal ownership is to limit the ways you can reference and change data, this limitation will reduce the number of bugs + make your code easier to understand.

- Multiple things can refer to a value at the same time but they are all read-only.
- A value can only be updated when there are no read-only references to it.
- Every value is 'owned' by as single variable at a time.
- Reassigning the value to another variables moves the value.

## Borrowing

- You can create many (immutable) read-only references to a value that exist at the same time.
- You can't move a value while a ref to the value exists.
- You can make a writable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time.
- You can't mutate a value through the owner when any reference to the value exists.
- Some type of values copied instead of moved (numbers, bools, chars, arrays/tuples that has copyable elements).

## Lifetimes

Lifetime: How long reference or owner exists.
Generic Lifetime ( Lifetime annotations ): Extra syntax added in to clarify relationship between different lifetimes.

- When an owner goes out of scope, the value owned by it dropped (cleaned up in memory).
- There can't be references to a value when its owner goes out of scope.
- References to a value can't outlive the value they refer to.

## General Advices

Need to store the argument ?
- Take ownership
Need to do a calculation or read operation ?
- Take immutable (read-only) reference.
Need to change the value in some way ?
- Take mutable (writable) reference.
