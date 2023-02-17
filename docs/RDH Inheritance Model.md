# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

RDH Inheritance Model.md  
Updated February 17, 2023

IMPORTANT NOTE: THIS IS A WORK IN PROGRESS. Please read documentation carefully to determine what,
                if anything, is considered permanent.

NOTES:
* Examples below are presented in terms of the fictional type hierarchies Character and Vehicle.
  They are not meant to be taken as literal buildable code.

## Overview

The Rust Development Hierarchy (RDH) is intended to provide a framework through which 1) developers
who are already familiar with consuming the inheritance features within programming languages such
as C++ and C# can, in as familiar a manner as possible, *consume* Rust-based type hierarchies, and
2) developers with extensive experience in building type hierarchies within such inheritance-based
languages can *produce* similar type hierarchies within Rust. While many of its aspects map well to
inheritance terminology commonly found within those languages, as this framework is built to extend
Rust, terminology often reflects the world of Rust as well. This document is therefore written for
an audience of developers who at minimum already understand, for example, what an interface is, but
who may not yet know the ways in which a Rust trait is, and is not, like an interface.


## Inheritance Mechanism

To start building an understanding of how RDH achieves type inheritance, it's important first to
understand the concept of a trait in Rust. Traits define behaviors that can be manifested within
types. While this can take a form similar to an interface within a language like C#, traits can be
used for other purposes as well. A trait in another form could behave more like an abstract base
class, a C#-like attribute with arguments, a simple flag that can be used elsewhere to control code
flow, or any number of other useful structures. Traits truly are the workhorses of the Rust
Programming Language! As such, traits are often the focus of code even over concrete types (e.g.
classes).

One example of this lies within Rust's *native* approach to inheritance. Although it's commonly
believed by those unfamiliar with the language that Rust doesn't support inheritance at all, it
does, in fact, support a limited form of inheritance via "supertraits," i.e. traits that encompass
other traits. When combined with default method implementations, this can begin to approximate some
measure of type inheritance as well. Within the bounds of what's supported natively by Rust,
however, a true type hierarchy such as those routinely developed within inheritance-based languages
can't be modeled. Despite this, understanding Rust's built-in inheritance support is critical to
understanding how RDH operates.

In languages such as C#, classes (i.e. types or concrete types in Rust) inherit from one another.
They may or may not implement interfaces, and those interfaces may or may not extend other
interfaces. C++, on the other hand, lacks a built-in concept of an interface altogether. *All* such
entities are classes! Because Rust's native inheritance is attached to traits rather than to types,
however, RDH also approaches inheritance in an interface-first manner. Consequently, *every*
concrete type has a corresponding public interface trait that can be used to abstractly represent
that type and any type that *encompasses* that type (more on that in a moment). Whereas in C# an
interface *may* extend a more highly abstract interface, here each interface *is* a supertrait of
at least one more highly abstract interface. Also, whereas in C# each class may extend a more
abstract class, here each type implements its corresponding interface and all dependencies and may
hold a concrete instance of a more abstract type via composition.

While the mantra "favor composition over inheritance" is commonly encountered when reading about
Rust, the debate proposed by that statement is essentially moot. In Rust, types *can't* inherit
from one another. The only way for an instance of a type to include data from a more abstract type
(beyond straight-up duplication anyway) is via composition. This means that whereas one may think
in terms of a class inheriting from, extending, or being derived from another class in a different
language, within Rust each type can only *encompass* an *instance* of a more abstract type. While
RDH provides the illusion of type inheritance, it's important to keep in mind that underneath the
covers everything is still based upon composition, not inheritance. The distinction between the two
becomes critically important within the subject of type casting, discussed below.

For the sake of clarity, it's also important to keep in mind that in addition to the public
interface traits implemented by each concrete type, several other traits are implemented behind the
scenes. These include those necessary to separate the implementations of virtual functions from
those of non-virtual functions, those needed to support type introspection, those necessary to
support casting from a public interface back to the correct concrete type, and those required to
support casting between public interfaces. These are covered in greater detail within the document
"Extenders' Handbook.md."


## Access Modifiers

While some of Rust's visibility scopes line up nicely with the access modifiers in common
inheritance-based languages, critically important gaps remain.

Using C# as a representative for inheritance-based languages:

| Accessible Scope                        | C#                   | Rust             | Notes                                                                                |
| --------------------------------------- | -------------------- | ---------------- | ------------------------------------------------------------------------------------ |
| Universal                               | `public`             | `pub`            | (none)                                                                              |
| Local                                   | `private`            | (nothing)        | "private" is always implicit in Rust<br />"Local" in Rust is a module, in C# a class |
| Current unit                            | `internal`           | `pub(crate)`     | "Unit" in Rust is a crate, in C# an assembly                                         |
| Derived classes                         | `protected`          | N/A              | Rust has no native concept of derived types                                                 |
| Current unit -or-<br /> Derived classes | `protected internal` | N/A              | Rust has no native concept of derived types                                                 |
| Self and parent                         | N/A                  | `pub(super)`     | C# access is always by class or assembly                                             |
| Self and (some) ancestors               | N/A                  | `pub(in <path>)` | C# access is always by class or assembly                                             |

Those familiar with inheritance-based languages will undoubtedly have noticed that Rust has no way
of modeling the protected or protected internal access modifiers! While this gap in parity makes
perfect sense when you consider that Rust has no concept of type inheritance, that there appears no
way to model such visibility held quite an impact on RDH architecture. RDH developers were
essentially left with a choice: change elements that would normally be *protected* into
*pub(crate)* and require developers to extend the RDH framework directly within a fork of its
crate, or make those elements *public* and allow developers to maintain their own crates. Because
one of RDH's design goals was to make its use as comfortable as possible to developers familiar
with implementing type hierarchies in other languages, the latter model was adopted. This allows
developers to extend the RDH framework within their own crates, but it has a nasty side effect.
This side effect is discussed within "Extenders' Handbook.md" and in Issue #2 within "Knowledge
Base.md."


## Abstract Representations and Type Casting

Abstractions inherently obfuscate details from those who leverage them. A developer who invokes the
GetType() method for any given .NET object knows they will receive a Type instance that represents
the type of that object. They have no insight into how the .NET Framework maps instances to their
types, nor how the returned Type object is instantiated in response to their request. The GetType
method acts as a *black box* that produces predictable output from a known set of inputs. The
developer doesn't need (and generally doesn't want) to know how the black box operates. So long as
its behavior is predictable and performant, *how* this black box accomplishes its task is
immaterial.

As developers, we rely on abstractions far more often than we may be aware. While the form and
function of abstraction within the above example is fairly obvious, many of the abstractions we
leverage on a daily basis are so well hidden we simply take them for granted. One such abstraction,
so well hidden it's practically invisible, lies in *casting*. Casting is an incredibly common
operation when working with a type hierarchy. Groups of objects are commonly held within
collections as instances of their least abstract common type. These collections are often processed
by functions that only invoke operations defined at that level of abstraction or higher, but not
always. Sometimes additional processing may be required for some objects depending upon one or more
of their less abstract types. For example, all humanoid characters in a video game may be held
within a single collection. During processing, all such characters may be instructed to proceed
with their current animations, but *player* characters may require special processing to handle
input events that may interrupt the animations they had been performing up to this point. The
ability to know whether that special processing must be performed for any given ICharacter instance
depends upon the ability to cast that object to another type, e.g. *icharacter as*
*IPlayerCharacter*.

Casting is such a common operation that we invoke it without any thought whatsoever. Developers
familiar with inheritance-based languages in particular are likely no more surprised by the ability
to cast an interface instance (i.e. a trait object in Rust) to a less abstract interface than they
are by the ability to initialize a variable. As both operations feel equally *primitive*, it's easy
to simply assume that all languages must fully support both. Unfortunately, this is not universally
true! As learned during the development of the RDH proof-of-concept, not all languages support all
seemingly obvious casting scenarios. Casting *feels* like a primitive not only because it's so
commonly used but also because its syntax is deceptively simple. An extensive abstraction hides
beneath the expression *icharacter as IPlayerCharacter* that makes it feel as though casting simply
works, like *magic*.

When working with languages that inherently support such operations, how casts are performed
doesn't matter. Because Rust lacks adequate native support for most of the casting scenarios
required within type hierarchies, however, the burden of such support must be handled by the RDH
framework. The Thaumaturgy module in the rdh crate provides casting support to the furthest extent
and with the simplest syntax RDH developers could manage. (Implementing this support was more
complicated and time consuming than all other aspects of this project combined, including
documentation!) The sections below discuss the *types* of casts that are supported as well as the
types that are not supported, why they're not supported, and how to function within those
limitations. Sections following this detail which casting *scenarios* are supported and how to
invoke them as well as which casting scenarios can't be supported, why they can't be supported, and
how to function within *those* limitations.


### Types of Casts

Again using C# as a representative inheritance-based language, we can see that casting takes
several forms:

* Explicit C++-style casting
  * Throws an exception if the requested operation is invalid
  * For example, `var iplayerCharacter = (IPlayerCharacter)character`
* Try-style casting
  * Sets iplayerCharacter to null if the requested cast is invalid
  * For example, `var iplayerCharacter = character as IPlayerCharacter`
* Casting queries
  * While not a casting operation per se, the *is* keyword may be used to determine whether
    requesting a cast would succeed
  * For example, `if (character is IPlayerCharacter)...`
* Coercion by function parameters
  * Throws an exception if the requested operation is invalid
  * For example, sending a (Character) instance to a function with the signature
    `void UpdatePlayerAnimation(IPlayerCharacter character)`

These are discussed in turn within the sections below.


#### Explicit C++-style casting

Because Rust doesn't support C++ and C#-like exceptions, the only way to interrupt normal code flow
is through a panic! call. As invoking panic! would preclude the possibility that the error could be
handled more gracefully than outright crashing the application, we chose to not model RDH casting
operations in this manner. Instead, if the developer opts not to first test to see whether the
requested cast would be valid, they'll have to check the result of the casting call for an error
before proceeding.


#### Try-style casting

Despite Rust's lack of a *null* value, most casts within RDH are of this type. Such calls return
`Option<&T>` (or `Option<&mut T>`) where T is the type requested by the casting call. This
`Option<&T>` instance either contains the type instance as an instance of the requested type (via
*Some*) or it contains no instance (via *None*). (Discussing the various ways in which the
`Option<&T>` instance can be unpacked is out of scope for this document. Documentation may be found
within Rust's website, and various examples are available in function headers and other documents
within this project.)

The one exception to this lies in casts that can't fail at runtime. Both casts from a concrete
instance to an instance of an implemented interface, as well as casts from an interface instance to
a more abstract interface instance (i.e. an "upcast") simply return `&T`. Such casts will always
succeed at runtime, as any possibility of failure would result in a compiler error.


#### Casting queries

Note: As this section is intended to emphasize concept over syntax, example calls are pseudocode.
Actual call syntax may be found within API reference documents.

Although RDH supports casting queries, the differences between Rust and inheritance-based languages
complicates doing so. In languages such as C#, the result of a casting operation and the instance
for which the cast was requested both refer to the same object at the same place in memory. There,
a *Character* instance and the instance returned from successfully casting it to a
*PlayerCharacter* instance both point to the exact same memory address. It's important to remember,
however, that because Rust doesn't natively support type inheritance, a PlayerCharacter instance
*encompasses* an instance of the Character class. While they both refer to the same entity on a
*conceptual* level, the PlayerCharacter instance and the Character instance it contains are, in
fact, *different* objects at *different* locations in memory. This distinction creates the need to
support an *implements* query in addition to *is*. Both are implemented within the Thaumaturgy
module's Divination component.

For an RDH instance, a call to *is* will only return true if the underlying concrete type literally
*is* the queried type. Whereas in C# a call such as *if (character is PlayerCharacter)...* would
return true whether the character variable was initially declared as a PlayerCharacter or, say, a
LocalPlayerCharacter, that same call within the RDH framework would return true *only* when the
character variable was initially declared as a PlayerCharacter. Had the character variable been
initially declared as a LocalPlayerCharacter, the call would return false. While the reasoning
behind this behavior might be difficult to grasp at first, it becomes clearer when you consider
what the *is* call is actually asking: "Is what lives at that memory address actually a
PlayerCharacter?" In C# the answer would be "yes" because its inherent support for inheritance
allows the PlayerCharacter portion of a LocalPlayerCharacter to reside at the same address in
memory. Because RDH can only accomplish the *illusion* of inheritance by providing an abstraction
around Rust's use of composition, here the answer is "no"-- what lives at that memory address is
actually a LocalPlayerCharacter. Its PlayerCharacter portion resides elsewhere. This also may be
easier to understand when you consider the casting call that may follow this query. If RDH said
"yes" in this scenario but the following cast to PlayerCharacter failed because that memory address
actually contains a LocalPlayerCharacter instance, people would be justifiably confused.

It also helps that a simple workaround is provided. While *is* may be called to be certain that
casting an interface instance to a concrete type will succeed, it's often useful to know whether
an object may also be an instance of another abstract type. This is where *implements* comes in.
Calls to *implements* return true if an instance's concrete type implements the specified
interface. In the example above, while an RDH call such as *if character is PlayerCharacter...*
would return false, a call such as *if character implements IPlayerCharacter* would return true.
Notice that the latter test specified an interface rather than a concrete type. Whereas *is*
queries an instance's actual concrete type, *implements* queries whether a specific *other*
interface may be used to abstractly represent that same instance.


#### Coercion by function parameters

Because function parameter coercion is handled directly by the Rust compiler, it's unfortunately
not something RDH can control. As such, coercion will only directly work in casting scenarios that
Rust can natively support, as detailed in the sections below. In all other scenarios, the instance
must be cast via RDH before being passed to the function.

There is a silver lining, however. Because Rust generates a compiler error for any coercion
operation that may fail, one can be assured that coercion will never result in a runtime error. The
same cannot be said for C++, C#, or other inheritance-based languages.


### Casting scenarios

Using C++ as a representative inheritance-based language (and keeping in mind that an interface in
C++ is, in fact, a pure virtual abstract base class), all casting operations may be seen as
belonging to one of the following scenarios:

* Concrete-to-concrete
  * Casts an instance represented by a concrete type variable to another qualifying concrete type
  * For example, `auto car = (Car)sedan`
* Concrete-to-interface
  * Casts an instance represented by a concrete type variable to a qualifying interface type
  * For example, `auto iCar = (ICar)sedan`
* Interface-to-concrete
  * Casts an instance represented by an interface type variable to a qualifying concrete type
  * For example, `auto sedan = (Sedan)isedan`
* Interface-to-interface
  * Casts an instance represented by an interface type variable to a qualifying interface type
  * For example, `auto iSedan = ISedan(icar)`

Within C++, each of the above may be used to obtain an instance of a type more abstract or less
abstract than the current type. Casts that yield more abstract types are referred to as upcasts,
whereas casts that yield less abstract types are referred to as downcasts.

Before reading the sections below, it's important to understand that much within these casting
scenarios is not easily modeled in Rust, and some can't be modeled at all!


#### Concrete-to-concrete

Within inheritance-based languages, casting between concrete types is incredibly common! Whether
explicitly performed via *as* invocations or implicitly via function parameter coercion,
representing an instance of a type as an instance of a more abstract, yet still concrete, type (or
reversing this) is beyond routine. Given this, it may be surprising to hear that RDH developers
decided to offer no support for this casting scenario and that they have no plans to ever do so.
While this decision may be baffling for some, for those more familiar with Rust it will likely make
perfect sense.

Because Rust lacks native support for type inheritance, the more abstract aspects of each type are
held by composition. Whereas in C++ a Sedan instance and the Car instance that results from casting
the Sedan to a Car type would both point to the same object at the same location in memory, in
Rust, the Sedan instance and such a Car instance would point to *different* objects at *different*
locations in memory. This holds several ramifications.

  In increasing order of severity:

  1. Developers familiar with inheritance-based languages but new to Rust may struggle to keep in
     mind that the resulting instance would *not* be the same object, but instead a completely
     different object. This gap in understanding could lead to all kinds of chaos.

  2. Nothing would tie the instance yielded from the cast back to its encompassing instance. There
     would be no (good) way to determine later which Sedan instance yielded this Car instance, or
     whether it was yielded from a Sedan instance at all, or even whether the instance was
     initially declared as a Car type, a Sedan type, or some other less abstract type. Because of
     this, it would be impossible to cast the instance back to its initial type.

  3. This would break polymorphism! As nothing would tie the Car instance back to its encompassing
     Sedan instance, calling a virtual function on the Car instance would invoke the implementation
     from the *Car* type, *not* the Sedan type. So, for example, while a call such as
     *car.show_type()* would be expected to yield "Sedan," it would instead yield "Car." This would
     clearly be buggy behavior.

While all the above may explain why no support is currently offered for this casting scenario, it
may still be surprising that we have no intention of ever offering such support, or even
investigating options for doing so. It certainly seems possible to find a way to tie an instance
back to its encompassing object, perhaps even in a reasonably performant and resource-friendly way.
Given that anything that could be achieved via concrete-to-concrete casting can be achieved at
least as performantly through other casting scenarios, however, there's simply no point in spending
further cycles on this issue.


#### Concrete-to-interface

Casting a concrete instance to an instance of one of the interfaces it implements is thankfully
straightforward! This is also the only casting scenario fully natively supported within Rust's
Stable Channel, albeit with (possibly) less comfortable syntax. Further, this is the only scenario
supported within function parameter coercion-- all other conversions must be made through explicit
casts before the function is invoked.

##### *Direct Invocations*

  Example casts from C#:
  
  *mutable*
  * `var isedan = sedan as ISedan;`
  * `var icar = sedan as ICar;`

  Corresponding RDH casts (via the Thaumaturgy module's Transmutation component):
  
  *immutable*
  * `let isedan = sedan.as_isedan();`
  * `let icar = sedan.as_icar();`

  *mutable*
  * `let isedan_mut = sedan_mut.as_isedan_mut();`
  * `let icar_mut = sedan_mut.as_icar_mut();`

  Corresponding native Rust casts:
  
  *immutable*
  * `let isedan = &sedan as &dyn ISedan;`
  * `let icar = &sedan as &dyn ICar;`
  
  *mutable:*
  * `let isedan_mut = &mut sedan_mut as &mut dyn ISedan;`
  * `let icar_mut = &mut sedan_mut as &mut dyn ICar;`

  Both the RDH and native Rust examples above provide the same outcome with similar performance.
  That being said, those new to Rust would be well served to embrace its native syntax whenever
  possible.

##### *Common Use Cases*

While the direct invocation examples above are perfectly valid, they will not generally be seen
within real world code. Instead, such casts are most commonly performed implicitly when collating
collections of instances that implement a common interface. Returning to the game object hierarchy
example, it would be unsurprising to find something *similar* to the below somewhere in the code
base:

```rust
let players: Vec<Box<dyn ICharacter>> = vec![Box::new(Npc::new()),
                                             Box::new(Npc::new()),
                                             Box::new(PlayerCharacter::new()),
                                             Box::new(LocalPlayerCharacter::new())];
```

When run, the players variable will contain four ICharacter instances, two of which are NPCs, one
of which is a player character not controlled by the console on which the code is being run, and
one of which is the locally controlled player character. (Of course, one would generally create the
actual instances elsewhere.) While no direct invocation syntax is seen, Rust's own native casting
feature is used behind the scenes to populate this collection.


#### Interface-to-concrete

Another casting operation commonly used within type hierarchies involves the conversion of an
interface instance to a concrete type. Within inheritance-based languages, this concrete type may
be the instance's actual declared type or any of its ancestors. As discussed in detail within
previous sections, however, because Rust supports composition but not type inheritance, RDH can
only support casting an interface instance back to its initially declared concrete type, and then
only when that type is known at compile time.

##### *Invocations*

  Example casts from C++:
  
  *mutable*
  * `auto sedan = (Sedan)icar;`  
  * `auto car = (Car)iconstruct;`

  Corresponding RDH casts (via the Thaumaturgy module's Necromancy component):
  
  *immutable*
  * `let sedan = icar.as_concrete::<Sedan>();`
  * `let car = iconstruct.as_concrete::<Car>();`  

  *mutable*
  * `let sedan_mut = icar_mut.as_concrete_mut::<Sedan>();`
  * `let car_mut = iconstruct_mut.as_concrete_mut::<Car>();`


##### *Native Rust Support*

While Rust does natively support casts of this variety via their Any trait, its use comes with a
eighty caveat: all types upon which such casts will be performed must be declared *'static*. Rust
lifetimes are decidedly out of scope for this document, but it's likely already clear that no type
hierarchy could be viable if all of its instances had to be static. Justifications for the 'static
requirement are legitimate, but they reflect the universal domain within which the Any trait must
function. The hope is that supporting these operations within narrower confines may alleviate some
of the concerns that led to this requirement. (In brief, given that each of the Necromancy
component's methods has a single parameter that expects a reference to an RDH type instance with a
lifetime known to the borrow checker and that each of these functions expresses a single reference
to the same underlying type instance, the lifetime to assign to the expressed reference seems
clear.)

##### *Cast-and-dispatch Patterns*

As shown above, the ability to obtain a concrete type instance from an interface instance hinges
upon knowing the underlying concrete type at compile time. This caveat likely appears more
consequential than it is, however. While it's true that *retaining* such a reference is possible
only if the type is known *a piori*, functions that utilize the cast-and-dispatch pattern may be
able to discover the underlying concrete type at runtime. While the obtained reference couldn't be
returned to the caller, the instance, as its concrete type, can be handled locally or passed onto a
handler function. Casts discussed within the next section, which are possible even when the
underlying concrete type is *not* already known, use this cast-and-dispatch approach behind the
scenes. Specific examples of this pattern in action can also be found in the type_dispatch_examples
module within /rdh_extension_example/src/_extras/.


#### Interface-to-interface

This final casting scenario involves the conversion of an interface instance to an instance of
another interface also implemented by that instance's underlying concrete type, notably whether
that underlying concrete type is known at compile time or not. Such casts fall into one of two
camps: conversion to a more abstract type (i.e. an "upcast"), or conversion to a less abstract type
(i.e. a "downcast"). This distinction is important for a number of reasons. Chief among them, if no
compiler errors are generated, all upcasts will always succeed. Errantly requested downcasts, on
the other hand, will result in errors at runtime. Rust also includes experimental support for such
upcasts within its Nightly Channel. (See https://github.com/rust-lang/rust/issues/65991 for further
information on Rust's upcoming native support, currently flagged "ready to stabilize.") Finally,
although out of scope for this document, it's important to understand that extending downcast
support for new types as they're added to the hierarchy is far more complicated than doing so for
upcasts. While this complexity is hidden by abstractions within the Thaumaturgy module's
Transmutation component such that developers consuming the framework will likely never even know of
 its existence, it still manifests within the resulting code: upcasts are more performant than
downcasts.

The following sections contain only brief descriptions of upcast and downcast functionality. Far
more detailed information can be found within "Extenders' Handbook.md".

##### *Upcasts*

Example casts from C#:

*mutable*
* `var isedan = ifull_size_sedan as ISedan;`
* `var icar = isedan as ICar;`

Corresponding RDH casts:

*immutable*
* `let isedan = ifull_size_sedan.as_isedan();`
* `let icar = isedan.as_icar();`

*mutable*
* `let isedan_mut = ifull_size_sedan_mut.as_isedan_mut();`
* `let icar_mut = isedan_mut.as_icar_mut();`

Corresponding native Rust casts (Nightly Channel only):

*immutable*
* `let isedan = &ifull_size_sedan as &dyn ISedan;`
* `let icar = &isedan as &dyn ICar;`

*mutable*
* `let isedan_mut = &mut ifull_size_sedan_mut as &mut dyn ISedan;`
* `let icar_mut = &mut isedan_mut as &mut dyn ICar;`

As upcasts always succeed, all of the above yield a bare reference to the requested type.

Both the RDH and (experimental) native Rust examples above provide the same outcome with similar
performance. That being said, those new to Rust would be well served to embrace its native syntax
whenever possible.

##### *Downcasts*

Example casts from C++:

*mutable*
* `auto ifull_size_sedan = (IFullSizedSedan)isedan;`
* `auto isedan = (ISedan)icar;`

Corresponding RDH casts:

*immutable*
* `let ifull_size_sedan = isedan.as_ifull_sized_sedan();`
* `let isedan = icar.as_isedan()`

*mutable*
* `let ifull_size_sedan_mut = isedan_mut.as_ifull_sized_sedan_mut();`
* `let isedan_mut = icar_mut.as_isedan_mut()`

As downcasts are runtime checked, they may fail. Because of this, all of the above yield an
`Option<&T>` (or `Option<&mut T>`), where T is the type requested by the casting call. This
`Option<&T>` instance either contains the type instance as an instance of the requested type (via
*Some*) or it contains no instance (via *None*). (Discussing the various ways in which the
`Option<&T>` instance can be unpacked is out of scope for this document. Documentation may be found
within Rust's website, and various examples are available in function headers and other documents
within this project.)


### Further Considerations

As demonstrated above, casting in Rust is often unexpectedly different than in many other
languages. There is, however, one additional likely unexpected difference remaining to discuss. In
inheritance-based languages, invoking methods from interface instances affords the same performance
as doing so from concrete type instances. In Rust, however, better performance is afforded by
invoking methods on concrete type instances. While the underlying reasons for this are best
explored within Rust's own documentation, in brief, because interface instances are created at
runtime, calls to their methods are completed via a dynamic dispatch system rather than by the
static dispatch system used for calls to concrete type instances. The performance difference stems
from the need to use a v-table to determine the actual method implementation to invoke for a given
interface instance, a step that isn't necessary for concrete type instances. The net takeaway from
this is that, whenever possible, one should choose to work with concrete type instances rather than
interface instances. Ideally, interface instances should be used if and when doing so is the only
option.
