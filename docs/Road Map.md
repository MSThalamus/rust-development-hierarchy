# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

Road Map.md
Updated February 7, 2023

Includes:
* Planned Work Items

IMPORTANT NOTE: THIS IS A WORK IN PROGRESS. Please read documentation carefully to determine what,
                if anything, is considered permanent.

NOTES:
* This is no one's sole occupation at the moment.
  * As such, no estimated dates are given.
* All issues below are reported in terms of a fictional Vehicle type hierarchy. Examples are not
  meant to be taken as literal character-by-character repro steps.
* Within the below, when types appear in parentheses, it means any type that encompasses that type,
  including itself.
  * For example, (IConstruct) means any trait object the encompasses IConstruct, whereas IConstruct
    (no parentheses) means the literal IConstruct trait or its trait objects.
  * Also, (Construct) means any concrete type that implements the IConstruct trait, whereas
    Construct (no parentheses) means the literal Construct struct or its instances.


## WORK ITEM #1: Add unit tests that provide adequate code coverage.

### Status:
Pending

### Triage:
Priority: 1

### Description:
Add unit tests via Rust's built-in testing harness, covering as much of the code base as possible,
ideally 100%. Do not add full coverage for code known to be placeholder, i.e. Construct Name and On
Click, all of UIElement, and all of Checkbox at this time.

### Details:
It's especially important to be sure working with collections of abstract types works as expected.
* With a vec<box<dyn IConstruct>> populated with mixed types, make sure:
  1. All public IConstruct methods can be invoked AND THAT THE CORRECT VIRTUAL METHODS ARE INVOKED.
  2. That we can attempt to cast each element to ICar, correctly skipping over actual Constructs
     without crashing, either by testing is() first or checking expect().
  3. That for all successfully cast ICars, all public ICar methods can be invoked AND THAT THE
     CORRECT VIRTUAL METHODS ARE INVOKED.

### Stretch:
Find a tool to measure code coverage.


## WORK ITEM #2: Populate IConstruct with real type hierarchy root functionality.

### Status:
Pending

### Triage:
Priority: 1

### Description:
IConstruct's current properties and virtual methods are placeholders. We need to add the real
stuff, possibly including but not necessarily limited to:
* clone - virtual
  * Possibly include bool parameter for shallow vs deep copy?
  * Use Rust's built-in if it will work.
* equals - virtual
  * This is field-by-field comparison.
  * Use Rust's built-in if it will work-- pretty sure it will.
* get_hash_code - virtual
  * Does Rust have a built-in for this?
* reference_equals - NON virtual
* to_string - virtual

Two things to keep in mind:
  * If Rust has an equivalent that works the same, is there a reason to add a method to the root?
    * Especially a virtual method-- remember that all virtual methods incur a cost for all
      encompassing types that follow.
  * How comfortable do we need to make this for people familiar with .NET, Java, etc., vs how
    quickly do we want them to acclimate to Rust's built-ins?
    * If we opt to lean to the side of Rust's built-ins, should provide mapping between what would
      have been functions to Rust's built-ins in docs.

### Stretch:
Consider whether to make Construct abstract, i.e. non-instantiable.


## WORK ITEM #3: Plan the substantial example type hierarchy.

### Status:
Pending

### Triage:
Priority: 1

### Description:
The current UIElement and Checkbox types are placeholder only. We need a non-trivial (really rather
substantial) example to justify the use of inheritance. Games and UI are good candidates-- but we
need the layers to add more than one or two lines of code, ideally approx. equal contribution to
their parents to show why avoiding code duplication is a good thing. Remember that we want to show
cross-library boundaries as well.

Note that the type_dispatch_examples module depends upon UIElement, so that will have to be
reworked to fit into the new hierarchy.

### Stretch:
Ideally we want to show more than a single path, but the example should not be contrived any more
than necessary considering we don't actually have a game engine or OS to drive. Do we want to point
out that multiple inheritance will work? Or let them discover that on their own? ('cause that ain't
gonna be controversial at all, is it? :D) Also, unlike C#, can skip generations, i.e. invoke
grandparent implementation rather than only the parent.


## WORK ITEM #4: Think through approach for Structure code generation.

### Status:
Pending

### Links:
See ISSUE #6 in Knowledge Base.

### Triage:
Priority: 2

### Description:
The NEW_TYPE_TEMPLATE and BARE_NEW_TYPE_TEMPLATE are skirting the edge of insufficiency-- creating
a new type is still too much work. I mean, I just went through it again, and it's a serious PITA.

Given existing module, new type name, and parent type name, would need to find:
* Module parent to add mod statement to
  * It's okay to ask them to create the parent module first
* File containing parent definition
* Library lib.rs or main.rs

Also need info from one or more of struct declaration, interface trait declaration, impl<T> of
trait for <T> including struct, and the provision_transmutation! macro call. Will need to update
that macro call for ancestor types. Likely a good idea to show preview of changes and ask for
approval, 'cause we'd be editing other files. Finally, spit out the type's module file, formatted
just like what following the templates would produce.

### Stretch:
* Can we add macro to wrap existing macros, so we end up with a single, single-line macro call at
  bottom of code generated type files? We'd need to find a way to append text to end of macro
  parameters-- this might not be possible.
* Write this in Rust? It's okay not to eat our own dog food for the first version to improve the
  flow sooner rather than later. Subsequent releases should be chowing down, though.
  * (If you, reading this, feel insulted by the term dog food, don't be. It's a good, olde-timey
    Microsoft thing, which means to use the thing you're building to build the thing itself, etc..)


## WORK ITEM #5: Port C++ / D ReaderWriterLockSlim-like implementation to Rust

### Status:
Pending

### Triage:
Priority: 3

### Description:
Rust's reader/writer lock implementation only has two possible modes of operation: wait until the
end of time or don't wait at all. The better approach is to define a timeout and to handle failures
gracefully if such a timeout is reached. 

We already have an implementation of a ReaderWriterLockSlim-like primitive that includes this
functionality (and more), but it exists only in C++ and D forms. Porting it to Rust could prove...
interesting?

Once ported, we should use it in all places where std::sync::RwLock is used now.

