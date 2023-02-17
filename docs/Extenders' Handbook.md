# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

Extenders' Handbook.md
Updated February 6, 2023

IMPORTANT NOTE: THIS IS A WORK IN PROGRESS. Please read documentation carefully to determine what,
                if anything, is considered permanent.

## Overview

The Rust Development Hierarchy (RDH) is intended to provide a framework through which 1) developers
who are already familiar with consuming the inheritance features within programming languages such
as C++ and C# can, in as familiar a manner as possible, *consume* Rust-based type hierarchies, and
2) developers with extensive experience in building type hierarchies within such inheritance-based
languages can *produce* similar type hierarchies within Rust. While many of its aspects map well to
inheritance terminology commonly found within other languages, extending the framework it provides
necessarily requires a working knowledge of Rust programming as well. This document is therefore
written for an audience of developers who have prior experience in coding within both inheritance-
based languages and Rust itself. For example, developers should understand the difference in Rust
between traits, trait objects, and type instances, as well as the situations in which generics are,
and are not, allowed and the ways in which macros can be used to work around these limitations.
While familiarity with lifetimes would also be helpful, a surface-level understanding should
suffice.


## Crate Manifest

While this repo includes several crates, only one is of fundamental importance. The remainder serve
to provide examples, templates, and documentation.

### rdh crate

At its core, RDH consists of a single crate, called rdh. Everything required to provide the basis
of any type hierarchy may be found within its *construct* and *thaumaturgy* modules.

#### construct module

The construct module contains the public interface trait and public struct that serve as the root
of the RDH type hierarchy (IConstruct and Construct respectively). For reasons detailed below, the
IConstruct trait is broken up into several smaller traits, all of which are defined within either
this module or the thaumaturgy module. Finally, a macro whose output is used to grant access to the
encompassed concrete instances of each encompassing type may be found at the bottom of the module's
file.

Manifest:

  * Intended for Public use:
    * IConstruct (pub trait)
      * The public interface for the root RDH type
      * Contains non-virtual methods
      * Aggregates support traits into a single interface
    * Construct (pub struct)
      * The root RDH type
      * Intended to eventually approximate the functionality of the Object class in .NET

  * Intended for Internal use:
    * CONSTRUCT_TYPE_UUID and ICONSTRUCT_TYPE_UUID (pub(crate) const)
      * Unique identifiers for the Construct type and IConstruct interface, respectively
      * Used internally for type introspection

  * Intended for Protected use:
    * ConstructVirtuals (pub trait)
      * Contains virtual methods
      * Aggregated into the IConstruct interface
    * ConstructInstances (pub trait)
      * Contains declarations of methods through which actual Construct instances may be obtained
      * Created from an invocation of the define_instances macro
    * define_instances (exported macro)
      * Invoked within the construct module and the modules of all encompassing types to grant
        access to the their encompassed concrete instances

  * Created by an invocation of the thaumaturgy module's provision_transmutation macro; intended
    for Protected use:
    * UpcastsToIConstruct (pub trait)
      * Contains methods through which instances of less abstract types and interfaces can be
        upcast to the IConstruct interface
      * Aggregated into the IConstruct interface
    * ConcreteDivinator (pub trait)
      * Contains methods through which information about the underlying concrete type may be
        obtained from an interface instance
      * Aggregated into the IConstruct interface

The keen observer may have noticed that several elements labeled as "intended for protected use"
are, in fact, defined as public entities. This is necessary because no method of modeling the
"protected" access modifier through Rust's visibility scopes is currently known. (See the "Access
Modifiers" section in "RDH Inheritance Model.md.") The factors that forced each of these elements
into public view fall into one of two categories, the first unfortunately rather nasty, but the
second thankfully innocuous.

The ConstructInstances trait (along with the exported define_instances macro that creates it)
belongs to the "rather nasty" category. For the RDH project to meet its initial goals as well as
possible, new encompassing types must be definable within separate libraries. Given that Rust is
unable to model protected access, the only available option through which this goal could be
retained was to expose these entities publicly. Unfortunately, doing so allows consuming code to
obtain concrete type instances that should not be separable, leading to all of the problems
discovered during the investigation of concrete-to-concrete casting (as described within the
corresponding section within "RDH Inheritance Model.md"). As an example, the call `let construct =`
`vehicle.construct()` would succeed. The construct variable would receive a reference to the
Construct instance encompassed by the Vehicle instance, and polymorphism would be broken for
virtual functions invoked on that instance. Please see Issue #2 within "Knowledge Base.txt" for
further details on the implications of this limitation.

Reason for separation:
  * ConstructInstances grants IConstruct's non-virtual method implementations access to the
encompassed Construct instance in both its immutable and mutable forms. Because non-virtual methods
behave the same for all RDH types, only a single IConstruct implementation is defined. Such
singular implementations are agnostic of the underlying type to which they're applied. In contrast,
the ConstructInstances trait is implemented for all encompassing types individually. Because all
encompassing types include a Construct instance, IConstruct is able to leverage the
ConstructInstances trait to obtain access to the encompassed Construct instance.

While those with a visceral reaction to coupling may find this architecture somewhat distasteful,
it reflects the very nature of type hierarchies, even more so when expressed through Rust's trait-
based inheritance model. Every type that encompasses the Construct type holds a Construct instance
and can be abstractly represented by an instance of the IConstruct interface. Therefore, very much
by design, there can *never* be an IConstruct instance that doesn't correspond to a Construct
instance, anywhere in the code base. Among other advantages, this allows us to forego the frankly
overwhelming boilerplate that adding accessor and mutator functions to every encompassing type for
every protected and public field in the hierarchy would otherwise require.


In contrast, The ConstructVirtuals, UpcastsToIConstruct, and ConcreteDivinator traits all belong to
the innocuous category. Because Rust requires the same visibility for all traits within a
"supertrait", all traits aggregated into IConstruct must be public before IConstruct itself can be
made public. Methods defined within these smaller traits are incorporated into the methods
IConstruct exposes to consuming code. While the distinction between the public interface and its
component traits is therefore *almost* seamless, it remains unfortunately possible for consuming
code to cast a concrete instance to one of these traits (e.g. `let virtuals = &construct as &dyn`
`ConstructVirtuals`). Doing so holds no serious negative implications, however. Code using an
instance from such a call would be restricted to invoking only methods defined within the
ConstructVirtuals trait. Also, because the resulting instance wouldn't qualify as an RDH type, no
framework operations could be performed upon it, including casts. The variable would be, in effect,
at a dead end. While potentially confusing, this behavior can't cause any serious problems.

Reasons for separation:
  * ConstructVirtuals: Virtual methods must be implemented separately for all encompassing types,
    generally to add additional behavior to the behavior of their parent type. In contrast, non-
    virtual methods are implemented only once, yielding identical behavior for all encompassing
    types. Initially, RDH developers had hoped to house both method varieties within the same
    trait, but Rust provides no support for method overrides or for partial trait implementation.
    While its support for default implementations for a subset of methods within the trait
    declaration itself looked promising at first, no means could be found for non-virtual methods
    to obtain access to the underlying concrete instance when implemented in this manner. Virtual
    and non-virtual methods are therefore held within separate traits.
  * UpcastsToIConstruct: The methods defined within this trait exist only to establish casting
    functionality that would be provided by the compiler in languages with native support for type
    hierarchies. While these methods could have been incorporated directly into IConstruct, they
    were placed into their own trait both to reinforce separation of concerns and to enable the
    trait to be both declared and implemented as part of a call to the provision_transmuation macro
    rather than manually.
  * ConcreteDivinator: As with UpcastsToIConstruct, this trait and the methods it defines exist
    only to establish functionality that would otherwise have been furnished by the compiler. While
    these methods could have been incorporated into ConstructVirtuals, they were placed into their
    own trait both to reinforce separation of concerns and to enable the trait to be implemented as
    part of a call to the provision_transmuation macro rather than manually for each encompassing
    type.


#### thaumaturgy module

The thaumaturgy module contains all the "magic" necessary to support casting a given concrete RDH
type to one of the interfaces it implements and back again, as well as from one implemented
interface directly to another. While such functionality may be taken for granted within an
inheritance-based language, most of the RDH development effort was invested in its near-
duplication, results of which are shown within the table below. Please see "RDH Inheritance
Model.md" for an in-depth analysis of this topic.

| Casting Scenario                     | Native Rust Support               | RDH Support      | Notes                                             |
| ------------------------------------ | --------------------------------- | ---------------- | ------------------------------------------------- |
| Interface-to-interface (upcasting)   | Yes (Nightly Channel only)        | Yes              | Rust support undergoing stabilization             |
| Interface-to-interface (downcasting) | No                                | Yes              | Sizable RDH boilerplate built via macro           |
| Interface-to-concrete                | Kinda (only if everything static) | Yes              | No known problems with RDH as non-static          |
| Concrete-to-interface                | Yes                               | Yes              | RDH syntax more C#-like; Rust syntax more "Rusty" |
| Concrete-to-concrete                 | No                                | No               | RDH workaround: Cast to target's public interface |

As indicated within the above table, Rust currently supports concrete-to-interface casts out of the
box and upcasts within its Nightly Channel. While Rust does support additional casting scenarios,
such support is limited to static data. Because a type hierarchy where every element must be static
is untenable, RDH includes a bespoke version of this functionality, but only for the greatly
constrained domain of RDH types. Such functionality includes the means to acquire a concrete type
instance from an interface instance, along with the type introspection operations required to do
so. Those casting scenarios already covered sufficiently by Rust in either Channel are also
reimplemented to provide a common lexicon across all casting operations. Finally, while the lack
of concrete-to-concrete casting may at first seem problematic, as shown within inheritance model
documentation, other casting operations can be used as necessary to facilitate the same scenarios.


Manifest:

  * Divination: The power to know the unknowable (module component)
    * Provides rudimentary type introspection

    * Divinator (pub trait)
      * Used to obtain the unique identifier for an RDH public interface or concrete type along
        with the unique identifier for the library in which it's defined
      * As this is implemented globally for all RDH types, it behaves as though aggregated into the
        IConstruct interface, the concrete Construct type, and all encompassing interfaces and
        types
    * ConcreteDivinator (pub trait)
      * Similar to Divinator but addresses concrete types only (i.e. the type of a concrete
        instance itself or the type of the concrete instance that underlies an interface instance)
      * Used to obtain the unique identifier for concrete types along with the unique identifier
        for the library in which they're defined.
      * Also supports operations to determine the actual underlying concrete type as well as
        whether that concrete type implements a specified interface
      * Implemented for each type as it's added to the hierarchy; behaves as though aggregated into
        the IConstruct interface, the concrete Construct type, and all encompassing interfaces and
        types
    * TypeRegistry (pub singleton struct)
      * Provides functions through which a specified RDH type's unique identifier or the unique
        identifier of the library in which it's defined may be obtained
      * Accessible either directly or via the Divinator and ConcreteDivinator traits
    * TypeIdentifier (pub struct)
      * Represents the unique identifier for an RDH public interface or concrete type
      * Used as an atomic token (i.e. an "opaque blob") when comparing instance types
    * LibraryIdentifier (pub struct)
      * Represents the unique identifier for the library in which an RDH public interface or
        concrete type is defined
      * Used as an atomic token (i.e. an "opaque blob") when determining the library to which a
        downcast request will be routed (see the "Transmutation: provision_transmutation macro"
        section, below)

    Note: That both TypeIdentifier and LibraryIdentifier are backed by a u128 is not meant to
          endorse 18.5 quintillion as a reasonable number of hierarchy types or RDH libraries for
          any application! :) Rather, this ultrawide possible range of identifiers (the same as
          with UUIDs/GUIDs in other languages) is intended to ease parallel development and
          integration work across large teams with multiple dependencies, as randomly selected u128
          values are exceedingly unlikely to conflict.


  * Necromancy: The power to raise the dead (module component)
    * Provides type UN-erasure when the underlying concrete type is known at compile time

    * Necromances (pub trait)
      * Used to obtain a reference to the actual concrete instance that underlies a given interface
        instance, in either immutable or mutable form
      * As this is implemented globally for all RDH types, it behaves as though aggregated into the
        IConstruct interface, the concrete Construct type, and all encompassing interfaces and
        types
    * Necromancer (pub struct)
      * Provides functions through which the actual concrete instance that underlies a given
        interface instance can be obtained, in either immutable or mutable form
      * Accessible either directly or via the Necromances trait


  * Transmutation: The power to transform (module component)
    * Provides interface-to-interface casting-- regardless of whether the underlying concrete type
      is known at compile time

    * provision_transmutation (exported macro)
      * The macro through which interface-to-interface casting support may be quickly and easily
        implemented as each new RDH framework type is added
    * extend_downcasting (exported macro)
      * The macro through which new types may be added to the list of underlying concrete types
        that may handle downcasting between certain interfaces across library boundaries

Of these three components, Transmutation is far more complicated and therefore difficult to
understand than either Divination or Necromancy. Following a brief explanation of Rust trait
objects and type erasure, remaining discussion of the rdh crate explores this complexity. Please
see "BASE API Reference for RDH Consumers.md" for call-level documentation of the Divination and
Necromancy components as exposed through the IConstruct interface.


##### Trait objects and type erasure

To understand how RDH supports the casting scenarios listed above, it's necessary to first
understand the structure of Rust trait objects (e.g. interface instances). A trait object consists
of two pointers: one to the concrete instance to which it corresponds and the other to a
dynamically created v-table for methods defined within the trait it represents. This v-table
contains mappings from each of these methods to the memory address of the implementation specific
to the concrete instance's type. When called, the correct method implementation is retrieved and
then invoked, with the underlying concrete instance supplied to its Self-typed parameter. Notice
that the actual *type* of the underlying concrete instance is stored nowhere within this structure.
Referred to as type erasure, this absence greatly complicates casting support implementation.

Within interface-to-concrete casts, the RDH framework overcomes type erasure quite easily, at least
from the framework's point of view! Necromancy component calls require the caller *themselves* to
specify the underlying concrete type. The reasons for this are complicated and mostly out of scope
for this document. In brief, however, the intention behind a Necromancy component call is to
retrieve and retain the underlying type instance as a reference to a concrete type. Notice that the
concrete instance is to be *retained,* something possible only when the type is known *a priori*.
While Rust's support for type inference may at times absolve a developer from having to know the
data type received from a call, the compiler doesn't get off so easily. For type inference to
succeed, the compiler must be able to determine the data type to express. That the Necromancer
struct's functions are generic doesn't help-- the data type of the instance sent in will always
differ from the requested type. As type erasure also removes the possibility of exhuming the return
type from the passed trait object, the caller must specify the type to expect or the compiler will
have no idea what to send back. Hence the caller must specify the underlying concrete type
explicitly to overcome type erasure when casting an interface instance to a concrete type instance.
How type erasure is overcome within other casting scenarios is discussed in the following sections.


##### Transmutation: Overview

Because interface-to-interface casting must be supported even when the underlying concrete type is
unknown at compile, type erasure is responsible for most of the complexity within the Transmutation
component. Overcoming this obstacle within these casts requires the ability to discover the
concrete type at runtime, to gracefully handle situations where a cast to an invalid interface was
requested, and to support new types added in libraries yet to come from within the RDH core crate's
implementation. In addition, despite similar syntax, the methodology through which type erasure can
be overcome is both more complicated and more costly for downcasts than for upcasts.

##### Transmutation: Upcasts vs downcasts

All interface-to-interface casts fall into one of two categories, depending on whether the
resulting type is more abstract or more concrete than the initial type. When the resulting type is
more abstract, the operation is referred to as an upcast. When the resulting type is more concrete,
it's referred to as a downcast. Depending upon how one visualizes the tree formed from a type
hierarchy, these terms may feel reversed. After all, the leaves on actual trees sprout from the
endpoints on branches, which in turn generally grow further upward. As the word "generally" in the
previous sentence nicely demonstrates, however, concepts in computer science are more heavily
regimented than their metaphorical inspirations may be. Whereas a real-world tree's branches may
bend sideways or downward, the relationships in a type hierarchy are always either more or less
abstract, either upward or downward from a given place within the hierarchy. Perhaps a better
visualization, then, would be an organizational chart. Within any organization, the further down
one is placed within the org chart, the more specialized the work undertaken becomes. When looked
at from this vantage point, it's likely easier to see why one refers to proceeding toward more
abstract types as an upcast rather than a downcast.

This analogy admittedly starts breaking down, however, when one considers how much simpler it is to
move *up* this tree than down. Because each type implements the interfaces for all types that
precede it, and all interfaces are supertraits for all interfaces that precede them, the outcome of
an upcast request is never in doubt. So long as the compiler will allow the call, the upcast will
succeed. The reverse isn't true, however. Whether a downcast for a given interface instance will
succeed depends on whether the concrete type underlying it implements the requested target
interface. Since the concrete type is unknown at compile time, the requested downcast may well
fail.

For example, consider the upcast request `let icar = isedan.as_icar()`. From this statement, we can
assert that the underlying concrete type is at least a Sedan. While it could be a type that
encompasses Sedan, it must be at least a Sedan, or it couldn't be represented by an ISedan
instance. Further, because ISedan is a supertrait of ICar, it necessarily follows that an ICar
instance may be obtained from an ISedan instance. Because all Sedans are Cars, this request will
always succeed. In contrast, consider the downcast request `let isedan = icar.as_isedan()`. Here
the outcome isn't so clear. While we can assert that the concrete type underlying the ICar instance
is at least a Car, we know nothing further about it. Does the ICar reference contain a Sedan or an
encompassing type such as a MidSizedSedan? If so, the cast will succeed. What if the concrete type
were, instead, a Coupe? A Crossover? A Convertible? As these are not Sedans, the cast would fail.
That not all Cars are Sedans demonstrates that any given downcast may fail. The outcome can only be
determined at runtime.

##### Transmutation: Type UNerasure

Overcoming type erasure within upcasts is even easier to achieve than within interface-to-concrete
casts. In fact, type erasure is essentially a non-factor in upcasts! When a method such as
as_icar() is invoked on an instance of the ICar interface or one of its supertraits, Rust simply
invokes the implementation of that function that belongs to the underlying concrete type. Because
this method can be implemented directly for the underlying concrete type and aggregated into the
public interface for all encompassing types, its implementation has direct access to the underlying
concrete instance. Further, since this method returns a type defined within the same library as the
type for which it's implemented (or an *earlier* library), no heroics are necessary for it to
express its result. It simply expresses the instance, already cast as the requested type via native
Rust functionality, using the single word "self."

In contrast, overcoming type erasure within downcasts is far harder to achieve than within
interface-to-concrete casts, mostly due to need to support the extension of type hierarchies across
library boundaries. Within the request *let isedan = icar.as_isedan()*, for example, the concrete
type underlying the ICar instance may or may not be defined within the same library as ISedan and
ICar. (In fact, ISedan and ICar may well be defined in separate libraries from one another!) When
an encompassing type is defined within a library that must be built after the library within which
the encompassed type is defined, a compile time reference to the encompassing type can't exist
anywhere within the encompassed type's library. This precludes the possibility that as_isedan()
could be aggregated into the trait that acts as the public interface for its encompassed types.
Instead, this function can only be implemented for the *trait objects* created from those traits.
The distinction between traits and trait objects here is critical. Whereas anything aggregated into
the trait itself has access to the underlying concrete type instance, anything that can be
implemented only for the trait *object* does not. 

Expanding the earlier example, let's say that Sedan is encompassed by CompactSedan, MidSizedSedan,
and FullSizedSedan. Within these encompassing types, as_isedan() is an upcast. Further, because
ISedan must be defined either within the same library as (or *earlier* than) Sedan and its
encompassing types, as_isedan() can be aggregated into the traits that act as their public
interfaces. This grants the as_isedan() implementation access to the underlying concrete type
instance, leading to an easy defeat of type erasure. In contrast, let's say that Sedan encompasses
Car, Vehicle, and Construct. Within these encompassed types, as_isedan() is a downcast. Further,
because ISedan is implemented within a *later* library than its encompassed types, as_isedan() can
only be implemented for their trait objects, barring access to the underlying concrete type
instance and complicating their path to defeating type erasure. Whereas upcasts are able to access
the underlying concrete instance easily through native Rust functionality, downcasts must rely on
Necromancy instead.

Astute readers may have already seen a problem with relying on Necromancy to gain access to an
interface instance's underlying concrete type: Necromancy functions require callers to specify the
concrete type to unearth, but interface-to-interface casts must be supported even when the concrete
type is *not* known. The Transmutation component must therefore *discover* the underlying concrete
type at runtime. While knowing the type only at runtime precludes the possibility of returning the
resulting concrete instance to the caller, doing so isn't part of interface-to-interface casts
anyway. Rather, once the concrete instance is unearthed, Transmutation simply casts that instance
to the requested type and then expresses the result, successful or not.

In an ideal world, this runtime discovery process would be an O(1) operation. Our world, however,
remains stubbornly nonideal in many ways, including this one. While the TypeIdentifier for the
underlying concrete instance can be obtained through a call to the ConcreteDivinator trait's
concrete_type_identifier() method, there's no direct way to associate a TypeIdentifier to an actual
Rust type. This requires the Transmutation component to test this TypeIdentifier instance against
the TypeIdentifiers for each candidate type. Unfortunately, obtaining the TypeIdentifiers to test
against currently requires a function call. This precludes the use of a match expression and its
commensurately improved runtime performance relative to separate conditionals. (Note that Rust's
built-in TypeId type and Any trait share all these limitations. See Issue #7 in "Knowledge Base.md"
for further details.)

The runtime discovery process is therefore, at the moment, as simple as it gets: the Transmutation
component simply *guesses* the underlying concrete type. If this guess is correct, the concrete
instance returned from the Necromancy call is cast to the requested interface and the downcasting
process is complete. Otherwise, Transmutation simply makes another guess and the process continues
until either the correct type is guessed or all types have been rejected. While this O(n) approach
is clearly not ideal, it's sufficient for a proof-of-concept, and many possible improvements are
already being considered. Additionally, two factors *may* help to mitigate its impact in the
meantime. First, because Transmutation also determines the library in which the underlying concrete
type is defined, all potential candidates defined within any other library are eliminated outright.
Second, a general rule of thumb discussed in many places around the Internet is that type
hierarchies should be constrained to no more than two or three levels deep at most. (See this
[thread] and many others.) When this rule of thumb is followed, the three comparisons required in
the worst case are utterly trivial for modern hardware to undertake. These mitigations are
admittedly flawed, however, in much the same way cooperative multitasking was a flawed concept: the
lack of central control. A rule of thumb is no more than a suggestion. RDH project developers are
well aware that relying on individual developers to follow such suggestions is, at best, a weak
mitigation to a performance concern. The intention is to rely solely on this for as short a time as
possible.

[thread]: https://www.c-sharpcorner.com/UploadFile/dacca2/measure-depth-of-inheritance-and-class-coupling-in-visual-st/

One may well wonder how Transmutation is able to test the underlying concrete instance's type
against candidate TypeIdentifiers when any or all of these types will be defined within libraries
that haven't even been thought of yet. That's a good question! Because Transmutation is part of the
rdh crate-- the very first hierarchy crate built within any RDH-based project-- its code can only
directly reference types defined within this same crate, namely Construct and IConstruct.
Transmutation is able to handle additional types with the help of the Divination component and
code yielded from invocations of its own component macros. As mentioned above, each concrete type
and public interface is assigned a TypeIdentifier that contains a unique u128 value. These values
are defined within each type's module and registered with the Divination component at application
startup. Registration is invoked from the init() function within the main or lib module for each
crate. The main or lib module also includes the definition of one additional u128 value to serve as
the crate's LibraryIdentifier. This is registered in tandem with individual type identifiers.
Further, functions yielded by the provision_transmutation and extend_downcasting macros handle
these comparisons on Transmutation's behalf. When a request to downcast to an unknown type is
received, Transmutation consults Divination to determine the library in which the underlying
concrete type is defined. Then, consulting its own lookup table, Transmutation invokes the function
within the concrete type's defining library that was registered to handle downcasts to the
requested interface. The init() function and these Transmutation macros are discussed in further
detail in the sections below.

##### Transmutation: Macros

Macros defined within the Transmutation component are invoked within each hierarchy type module to
facilitate the implementation of interface-to-interface casting functionality for their public
interfaces. While it may not seem ideal to use macros for this purpose, doing so achieves two
important things. First, because casting support is implemented by the compiler within inheritance-
based languages, for RDH project goals to be met, the amount of manual development effort required
to replicate this functionality must be as close to *none* as possible, something macros can help
achieve. Second, their use makes providing downcasting support *possible at all.* The world of type
hierarchies is naturally highly abstract, yet interface-to-interface downcasts still somehow manage
to stand out for their highly abstract nature within this already highly abstract world! Code
performing such casting operations would typically therefore be naturally highly generic. Rust's
support for generics is somewhat lacking, however. Generic methods can't be included within traits
that must serve as trait objects. Function pointers and closures don't support generic parameters
or return types. Generic functions can't be constrained to accept only trait objects. The list goes
on! After running into dead ends time and time again, it became evident that no clear way exists
within the limited extent of Rust's support for generics to implement downcasts using them, and
macros were selected instead. While these macros solve difficult problems, it's important to keep
in mind that their use is not *free.* They do help reduce developer burden, but the functionality
they add still increases binary sizes, and downcasts are still expensive relative to upcasts,
exactly as though these features been hand coded as each new type was added.

Manifest:

  * Intended for Protected use:
    * provision_transmutation (exported macro)
      * Declares and/or implements traits and structs required to support casting operations, as
        detailed below
      * Contains two branches, one for Construct and the other for all types that encompass
        Construct
        * The Construct-only branch skips downcasting support as no casting operation *to*
          IConstruct constitutes a downcast
    * extend_downcasting (exported macro)
      * Declares the struct and implements the interface required to perform downcasts for each
        interface type within each RDH library, as detailed below
      * Invoked by provision_transmutation to establish downcast support within the originating
        library
      * Invoked by bespoke RDH libraries when adding new encompassing types that also implement
        that interface

  * Intended for Private use:
    * impl_concrete_divinator (exported macro)
      * Implements the ConcreteDivinator trait, defined within the Divination component as shown
        above
      * Allows type and library information to be obtained for the concrete type that underlies an
        interface instance
      * Separated as it's invoked from both provision_transmutation branches
    * provision_upcasting (exported macro)
      * Declares and implements the trait required to support interface-to-interface upcasts
      * Separated as it's invoked from both provision_transmutation branches
      
Although two macros are labeled as "intended for protected use" and two are labeled as "intended
for private use," all of them are exported. As discussed within the construct module section,
above, currently no way is known to model protected access through Rust's visibility scopes.
Further, Rust's support for macros across library boundaries is rudimentary. A macro is either
exported, in which case it can be leveraged everywhere, or it isn't exported, in which case it can
be leveraged only by private elements within its defining module. Any macro invoked by an exported
macro must itself be exported, or the compiler won't be able to find them. While the consequences
of exposing these macros publicly aren't dire, doing so is clearly not ideal. Ideas for reducing
this exposure are currently being considered. See Issue #11 in "Knowledge Base.md" for further
details.

##### Transmutation: provision_transmutation macro

An invocation of the provision_transmutation macro must be included within the definition of each
new RDH framework type. Ignoring line numbers for the time being, an example invocation could be:

```rust
 1  provision_transmutation!
 2  {
 3      for Car: ICar + [IVehicle IConstruct]
 4      {
 5          add Fn[as_icar, as_icar_mut] to
 6          (
 7              UpcastsToICar,
 8              DowncastsToICar,
 9              ICarDowncaster,
10              DowncastsGeneralCarsToICar: DowncastsLibraryTypesToICar include [Sedan]
11          )
12      }
13  }
```

The provision_transumtation macro implements everything needed to support all casting operations
for each new type, but it needs quite a bit of information from the developer to do so. Pertinent
lines are translated below.

Line  3: "For the Car type, which introduces the ICar interface and implements both IVehicle and
          IConstruct:"
Line  5: " for casting support, while creating each item below, add functions called as_icar and
           as_icar_mut to..."
Line  7: " ... the UpcastsToICar trait, to enable upcasts to the ICar interface..."
Line  8: " ... the DowncastsToICar trait, to enable downcasts to the ICar interface..."
Line  9: " ... the ICarDowncaster singleton struct, which determines the library to which these
               downcast requests must be routed based on the underlying concrete type..."
Line 10: " ... the DowncastsGeneralCarsToICar struct, which handles these downcast requests for
               *this* library..."
         " ... and the DowncastsLibraryTypesToICar trait, which must be implemented by the struct
               that handles these downcasts within each library."
         " and, finally, invoke *this* library's downcast handler when the underlying concrete type
           is Sedan."

Note that the lists in Lines 3 and 10 are both *space*-delimited.

For those used to the compiler automatically adding support for necessary casting scenarios without
any developer involvement whatsoever, this probably seems like quite a bit of work! When compared
to the actual implementation of the provision_transmutation macro, however, this can be seen as
amounting to little more than markup notation. That being said, as explained within the
rdh_extension_example crate sections below, ideas for further simplification are currently gaining
traction. Hopefully even this will be abstracted out of developer hands soon enough.

The example provision_transmutation call above likely makes it clear that this macro will add all
code needed to define and implement the traits and structs required to process casting requests for
the Car type and ICar public interface, using the specified names. Perhaps less obvious but at
least as important to understand is how this invocation will wire Car and ICar to their ancestor
and descendant types to facilitate such casts. Within this example, Line 3 above lists all
interfaces implemented by the Car type that existed prior to its creation. This list is used to
populate the response to ConcreteDivinator's implements() method. It also serves as the list of
trait objects for which the DowncastsToICar trait will be implemented. The end of Line 10 also
includes a list, this time of the types that in turn encompass the Car type. When the Transmutation
component receives a downcast request to the ICar type, once it determines that the underlying
concrete type was defined within this library, this list contains the possible guesses it should
make as to the concrete instance's exact type-- in this case, only Sedan and (implied) Car would be
guessed. Note the list in Line 10 only contains types also defined within this same library. In
order for Transmutation to guess types defined in *later* libraries when the underlying concrete
type was defined there, the extend_downcasting macro must be called from within that library.

While only the provision_transmutation macro is directly invoked, each of the other macros is
invoked behind the scenes. Work described within Line 7 is performed by the provision_upcasting
macro, while the work in line 10 is delegated to extend_downcasting. Finally, though not
specifically called out within a line, the impl_concrete_divinator macro is invoked just before
provision_upcasting, ensuring Divination is ready to handle queries about this new type.

##### Transmutation: extend_downcasting macro

When a new type is added to the hierarchy that encompasses a type defined within an earlier
`library, it becomes possible for a downcast to be requested for an interface instance whose
underlying concrete instance is this new type. It's also possible for both the initial interface
and the requested interface to have been defined in earlier libraries as well. Building on the
earlier example, the MidSizedSedan type and its public interface, IMidSizedSedan, may encompass the
Car type and ICar public interface, both of which are defined within an earlier library. Further,
an instance of the MidSizedSedan type could be abstractly represented by an IConstruct instance. A
downcast to ICar could then be requested. As IConstruct is defined within the core RDH library,
ICar is defined within a different library, and MidSizedSedan is defined within yet a third
library, this downcast operation crosses crate boundaries multiple times.

The invocation to provision_transmutation within the MidSizedSedan module ensures that casts to and
from IMidSizedSedan will function properly. For casts to and from other interfaces implemented by
MidSizedSedan to function properly, however, more work remains. If MidSizedSedan encompasses other
types in the same library within which it's defined, e.g. Sedan, the provision_transmutation
invocations for these types must be updated to include the MidSizedSedan type, as discussed within
the previous section. For each interface implemented by MidSizedSedan that was defined within
previous libraries, however, the extend_downcasting macro must be invoked to establish
MidSizedSedan as a possible concrete type that could be involved in downcasts to those interfaces.
Note, however, that only at most one such invocation per-interface is required within any one
library. Within this example, because Sedan is defined within the same library as MidSizedSedan, an
extend_downcasting invocation would already have been added for the ICar interface to account for
Sedan. Rather than adding a new invocation, the MidSizedSedan type would simply be added to this
existing invocation, in much the same way as it would be added to Sedan's provision_transmutation
call, as previously described.

Again ignoring line numbers, an example invocation could therefore be:

```rust
 1  extend_downcasting!
 2  {
 3      for ICar
 4      {
 5          add Fn[as_icar, as_icar_mut] to DowncastsSedansToICar: DowncastsLibraryTypesToICar
              include [Sedan MidSizeSedan]
 6      }
 7  }
```
Note that the list in Line 5 is *space*-delimited.

The above may be translated as: "To extend downcasting support for ICar, add functions called
as_icar and as_icar_mut to the new DowncastsSedansToICar struct. This struct will implement the
DowncastsLibraryTypesToICar trait to handle all such downcasts for concrete types defined within
this library. When Transmutation receives a request to downcast an interface instance to ICar when
its underlying concrete type was defined within this library, it should call this struct's methods
to perform the cast on its behalf. The called method will first guess that the underlying concrete
type is Sedan. If necessary, MidSizedSedan will be tested as well."

As additional new types are added within this same library, e.g. CompactSedan and FullSizedSedan,
their types would also have to be added to the above invocation. Within this example, of course, an
additional extend_downcasting invocation would be required for the IVehicle interface as well. Its
formulation is left as an exercise for the reader.

##### Transmutation: Other macros

Although the remaining two macros are intended only for private consumption, as they're currently
exported regardless, it's likely a good idea to discuss them briefly.

* The impl_concrete_divinator macro implements the Divination component's ConcreteDivinator trait
for the new type. This implementation allows developers to determine whether the type of the
concrete instance underlying an interface instance *is* a specific concrete type, as well as
whether it *implements* a specific other interface. Calls to these methods are instrumental within
casting operations.
  * For example, following `let car = Car::new(); let ivehicle = car.as_ivehicle();`, a call to
    is() for ivehicle would return true IFF the queried type were Car, but implements() would
    return true if the queried type were ICar, IVehicle, or IConstruct.
* The provision_upcasting macro declares the new type's upcasting trait and implements it for that
  type and all types that encompass it.
  * This allows calls such as `let icar = imid_sized_sedan.as_icar()` to succeed in casting an
    IMidSizedSedan instance to an ICar instance.


### rdh_extension_example crate

The rdh_extension_example crate contains example RDH framework types, templates through which new
RDH framework types encompassing Construct may be added, and type_dispatch_examples, a module that
demonstrates a method for dispatching hierarchy instances to handlers depending on their underlying
concrete type even when that type is unknown at compile time.

#### Example RDH framework types

The two example RDH framework types, UIElement and Checkbox, are intended to represent a fictional
graphical user interface, where UIElement directly encompasses Construct and Checkbox encompasses
UIElement. It's important to note that both of these example types are *placeholder only*. They
will eventually be removed altogether and replaced with a more fully featured hierarchy intended to
showcase a situation where a type hierarchy is a good fit, regardless of language. See Work Item #3
in "Road Map.md."

#### Type templates

Two templates are included within the example extension crate. NEW_TYPE_TEMPLATE.rs is a barebones
RDH framework type module that contains in-place instructions on what to add or change, where, and
under which circumstances. BARE_NEW_TYPE_TEMPLATE.rs is identical to the first template, but it
lacks these instructions. Developers creating their first few new RDH framework types would be well
advised to follow the first template, but anyone with even a small amount of experience doing so
will likely be able to fill out the second template easily enough. While these templates do help,
RDH developers are well aware that it still takes quite a bit more work to create a new RDH
framework type than it does to derive a type in languages such as C++ or C#. A means of automating
these templates is currently being considered. See Work Item #4 in "Road Map.md" for further
details.

#### type_dispatch_examples module

While an RDH framework type could be represented abstractly at any given time, the most common
situation where this will occur is within collections of instances of a specific interface. For
example, a function may process a list of all instances whose underlying concrete type encompasses
Car. This could be represented as Vec<Box<dyn ICar>>. It's important to note that not all elements
within this vector would (necessarily) be Car instances. Sedans, Coupes, Crossovers, Convertibles,
etc., could all be present within this collection. If the function that processes this collection
only needs to invoke methods that were defined within ICar (or earlier), this distinction is
unimportant. Additional processing may be required for some types of cars, however. For example, if
the function is designed to test moving parts, Convertibles may require special processing to
account for their roofs, which are stationary on all of these other types of Cars. The
type_dispatch_example contains example functions for such situations. These functions accept an RDH
framework type instance and then dispatch it to an appropriate handler function, depending on
either its concrete instance type or the other interfaces that type implements. Adapting these
example functions to work with whole collections should be trivial.


### rust_development_hierarchy crate

The rust_development_hierarchy crate serves the role of an example application leveraging an RDH
type hierarchy. At this time, other than building the other crates, this crate only provides a main
module that performs some highly rudimentary framework tests. Formal automated testing is currently
planned. See Work Item #1 in "Road Map.md" for further information.


### Crate manifest summary

As shown within the sections above, the three crates within this project together comprise a proof-
of-concept for the creation of type hierarchies within Rust. The rdh crate contains the basis of
any type hierarchy, using the near-complete base type, Construct. The rdh_extension_example crate
contains placeholder framework types that encompass this base Construct type, templates through
which new framework types can be created, and a module that contains examples of how to work with
heterogeneous collections of framework instances when special processing is required for a subset
of their concrete types. Finally, the rust_development_hierarchy crate serves as a placeholder for
an application that consumes a type hierarchy.

While reading this documentation, those familiar with developing programs in either C++ or C# may
have been wondering about the absence of the word *DLL* (or *SO*). After all, when built out to
real world application scale, the most likely binary form for these crates in both languages would
be two DLLs and an EXE (or the Linux equivalent thereof-- stipulated going forward). Rust's support
for DLLs, however, is not as fully featured as one may hope. While C-style DLLs are well supported,
their use is intended for interoperability with existing C code, not within pure Rust applications.
While one *could*, presumably, implement C-style DLL shims to glue crates together, this would be
as unfathomable as creating COM components using ATL in order to glue C# assemblies together. This
would definitely work, but no reasonable person would do it! While Rust is also capable of building
*native* DLLs, the Rust compiler is not capable of linking to pre-built binaries. This has two
important ramifications. First, any project that dynamically links to a DLL will fail to build if
the DLL's source code is unavailable. Second, it isn't possible to overwrite the DLL with an
updated version to support patching scenarios. When one attempts to do so, the application no
longer runs as its executable is tied to a particular DLL build. Finally, it seems as though the
Rust compiler is unable to statically link to some libraries while dynamically linking to others.
As all of this together essentially negates any potential gains from building crates as DLLs, RDH
developers elected to simply build plain old Rust libraries instead.


## Getting Started

By the time all provided documentation has been read, developers are likely eager to start building
out type hierarchies! The steps below can be used as a guide for getting started. (Note that the
below assumes familiarity with both Rust and Cargo.)

1. Use Cargo to create a new Rust project.
   a. While it would be advisable to create a binary executable to consume the type hierarchy and a
      separate library crate to hold the RDH framework types themselves, separating these isn't
      necessary while learning or for very simple applications undertaken by a single developer.
   b. If creating both an executable and a library, import the library into the executable.
   c. Import the rdh crate into the executable and the library (if one was created).
2. Create a new module to hold a new RDH framework type and copy the contents of
   NEW_TYPE_TEMPLATE.rs into it.
   a. Be sure to wire this new module up properly to be accessible from the binary executable and
      separate library (if one was created).
3. Follow all directions within the template.
   a. Each direction is prepended with TODO:
4. Make sure to register the new type within its binary's main module (or library's lib module).
   a. You can use the lib module within the rdh_extension_example crate as an example.
5. Repeat steps 2-4 to build out the hierarchy.
   a. Be sure to follow instructions within the template carefully as additional work is required
      when encompassing a type that encompasses a previous type, especially across library
      boundaries.
6. In the short term, the main module may be used to drive the type hierarchy to ensure everything
   is wired properly.
7. Finally, create a copy of the "BASE API Reference for RDH Consumers.md" and do the following:
   a. Rename it to reflect the type hierarchy and its purpose.
   b. Replace downcasting placeholder information with the actual traits, structs, methods, and
      functions within the hierarchy.
      i. Sections to replace are marked as **** ABSTRACT PLACEHOLDER START **** and **** ABSTRACT
         PLACEHOLDER END ****
   c. Add documentation as needed to explain each new type in the hierarchy.

