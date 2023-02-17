# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

Knowledge Base.md
Updated February 7, 2023

Includes:
* Open Issues: Bugs, design limitations, and other issues in one of three possible states:
               * Not fully understood and undergoing active triage
               * Not fully understood and currently under investigation
               * Fully understood and currently being fixed
* Known Issues: Bugs, design limitations, and other issues that are fully understood but no fix can
                be made (at least not by us / not yet).

IMPORTANT NOTE: THIS IS A WORK IN PROGRESS. Please read documentation carefully to determine what,
                if anything, is considered permanent.

NOTES:
* All issues below are reported in terms of a fictional Vehicle type hierarchy. Examples are not
  meant to be taken as literal character-by-character repro steps.
* Within the below, when types appear in parentheses, it means any type that encompasses that type,
  including itself.
  * For example, (IConstruct) means any trait object the encompasses IConstruct, whereas IConstruct
    (no parentheses) means the literal IConstruct trait or its trait objects.
  * Also, (Construct) means any concrete type that implements the IConstruct trait, whereas
    Construct (no parentheses) means the literal Construct struct or its instances.


## Open Issues

Triage Info:

Triage ratings consist of a Priority and a Severity rating. Of these, the Priority rating is the
more straight forward: issues with higher numbers will be addressed before those with lower
numbers. In addition to this, the (hopefully!) rare Priority 0 marker indicates *all* other
development work must cease until this issue is resolved (or lowered in priority). Severity is a
little more amorphous and therefore harder to explain, but roughly:

0: Any potentially exploitable security issue. (Would also be Priority 0.)
1: Causes *any* application that performs the given legal action to crash, or blocks further
   development involving that action, and no workaround is known.
2: One of:
   * Causes *any* application that performs the given legal action to crash, or blocks further
     development involving that action, and a workaround *is* known.
   * Causes a *subset* of applications that perform the given legal action to crash under *some*
     circumstances, or blocks further development involving that action, and no workaround is known.
3: One of:
   * Blocks further "natural" development involving that action (e.g. the workaround would need to
     be undone later).
   * May cause significant confusion or difficulty for developers who consume (as opposed to
     extend) the hierarchy.
4: May cause significant confusion or difficulty for developers who extend (as opposed to consume)
   the hierarchy.
5: A feature that may be harder than average to understand or implement.
6: Requires fit-and-finish work.

Note that these ratings and their definitions are all subject to change.


### Under Investigation (*with* temporary solutions)


#### ISSUE #1: Compiler error within non-generic abstract functions when casting (IConstruct) trait
####           object to another interface also implemented by its underlying concrete type.

##### Status:
Investigation Active

##### Triage:
Priority: 1
Severity: 3

Notes: All information presented below applies to the mutable case as well (i.e. as_ivehicle_mut is
       equally affected).

##### Repro:
Send a Vehicle instance or the IConstruct formed from it to a function with a signature like:
```rust
fn abstract_func(iconstruct: &dyn IConstruct)
```
Within that function, invoke the following method:
```rust
iconstruct.as_ivehicle();
```

##### Result:
Compiler error "Borrowed data escapes outside of function."

##### Cause:
Currently unknown.
The only information in Rust's documentation regarding this error (E0521) refers to closures rather
than functions. The workaround presented there is to drop the type annotation, something that can't
be done for functions.

##### Solution:
Currently none.

##### Temporary Solution and its Problems:
Directly invoke the underlying struct, e.g.:
```rust
IVehicleDowncaster::as_ivehicle(iconstruct);
```
Problems:
  1. Developers will come across a confusing compiler error before they come across this Open
     Issues report that explains how to work around it.
  2. This will trip up developers both consuming and extending the object hierarchy.
  3. The temporary solution's syntax is far less natural than the syntax that's supposed to work.

##### Further Analysis:
* The as_ivehicle function comes from the DowncastsToIVehicle implementation for dyn IConstruct. It
  shows up in the list of what can be called in Visual Studio Code, so it seems like it should be
  callable.
* At least some scenarios that lead to this compiler message will show a different but related
  error if similar calls are made in the function in which the Vehicle instance and its IConstruct
  were made:
```rust
"`<(IConstruct) variable>` dropped here while still borrowed."
```
  * This is true even when the initial concrete type variable and both references are only in scope
    within the same, shared code block and should therefore reach the end of their durations with
    the same closing brace.

Ruled Out Changes:
* While it may seem that implementing DowncastsToIVehicle for &dyn IConstruct would be the right
  next step, doing so would also require implementing every prerequisite trait for &dyn IConstruct
  as well, including ConstructVirtuals, ConstructInstances, and ConcreteDivinator.
  * This can't be done because traits implemented for &dyn IConstruct would not be able to access
    the underlying Construct instance.
* Although it would still only constitute a workaround, we did see what would happen if we could
  access the dyn IConstruct behind the reference within the function's scope.
  * Unfortunately, we were thwarted either by Sized requirements or the same "Borrowed data escapes
    outside of function" problem we started with.


### Planned Investigations (*with* temporary solutions)

#### ISSUE #5: Need to find the most seamless possible way to initialize the RDH library stack.

##### Status:
Investigation Pending

##### Triage:
Priority: 2
Severity: 3

##### Description:
* Each RDH library requires initialization at runtime to function properly. It would be best to
  avoid requiring developers consuming (as opposed to extending) the type hierarchy to take action
  to ensure this happens.

##### Temporary Solution and its Problems:
Currently, consuming developers must invoke the init() function for the top-level RDH library
they're consuming.

Problems:
  1. This requires the consuming developer to take immediate action within their main() function to
     invoke the top-level init() function.
  2. This requires the consuming developer to understand RDH infrastructure enough to know which
     library's init() function must be invoked.
  3. If the consuming developer does not do this or does it incorrectly, the problem won't manifest
     itself until runtime.
  4. The resultant runtime error will not make it immediately obvious that calling the proper
     init() function will fix the problem.
  5. Calling init() multiple times will also cause a runtime error.
 
##### Preferred Potential Solution and Investigation Lead (if any):
Each library should initialize itself when the application launches. At this time, it appears that
the ctor crate may provide such functionality, but one of RDH's initial design goals was to avoid
any external dependencies. See https://docs.rs/ctor/latest/ctor/ for more information on the ctor
crate.
 
##### Alternative Solutions:
* Raise a compiler error if the proper init() function isn't called.
  * A compiler error is far easier to understand and correct than a runtime crash.
  * One would need to know *how* to raise a compiler error, though.
* Use a Once object to ensure that init() cannot be called more than once.


#### ISSUE #6: Adding new types is too cumbersome.

##### Status:
Investigation Pending

##### Links:
See Work Item #4 in Road Map.

##### Triage:
Priority: 2
Severity: 4

##### Description:
Despite extensive macros that declare and implement prerequisite traits and structs, creating a new
type for the RDH type hierarchy is still far too labor intensive.

##### Cause:
Because Rust doesn't support type inheritance, most of the glue that inheritance-based language
compilers insert during the build process had to be reverse engineered. This glue must therefore be
included within each type added to the type hierarchy. While macros have abstracted the bulk of
this glue, adding a new type is still substantially more complicated than doing the same in an
inheritance-based language.

##### Temporary Solution and its Problems:
We've added two templates to help guide someone through creating a new type, one for those new to
the RDH framework, and one for those who have some understanding of how to proceed.

Problems:
  1. Following the templates is *still* far more work than adding a new type to an inheritance-
     based language.
  2. Invoking macros, updating corresponding invocations within ancestor types, and updating the
     library's initialization code is tedious and potentially error prone.

##### Analysis:
There are essentially three implementation Levels within each type defined for any type hierarchy.
From the bottom up, these are:

  Level 1) Infrastructure: Code necessary to work with type instances within various abstract forms
                            and to cast between them.
                           This is provided by the compiler in other languages.
                           Thaumaturgy does a great job of abstracting this out of individual
                            developers' hands.

  Level 2) Structure:      Code necessary to establish relationships between the new type and other
                            types within the type hierarchy as well as to tie the type to its
                            necessary infrastructure.
                           This is provided by the compiler in other languages.
                           The new type templates walk developers through building out this
                            structure, but it's still a wholly manual process.

  Level 3) Functionality:  Virtual and non-virtual methods, overrides for ancestor virtual method
                            implementations, and any necessary (private or protected) supporting
                            code.
                           (To my knowledge), no compilers provide anything at this level.
                           This is the part developers actually *want* to implement themselves.

To make RDH more compelling, we have to find a way to make Structure implementation less painful.
Unfortunately, it's more complicated to automate Structure than Infrastructure, so it seems likely
that macros will be insufficient to solve this problem.

##### Preferred Potential Solution and Investigation Lead (if any):
Frameworks that extend a language's functionality beyond what it can natively support are nothing
new, and most of them also have far more required boilerplate than would be ideal. The general
solution is to employ code generation, much as Visual Studio does when a project template is
selected, and as Cargo does when creating a new crate.

An ideal solution would be a simple CLI script that, when provided with the new type's name and the
name of the type it directly encompasses, would build out Structure implementation automatically,
including updating ancestor types and library initialization as required-- with a preview and
opportunity to reject changes. Between Thaumaturgy and this script, creating a new RDH type would
become almost as simple as creating a new type in another language, and in some ways even easier.
Unfortunately, we have no leads at this time and so would likely have to build this script from
scratch.
 
##### Alternative Solutions:
None currently known.


#### ISSUE #7: Improving Thaumaturgy's Transmutation component performance (if needed).

##### Status:
Investigation Pending

##### Triage:
Priority: 3
Severity: 5

##### Description:
Transmutation involves determining the concrete type underlying the trait objects sent to it for
casting. This process resembles a game of Go Fish, with an if / else if / else if / else structure
making successive guesses for the type. Transmutation mitigates this problem to some degree by only
testing types defined in the library in which the concrete type was defined. Using a match
statement would naturally provide better performance than a series of conditionals, were its use a
possibility.

##### Cause:
Rust's match statement doesn't support the invocation of functions for its branches, but obtaining
types to test against currently requires a function call. Given Rust's ideals and its goals, this
requirement makes sense, but it does complicate otherwise simple tasks.

##### Temporary Solution and its Problems:
Go Fish!

Problems:
  1. Although Rust's compiled code may be fast enough on modern hardware that the performance
     difference doesn't matter in a practical sense, it's clear that match would offer better
     performance than Go Fish.
  2. While the oft-cited rule of thumb for maximum hierarchy depth is three levels, once a
     developer is empowered to add more types, we have no way reasonable way of controlling how
     many they add. We therefore have no upper bound (other than max u128!) on the number of
     potential matches Go Fish would have to try.

##### Analysis:
At this time, it's unclear how beneficial this change would be within real world code. This makes
it difficult to determine the relative priority for this work. We also have to consider how these
identifiers could be stored in such a way that the match statement could use them without undue
compromise on other design goals and without needlessly complicating the code. Note that Rust's own
TypeId is currently disallowed within match statements as well.

##### Preferred Potential Solution and Investigation Lead (if any):
Pending further investigation.
 
##### Alternative Solutions:
None currently known.


#### ISSUE #9: Type registry keys preclude the use of grapheme clusters within RDH type names.

##### Status:
Investigation Pending

#####  Triage:
Priority: 4
Severity: 4

##### Description:
Due to the way type registry keys must be formed, RDH type names may not contain characters that
require the use of grapheme clusters.

##### Cause:
While Rust has advised against relying on `type_name<T>()` due to variability within its returned
values, it remains the only *known* available option to act as the basis for type registry keys.
(See Issue #8 for further information.) Because of this, type registry keys are formed from the
extraction of the fully qualified RDH type name from the often larger `type_name<T>()` return
value. String parsing within Rust can be done per byte and per character, but not per grapheme
cluster, and so grapheme clusters cannot be used in anything that needs to be parsed.

##### Temporary Solution and its Problems:
Developers extending the RDH type hierarchy must choose interface, type, and module names that do
not contain grapheme clusters.

Problems:
  1. Assuming Rust supports grapheme clusters within its type names (something that has not yet
     been confirmed), RDH introduces an artificial limitation on the characters than can be used
     within its type names, limiting the expressiveness available to developers when naming their
     types.

##### Analysis:
Sadly, because determining a type's name requires parsing the return value of calls to Rust's
type_name<T> function, we will be unable to support characters that require the use of grapheme
clusters. As it's unclear at this time whether such characters are supported within Rust
identifiers, it's possible this issue will become moot. We're leaving this open until we have
confirmation one way or the other.

If Rust *does* support grapheme clusters within its type names, we'll tie this to Issue #8. If and
when an alternative to type_name<T> is discovered, we'll also unearth this issue and hopefully be
able to resolve it. Otherwise, we will close this issue as External / Can't Fix and revisit only if
Rust adds grapheme clusters support to its type names and/or during string parsing at some point.

##### Preferred Potential Solution and Investigation Lead (if any):
Expand the legal character set to include grapheme clusters. No leads at this time.
 
##### Alternative Solutions:
None currently known.


#### ISSUE #10: Type registry keys require RDH type names to adhere to Anglocentric characters.

##### Status:
Investigation Pending

##### Triage:
Priority: 3
Severity: 4

##### Description:
We don't currently have a full understanding of the set of characters legal within Rust
identifiers, nor how to express them properly within the parsing process. Because of this, we
elected to temporarily constrain RDH type names to the set of characters commonly legal in other
programming languages for fully qualified type names: {A..Z}, {a..z}, {0..9}, {_}, and {:}

##### Cause:
While Rust has advised against relying on `type_name<T>()` due to variability within its returned
values, it remains the only *known* available option to act as the basis for type registry keys.
(See Issue #8 for further information.) Because of this, type registry keys are formed from the
extraction of the fully qualified RDH type name from the often larger `type_name<T>()` return
value. Part of the parsing process requires defining the set of legal characters for a fully
qualified type name. We are uncertain whether the legal character set we chose is even close to
complete.

##### Temporary Solution and its Problems:
Developers extending the RDH type hierarchy must choose interface, type, and module names whose
characters all fall within the set {A..Z}, {a..z}, {0..9}, {_}, and {:}.

Problems:
  1. This Anglocentric list excludes at least some portion of the character set within most
languages, and many languages are excluded altogether. This isn't the 1980s. We shouldn't be
forcing people around the globe to write code in English anymore.

##### Analysis:
In most programming languages, the full set of characters legal within identifiers and the means
through which they may be expressed within the parsing process is trivial to determine. Because
Rust supports Unicode at all levels (other than grapheme clusters anyway), it would seem specious
to assume their identifier names are limited as they are in other languages. Initial efforts to
find a definitive answer to this question came up empty.

##### Preferred Potential Solution and Investigation Lead (if any):
Expand the legal character set to include all characters legal in Rust identifier names. No leads
at this time.
 
##### Alternative Solutions:
None currently known.


#### ISSUE #11: Exporting protected and private macros is not ideal

##### Status:
Investigation Pending

##### Triage:
Priority: 3
Severity: 5

# Description:
All macros defined within the Thaumaturgy module's Transmutation component are exported, making
them public. Two of them are supposed to be protected, and the other two are supposed to be
private.

# Cause:
Rust has no known way of modeling the protected access modifier through its visibility scopes. Rust
macros can be leveraged across library boundaries only if they're exported. A macro that's invoked
by an exported macro must itself be exported, or the compiler won't be able to find it.

##### Temporary Solution and its Problems:
All macros are exported.

Problems:
  1. Private macros should not be invokable outside of the module in which they're defined, but any
     exported macro is invokable from anywhere.
  2. Protected macros should only be invokable within encompassing types, but without a way to
     model this, they must be exported (i.e. exposed publicly) to be called across libraries.

##### Analysis:
While we can do nothing about Rust's visibility scopes, that the protected access modifier can't be
modeled only explains half the problem. The private macros do not *have* to exist. They were pulled
out because each of them is leveraged within both the Construct-only and Construct-encompassing
branches within the provision_transmutation macro.

##### Preferred Potential Solution and Investigation Lead (if any):
Likely the only thing we can achieve is to hide the private macros. We may be able to find a clever
way to use macro parameters to simulate a conditional and therefore obviate the need for two
branches within provision_transmutation. This would allow us to fold the two private macros
directly back into provision_transmutation's implementation.
 
##### Alternative Solutions:
None currently known.


## Known Issues


### Resolved External / Can't Fix (*without* workarounds)

#### ISSUE #2: The RDH type hierarchy must be extensible across libraries, but enabling this also
####           grants consuming developers access to encompassed objects.

##### Status:
Investigation Closed

##### Notes: All information presented below applies to the mutable case as well (i.e.
#####        as_ivehicle_mut is equally affected).

##### Repro:
Within a consuming crate, create a new Vehicle instance and then access its underlying concrete
Construct instance:
```rust
         let vehicle = Vehicle::new();
         let hand_in_cookie_jar = vehicle.construct();
```

##### Result:
Developer now has access to the encompassed concrete Construct instance of what is, in fact, a
Vehicle instance.

##### Expect:
Encompassed concrete instances should be "protected," i.e. accessible only by types that extend the
RDH type hierarchy.

##### Cause:
Rust has no native idea of type inheritance, and so it provides no way to model a "protected"
access modifier.

##### Solution:
NONE. There is no known way to adequately protect these encompassed concrete instances with Rust's
current privacy model.

##### Workaround:
NONE.

##### Resolution:
External. Only the Rust folks can change their privacy model.

##### Further Analysis:
Part of the intent behind RDH is to allow third party developers to extend the type hierarchy in
their own libraries. The closest Rust offers to "protected" is pub(crate), which is more-or-less
equivalent to "internal" in C#. A pub(crate), or internal, level of privacy is not sufficient,
because it's too restrictive to grant access to extending libraries. At the same time, a pub, or
public, level of privacy is too permissive, because it allows the entire world access.

This is not good!

There are three known problems with allowing consuming developers access to the encompassed
concrete instances. In increasing order of severity:
1. Developers familiar with inheritance-based languages but new to Rust may struggle to keep in
   mind that the resulting instance would *not* be the same object, but instead a completely
   different object. This gap in understanding could lead to all kinds of chaos.
2. Nothing would tie the instance yielded from the cast back to its encompassing instance. There
   would be no (good) way to determine later which Sedan instance yielded this Car instance, or
   whether it was yielded from a Sedan instance at all, or even whether the instance was initially
   declared as a Car type, a Sedan type, or some other less abstract type. Because of this, it
   would be impossible to cast the instance back to its initial type.
3. This would break polymorphism! As nothing would tie the Car instance back to its encompassing
   Sedan instance, calling a virtual function on the Car instance would invoke the implementation
   from the *Car* type, *not* the Sedan type. So, for example, while a call such as
   *car.show_type()* would be expected to yield "Sedan," it would instead yield "Car." This would
   clearly be buggy behavior.

These are the exact problems that led to our decision to not support casting between concrete
types, even though doing so is incredibly common in inheritance-based languages.

##### Ruled Out Changes:
Of all Rust's visibility scopes, only pub(in path) was even close to providing a mechanism to
approximate "protected" access. The thought that RDH extension libraries could be nested within
each other was entertained, but pub(in path) could only reach as far up as the crate's /src folder.
Nesting each subsequent library within the crate folder for its parent was a tough enough sell;
nesting them inside the src folder would undo any pretense of separation of concerns.


#### ISSUE #8: Type registry keys potentially fragile.

##### Status:
Investigation Closed

##### Description:
While Rust has advised against relying on `type_name<T>()` due to variability within its returned
values, it remains the only *known* available option to act as the basis for type registry keys.
(See "`impl dyn Any + Send + Sync::type_name<T>`" [definition] for more information.) Although they
may be embedded within other types (e.g. Box<T>), from our own empirical experience, it appears
that `type_name<T>()` always returns the proper fully qualified type names we require. Given this,
and considering the lack of alternatives, RDH uses the substring of `type_name<T>()` return values
that's *expected* to contain the needed fully qualified type for type registry keys.

[definition]: https://doc.rust-lang.org/beta/src/core/any.rs.html

All that said, we realize that empirical experience is not a data contract.

##### Cause:
The only way to identify a type in Rust that doesn't require that all involved types must have
'static lifetime is through calls to type_name<T>, but Rust provides no guarantees as to the form
values returned from type_name<T> will take, nor that what's returned won't change over time.

##### Temporary Solution and its Problems:
Having noted that what we need is always a substring within what type_name<T> returns, currently we
extract that substring from the value Rust gives us.

Problems:
1. Without a data contract we can't be absolutely certain any parsing algorithm will remain valid
   over time.
2. There is some concern due to the cautions within the method's documentation that type_name<T>
   may become deprecated at some point.

##### Analysis:
If Rust changes the way type_name<T> works in an unpredictable way or removes it altogether without
replacing it with something better, RDH infrastructure will stop functioning. Overall we believe
there's little risk of this, as Rust appears to be (slowly!) moving forward with dynamic typing and
reflection support (likely because growing the language into a popular application development
platform pretty much requires them to do so).

##### Preferred Potential Solution and Investigation Lead (if any):
We have no leads on an alternative basis for registry type keys whatsoever.
 
##### Alternative Solutions:
None currently known.


### Resolved External / Can't Fix (*with* workarounds)

#### ISSUE #3: Compiler error within generic functions when downcasting (IConstruct) trait object
####           to a downstream interface also implemented by its underlying concrete type.

##### Status:
Investigation Closed

##### Notes: All information presented below applies to the mutable case as well (i.e.
#####        as_ivehicle_mut is equally affected).

##### Repro:
Send an (IConstruct) formed from a Vehicle instance to a function with a signature like:
```rust
fn generic_func<T>(iconstruct: &T) where T: IConstruct + ?Sized
```
Within that function, invoke the following method:
```rust
iconstruct.as_ivehicle();
```

##### Result:
Compiler error "No method named `as_ivehicle` found."

##### Cause:
The generic constraint tells the compiler that iconstruct implements IConstruct and might not be
sized. This isn't enough for the compiler to conclude that what was sent in is an &dyn IConstruct,
though.

##### Solution:
NONE. There is currently no way in Rust to assert that the generic parameter must be a trait
object.

##### Workaround:
Directly invoke the underlying struct, e.g.:
```rust
IVehicleDowncaster::as_ivehicle(iconstruct);
```

##### Resolution:
External. Only the Rust folks can change how constraints are modeled and handled by the compiler.

##### Further Analysis:
Upcasts and casts to concrete types are not affected by this issue because UpcastsToI* and
Necromances are both implemented for all types that implement IConstruct, etc.. This can be done
because UpcastsToI* applies to the current type and *all* (IConstructs) that follow it, and
Necromances applies to all (IConstructs) of all types. Downcasts, on the other hand, apply only to
upstream, encompassed, types.

##### Ruled Out Changes:
While DowncastsToI* could be implemented on either *dyn* I* or its concrete type, the problem
manifests either way. As there's no overlap between types that implement UpcastsToI* and those that
implement DowncastsToI* for any given type, combining these into a single trait was considered.
Unfortunately, Rust currently only allows the default trait implementation to be overridden. For
these methods, however, "self" must be returned for the default Upcast scenario, something the
compiler doesn't allow. The compiler is unable to determine which traits would be implemented by
self from within the trait declaration.

A further possibility was to use conditionals to branch into downcasting behavior when necessary
for a given type. Doing so, however, would require changing trait implementations for types defined
previously, possibly even within other libraries. Rust doesn't support doing this (and very likely
shouldn't).


#### ISSUE #4: Compiler error when passing (IConstruct) trait object from a generic function to a
####           non-generic abstract function.

##### Status:
Investigation Closed

##### Notes: All information presented below applies to the mutable case as well (i.e.
#####        as_ivehicle_mut is equally affected).

##### Repro:
Send an (IConstruct) formed from a Vehicle instance to a function with a signature like:
```rust
fn generic_func<T>(iconstruct: &T) where T: IConstruct + ?Sized
```
      Within that function, send iconstruct on to a function with a signature like:
```rust
fn abstract_func(iconstruct: &dyn IConstruct)
```

##### Result:
Compiler error "The size for values of type `T` cannot be known at compilation time."

##### Cause:
The generic constraint tells the compiler that iconstruct implements IConstruct and might not be
sized. This isn't enough for the compiler to conclude that what was sent in is an &dyn IConstruct,
though.

##### Solution:
NONE. There is currently no way in Rust to assert that the generic parameter must be a trait
object.

##### Workaround:
Cast iconstruct to an &dyn IConstruct *via Transmutation*, e.g.:
```rust
abstract_func(iconstruct.as_iconstruct());
```
##### Resolution:
External. Only the Rust folks can change how constraints are modeled and handled by the compiler.
