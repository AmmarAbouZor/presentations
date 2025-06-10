# Compiler and type-system:

## new-type idiom
Example in draft is good.

**Group Argument won't be included**

## Favor Enums over Booleans:
I can start with a function having two booleans as arguments and then show the change by using enums instead.
Then I can show the case were we can even bake the struct into the enum itself.

## State pattern:

Example with the plan is good.
I need to provide a neutral methods like `seats_count()`.

## Exhaustive pattern matching:
* The example of CLI arguments validation is good.
* Showcase from Chipmunk develop. 

----

# Testing:

## Snapshot testing:

Short recap + Chipmunk example.

## Fuzzy & Prop testing:

* Concept of fuzzy testing with small example.
* Support in rust echo system with cargo-fuzz and proptest
* Example from Chipmunk.

----

# Invariant testing

Show examples where this can be used.
* Fetch api with the provided count
* Example where we need the testing in our function.

