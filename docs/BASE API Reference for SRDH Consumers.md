# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

BASE API Reference for RDH Consumers.md
Updated February 7, 2023

IMPORTANT NOTES:
* THIS IS A WORK IN PROGRESS. Please read the Status section for each entry carefully to determine
  what, if anything, is considered permanent.
* THIS IS A *BASE* REFERENCE. Those who implement RDH extension libraries should incorporate their
  own types, etc., into a fork of this document, including replacing abstract casting placeholders
  with their actual implementation details. Sections to replace are marked as **** ABSTRACT
  PLACEHOLDER START **** and **** ABSTRACT PLACEHOLDER END ****

NOTES:
* This document covers only the base Construct type, in its current temporary form.
  * UIElement and Checkbox are not documented as they are wholly placeholder and will be removed
    entirely going forward.
  * Construct documentation will be updated as it's brought into its permanent form.
    * See Work Item #2 in "Road Map.md" for details.
* Code within documentation below is presented in terms of a fictional Vehicle type hierarchy.
  Snippets are not meant to be taken as literal buildable code.
* Within the below, when types appear in parentheses, it means any type that encompasses that type,
  including itself.
  * For example, (IConstruct) means any trait object the encompasses IConstruct, whereas IConstruct
    (no parentheses) means the literal IConstruct trait or its trait objects.
  * Also, (Construct) means any concrete type that implements the IConstruct trait, whereas
    Construct (no parentheses) means the literal Construct struct or its instances.
* Casting between concrete types is not supported within RDH and never will be.
  * In inheritance-based languages such as C# and C++, a reference to an object and the result of
    casting that reference to an ancestor type both refer to the same object at the same memory
    location.
  * As Rust has no native concept of type inheritance, the equivalent operation would yield a
    reference to a *different* object at a *different* memory location.
  * Allowing access to these encompassed instances would cause three known problems. In increasing
    order of severity:
    * Developers familiar with inheritance-based languages but new to Rust may struggle to keep in
      mind that the resulting instance would *not* be the same object, but instead a completely
      different object. This gap in understanding could lead to all kinds of chaos.
    * Nothing would tie the instance yielded from the cast back to its encompassing instance. There
      would be no (good) way to determine later which Sedan instance yielded this Car instance, or
      whether it was yielded from a Sedan instance at all, or even whether the instance was
      initially declared as a Car type, a Sedan type, or some other less abstract type. Because of
      this, it would be impossible to cast the instance back to its initial type.
    * This would break polymorphism! As nothing would tie the Car instance back to its encompassing
      Sedan instance, calling a virtual function on the Car instance would invoke the
      implementation from the *Car* type, *not* the Sedan type. So, for example, while a call such
      as *car.show_type()* would be expected to yield "Sedan," it would instead yield "Car." This
      would clearly be buggy behavior.
  * While casting between concrete types can't be supported, equivalent access can be acquired by
    casting to the corresponding public interface instead.


## Public Interfaces and Types

Notes:
* Public interfaces are trait objects that can be used to abstractly represent instances of their
  corresponding type or of any type that encompasses that type.
* While types are typically structs, they can be any type that can suitably implement the public
  interface and all required traits.
  * As traits other than the public interface implemented by these types are not intended for
   (direct) public consumption, they are not discussed within this document.
* Items below are presented in <public interface name> (<corresponding type name>) format.
  * In some languages, such as C#, both types (i.e. classes) and interfaces can manifest
    inheritance.
  * In others, such as C++, only classes can manifest inheritance, as interfaces don't exist as
    distinct entities.
  * In Rust, on the other hand, all natively supported aspects of inheritance are manifested within
    "supertraits"-- there is no concept of inheritance for types whatsoever.
    * That RDH types appear to support inheritance is, in effect, an illusion brought about by the
      core RDH infrastructure layer component (Thaumaturgy).
  * Because "supertraits" are the basis of inheritance in Rust, unlike in C# where interfaces never
    *have* to inherit from one another, in Rust they *must* do so.
  * Presenting an inheritance structure in Rust therefore is far more naturally done through traits
    than through types, and so the RDH type hierarchy is presented public interface first.


### IConstruct (Construct)

Summary: The public interface and struct that serve as the root of the RDH type hierarchy.
Library: rdh (the core RDH library)
Module: construct
Status: Properties and virtual methods are placeholder only and will be replaced by permanent
        functionality going forward.

#### Instantiation

```rust
let construct = Construct::new();
```
-OR-
```rust
let mut construct_mut = Construct::new();
```

#### Properties

* [PLACEHOLDER ONLY] name: Gets and sets this (Construct's) name.
  * Accessor: `fn get_name(&self) -> &str;`
    * Example: `println!("{}", construct.get_name());`
  * Mutator: `fn set_name(&mut self, value: &str);`
    * Example: `construct.set_name("Nifty new name!");`

#### Virtual Methods

* [PLACEHOLDER ONLY] on_click: Handles click events for this (Construct), according to its concrete
  type.
  * `fn on_click(&mut self);`

#### Non-Virtual Methods

* NONE

#### Casting Support

##### Upcasting

###### Summary
* Supports upcasting instances of encompassing types to IConstruct trait objects.
* Upcasting always succeeds.
* When invoked by trait objects, creates an &dyn IConstruct with the same underlying concrete
  instance.
  * Note that this is equivalent to *Nightly* Channel Rust casting syntax: `as &dyn IConstruct`.
    * See https://github.com/rust-lang/rust/issues/65991 for feature status.
* When invoked by concrete instances, creates an &dyn IConstruct with itself as the underlying
  concrete instance.
  * Note that this is equivalent to *Stable* Channel Rust casting syntax: `as &dyn IConstruct`.
* Elsewhere in documentation, this functionality is referred to as Transmutation, a subcomponent of
  the core RDH infrastructure layer component, Thaumaturgy.

###### Methods
  * `fn as_iconstruct(&self) -> &dyn IConstruct;`
    * Examples: `let iconstruct = isedan.as_iconstruct();`
                `let iconstruct = sedan.as_construct();`
  * `fn as_iconstruct_mut(&mut self) -> &mut dyn IConstruct;`
    * Examples: `let iconstruct_mut = isedan_mut.as_iconstruct();`
                `let iconstruct_mut = sedan_mut.as_construct();`

##### Downcasting

###### Summary
* Supports downcasting IConstruct and Construct instances to encompassing types.
* When successful, methods below express a reference to the underlying concrete instance cast to
  the requested interface, embedded within an `Option<&T>` or `Option<&mut T>`. Otherwise they
  express None.
* Elsewhere in documentation, this functionality is referred to as Transmutation, a subcomponent of
  the core RDH infrastructure layer component, Thaumaturgy.

************************************
**** ABSTRACT PLACEHOLDER START ****

NOTES FOR EXTENDING DEVELOPERS (To be removed):
* Documentation for downcasting is abstract because methods are *added* to dyn IConstruct as
  encompassing types are implemented.
  * As it isn't possible to later modify the definition of any type in C# or C++, it may take some
    time for those new to Rust to feel comfortable with this concept!
* Both casting methods for each additional encompassing type must be added below.
* The following placeholders are used within this template documentation:
  * $target_type: The type that corresponds to the public interface, e.g. Vehicle.
  * $target_interface: The public interface to which downcasts will be made, in snake case.
    * Suggested naming convention: I<TypeName>, e.g. IVehicle => ivehicle
* Nothing enforces the use of the suggested naming conventions above.

************************************

###### Methods

  * To $target_interface:
    * Downcasting will only succeed if the underlying concrete type is $target_type or a type that
      encompasses $target_type.
    * `fn as_$target_interface -> Option<&dyn $target_interface>;`
      * Examples: `let $target_interface = iconstruct.as_$target_interface().expect(`
                    `"Underlying concrete instance is not of an expected type!");`
                  `let $target_interface = construct.as_$target_interface().expect(`
                    `"Underlying concrete instance is not of an expected type!");`
    * `fn as_$target_interface_mut -> Option<&mut dyn $target_interface>;`
      * Examples: `let $target_interface_mut = iconstruct_mut.as_$target_interface_mut().expect(`
                    `"Underlying concrete instance is not of an expected type!")`
                  `let $target_interface_mut = construct_mut.as_$target_interface_mut().expect(`
                    `"Underlying concrete instance is not of an expected type!");`

  * To NEXT TARGET INTERFACE:
    * ETC.

**********************************
**** ABSTRACT PLACEHOLDER END ****
**********************************

##### Concrete Type Recovery

###### Summary
* Supports casting an IConstruct trait object to its underlying concrete type IFF the concrete type
  is known at compile time.
* Recovery will only succeed if the underlying concrete instance is *literally* the specified
  concrete type.
* When successful, the methods below express an &<Target Concrete Type>, embedded within an
  `Option<&T>` or `Option<&mut T>`. Otherwise they express None.
* Note that although these methods can be invoked from concrete instances, doing so essentially
  constitutes an expensive no-op.
  * Were a reference needed, the following call would be a far better choice:
    `let construct_ref = &construct;`
* Elsewhere in documentation, this functionality is referred to as Necromancy, a component of the
  core RDH infrastructure layer module, Thaumaturgy.
 
###### Methods
  * `fn as_concrete<T>(&self) -> Option<&T> where T: IConstruct;`
    * Examples: `let sedan = iconstruct.as_concrete::<Sedan>().expect(`
                  `"Underlying concrete instance is not a Sedan!");`
                `let mid_sized_sedan = construct.as_concrete::<MidSizedSdan>().expect(`
                  `"Underlying concrete instance is not a MidSizedSedan!");`
  * `fn as_concrete_mut<T>(&mut self) -> Option<&mut T> where T: IConstruct;`
    * Examples: `let sedan_mut = iconstruct_mut.as_concrete_mut::<Sedan>().expect(`
                  `"Underlying concrete instance is not a Sedan!");`
                `let mid_sized_sedan_mut = construct_mut.as_concrete_mut::<MidSizedSdan>().expect(`
                  `"Underlying concrete instance is not a MidSizedSedan!");`

##### Type Introspection

###### Summary
* Supports obtaining information about an IConstruct or Construct instance.
* Elsewhere in documentation, this functionality is referred to as Divination, a component of the
  core RDH infrastructure layer module, Thaumaturgy.

###### Methods 

  * `fn type_identifier(&self) -> TypeIdentifier;`
    * Retrieves the TypeIdentifier for the instance's type.
    * Examples:
      * Retrieve the TypeIdentifier for the IConstruct public interface:
        `let itype_identifier = iconstruct.type_identifier();.`
      * Retrieve the TypeIdentifier for the Construct struct:
        `let type_identifier = construct.type_identifier();`
  * `fn library_identifier(&self) -> LibraryIdentifier;`
    * Retrieves the LibraryIdentifier for the instance's type (i.e. the identifier of the RDH
      library in which the type was defined).
    * Examples:
      * Retrieve the LibraryIdentifier for the library in which the IConstruct public interface was
        defined:
        `let ilibrary_identifier = iconstruct.library_identifier();`
      * Retrieve the LibraryIdentifier for the library in which the Construct struct was defined:
        `let library_identifier = construct.library_identifier();`

  * `fn concrete_type_identifier(&self) -> TypeIdentifier;`
    * Retrieves the TypeIdentifier for the underlying concrete instance's type.
    * Examples:
      * Retrieve the TypeIdentifier for the IConstruct's underlying concrete instance:
        `let concrete_type_identifier = iconstruct.concrete_type_identifier();`
      * Functionally equivalent to invoking construct.type_identifier():
        `let concrete_type_identifier = construct.concrete_type_identifier();`
  * `fn concrete_library_identifier(&self) -> LibraryIdentifier;`
    * Retrieves the LibraryIdentifier for the underlying concrete instance's type.
    * Examples:
      * Retrieve the LibraryIdentifier for the IConstruct instance's underlying concrete instance:
        `let concrete_library_identifier = iconstruct.concrete_library_identifier();`
      * Functionally equivalent to invoking .library_identifier():
        `let concrete_library_identifier = construct.concrete_library_identifier();`
  * `fn implements(&self, interface_type: TypeIdentifier) -> bool;`
    * Determines whether the underlying concrete instance's type implements a given public
      interface.
    * Examples:
```rust
      if iconstruct.implements(TypeRegistry::type_identifier_of::<ISedan>())
      {
          println!("It's a brand new car!!!!");
      }
```
      While the following is a legal call, as the concrete type of a concrete type instance is
      always its own type, no new information can be learned from invoking it:
```rust
      if construct.implements(TypeRegistry::type_identifier_of::<ISedan>())
      {
          println!("It's a brand new car!!!!");
      }
```
  * `fn is(&self, test_type: TypeIdentifier) -> bool;`
    * Determines whether the underlying concrete instance is *literally* the specified concrete
      type (see "IMPORTANT NOTES" and "Discussion points", below).
    * Examples:
  ```rust
      if iconstruct.is(TypeRegistry::type_identifier_of::<Sedan>())
      {
          println!("You get a car!! And you get a car!! And *you* get a car!!");
      }
  ```
      The following call will always return true if test_type is the TypeIdentifier for Construct
      and will always return false otherwise:
  ```rust
      if construct.is(TypeRegistry::type_identifier_of::<Construct>())
      {
          println!("I would accept that as an axiom.");
      }
  ```
    * IMPORTANT NOTES: 
      * Although in languages such as C#, "is" will also return true if test_type is an ancestor of
        this object's type or one of its implemented interfaces, this function will return *false*.
        * The *only* time "is" will return true is when the specified TypeIdentifier is the *exact*
          type identifier for this object's concrete type.
      * To determine whether a type implements a given interface, call implements() rather than
        is().
      * To determine whether a type is "derived" from another type (or, really, *encompasses* it),
        call implements(), specifying the type's corresponding public interface, e.g.
  ```rust
         if car.implements(TypeRegistry::type_identifier_of::<IVehicle>())
         {
             println!("A Car IS a Vehicle!");
         }
  ```
    * Discussion points:
      * While returning true may be correct for other "is" queries on a conceptual level, Rust's
        use of composition over inheritance makes doing so problematic, especially in casting
        scenarios.
      * In C# "is" returning true for the same Car before and after an upcast to Vehicle is correct
        on both conceptual and literal levels-- they're the same object at the same memory address.
      * In Rust, however, "is" returning true after such an upcast would *only* be correct
        *conceptually*-- Car and Vehicle are different objects at different memory addresses!


## Support Types

Notes:
* While support types are not a part of the RDH type hierarchy, they are essential to the
  hierarchy's operation.
* Some support types are expressed by public interface methods, others are sent into those methods,
  and others still are invoked by those methods as they carry out their tasks.
* Several of the structs below behave in a manner similar to static classes in C#.
  * Just as within C#, functions within these structs are invoked with the type name rather than
    with an instance of the type.
  * Within Rust terminology, functions invoked on an instance of a type are called "methods,"
    whereas functions invoked on the type itself are called "associated functions."
* For each type below, functionality not intended for (direct) public consumption is not discussed
  within this document.

### LibraryIdentifier

Summary: An opaque instance used to identify the library in which an RDH type was defined.
Library: rdh (the core RDH library)
Module: thaumaturgy
Status: Undergoing stabilization (i.e. interface changes may occur but are not expected).

#### Invocation

NONE. LibraryIdentifier instances should be treated as opaque blobs, requested only when needed and
passed on blindly.


### TypeIdentifier

Summary: An opaque instance used to identify an RDH type.
Library: rdh (the core RDH library)
Module: thaumaturgy
Status: Undergoing stabilization (i.e. interface changes may occur but are not expected).

#### Invocation

NONE. TypeIdentifier instances should be treated as opaque blobs, requested only when needed and
passed on blindly.


### TypeRegistry

Summary: The struct from which information about all RDH types can be obtained.
Library: rdh (the core RDH library)
Module: thaumaturgy
Status: Undergoing stabilization (i.e. interface changes may occur but are not expected).

#### Invocation

The functions within TypeRegistry must be invoked through the type name, as shown below.

##### Associated Functions:
  * `pub fn type_identifier_of<T>() -> TypeIdentifier where T: IConstruct + ?Sized`
    * Retrieves the TypeIdentifier for the indicated RDH instance's type.
    * Examples:
```rust
      let car_type = TypeRegistry::type_identifier_of::<Car>();
```
```rust
      if iconstruct.implements(TypeRegistry::type_identifier_of::<ISedan>())
      {
        println!("It's a brand new car!!!!");
      }
```
  * `pub fn library_identifier_of<T>() -> LibraryIdentifier where T: IConstruct + ?Sized`
    * Retrieves the LibraryIdentifier for the indicated RDH instance's type.
    * Example: `let car_library = TypeRegistry::library_identifier_of::<Car>();`


## Necromancer

Summary: The struct through which casts from (IConstruct) objects to (Construct) instances may be
         requested.
Library: rdh (the core RDH library)
Module: thaumaturgy
Status: Undergoing stabilization (i.e. interface changes may occur but are not expected).

### Invocation

The functions within Necromancy must be invoked through the type name, as shown below.

#### Associated Functions:

Notes:
* The functions below can only succeed if the underlying concrete instance is *literally* the
  specified concrete type.
* When successful, these functions express an &<Target Concrete Type>, embedded within an
  `Option<&T>` or `Option<&mut T>`. Otherwise they express None.
* Note that although the methods below can be invoked for concrete instances, doing so essentially
  constitutes an expensive no-op.
  * Were a reference needed, the following call would be a far better choice:
    `let construct_ref = &construct;`

  * `pub fn unearth_concrete<T, U>(iconstruct: &T) ->`
      `Option<&U> where T: IConstruct + ?Sized, U: IConstruct`
    * Attempts to cast the specified immutable (IConstruct) object into an immutable (Construct)
      instance.
    * Example: `let sedan = Necromancer::unearth_concrete::<_, Sedan>(icar).expect(`
                 `"The Car isn't a Sedan!");`
  * `pub fn unearth_concrete_mut<T, U>(iconstruct: &mut T) ->`
      `Option<&mut U> where T: IConstruct + ?Sized, U: IConstruct`
    * Attempts to cast the specified mutable (IConstruct) object into a mutable (Construct)
      instance.
    * Example: `let sedan_mut = Necromancer::unearth_concrete_mut::<_, Sedan>(icar_mut).expect(`
                 `"The Car isn't a Sedan!");`


************************************
**** ABSTRACT PLACEHOLDER START ****

NOTES FOR EXTENDING DEVELOPERS (To be removed):
* Each new RDH type requires its own global downcasting struct, created by the
  provision_transmutation macro.
  * See the "Transmutation: provision_transmutation macro" section within "Extenders' Handbook.md"
    for further details.
* As types are added, documentation for their downcasting struct and the functionality added to
  upstream trait objects must be placed within the "Public Interfaces and Types" and "Support
  Types" sections.
* The following placeholders are used within this template documentation:
  * $target_type: The type that corresponds to the public interface, e.g. Vehicle.
  * $target_interface: The public interface to which downcasts will be made, in snake case.
    * Suggested naming convention: I<TypeName>, e.g. IVehicle => ivehicle.
  * $global_downcasting_struct: The global downcasting struct for a given type.
    * Suggested naming convention: $target_interfaceDowncaster, e.g. IVehicleDowncaster.
  * $cast: The immutable version of the downcasting function.
    * Suggested naming convention: as_<target_interface>, e.g. as_ivehicle.
  * $cast_mut: The mutable version of the downcasting function.
    * Suggested naming convention: as_<target_interface>_mut, e.g. as_ivehicle_mut.
* Nothing enforces the use of the suggested naming conventions above.

************************************

## $global_downcasting_struct

Summary: The struct through which (IConstructs) whose underlying concrete type is $target_type or a
         type that encompasses $target_type may be downcast to $target_interface.
Library: FILL THIS IN
Module: FILL THIS IN
Status: FILL THIS IN

### Invocation:

Notes:
* The functions within $global_downcasting_struct must be invoked through the type name, as shown
  below.
* Downcasting will only succeed if the underlying concrete type is $target_type or a type that
  encompasses $target_type.
* When successful, methods below express a reference to the underlying concrete instance cast to
  the requested interface, embedded within an `Option<&T>` or `Option<&mut T>`. Otherwise they
  express None.
* Elsewhere in documentation, this functionality is referred to as Transmutation, a subcomponent of
  the core RDH infrastructure layer component, Thaumaturgy.

#### Associated Functions:

  * `pub fn $cast<T>(iconstruct: &T) ->`
      `Option<&dyn $target_interface> where T: IConstruct + ?Sized`
    * Attempts to downcast the specified more abstract immutable (IConstruct) object to an
      immutable $target_interface object.
    * Examples:
```rust
      let $target_interface = $global_downcasting_struct::as_$target_interface(iconstruct).expect(
        "Underlying concrete instance is not of an expected type!");
```
```rust
      let $target_interface = $global_downcasting_struct::as_$target_interface(construct).expect(
        "Underlying concrete instance is not of an expected type!");
```
  * `pub fn $cast_mut<T>(iconstruct: &mut T) ->`
      `Option<&mut dyn $target_interface> where T: IConstruct + ?Sized`
    * Attempts to downcast the specified more abstract mutable (IConstruct) object to a mutable
      $target_interface object.
    * Examples:
```rust
      let $target_interface_mut = $global_downcasting_struct::
        as_$target_interface_mut(iconstruct_mut).expect(
          "Underlying concrete instance is not of an expected type!");
```
```rust
      let $target_interface_mut = $global_downcasting_struct::
        as_$target_interface_mut(construct_mut).expect(
          "Underlying concrete instance is not of an expected type!");
```

**********************************
**** ABSTRACT PLACEHOLDER END ****
**********************************
