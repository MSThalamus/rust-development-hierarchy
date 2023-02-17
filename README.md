# Rust Development Hierarchy
Version 0.5.0: Proof-of-concept milestone

README.md
Updated February 6, 2023

IMPORTANT NOTE: THIS IS A WORK IN PROGRESS. Please read documentation carefully to determine what,
                if anything, is considered permanent.

## Motivation

It's well known that the Rust Programming Language doesn't support the kind of type inheritance
found in other object-oriented programming languages such as C++ or C#. This has led some to
question whether Rust should count as an object-oriented programming language at all. While many
compelling arguments to justify labeling Rust as an OOP language have been offered, these arguments
and the question itself are really little more than red herrings. The questions of whether Rust
qualifies as an OOP language and what features are required for any language to be labeled as such
are purely academic. From the point of view of those with boots on the ground, there's only one
question that truly matters: "Is this language a compelling choice for the problem I'm trying to
solve?" The language features necessary to answer that question with a resounding "yes" vary
greatly from one problem space to the next. As with any other feature, whether a language supports
type inheritance is meaningless if its inclusion is unhelpful in solving the problem at hand, yet
wholly meaningful otherwise.

One of the arguments offered against including type inheritance within Rust is that inheritance is
overused, that it's used in places where other modalities would produce objectively better code.
Given the nature of this project, it may be surprising to hear that I absolutely *agree*! An entire
generation of developers came into the industry having been taught that inheritance is the
*correct* approach under *all* circumstances. That this led to overuse is wholly predictable, but
also understandable. Paradigm shifts always lead to sweeping changes in priorities and
perspectives, something I've witnessed many times over. My first programming languages were
unstructured. When C, Pascal, and other structured, procedural languages were gaining momentum,
unstructured languages were suddenly anathema. Then, with C++ and later Java and C# came the
object-oriented revolution. Suddenly structured programming was passé, and everything-- even
main()-- *had* to be built into an object, unnecessary boilerplate and all.

The industry is currently undergoing another shift, this time away from some object-oriented
approaches, chief among them inheritance hierarchies. Overall, I find this a healthy change! I can
say that in my quarter century of professional experience-- and a good decade as a hobbyist prior--
95% of the time or more a deep type hierarchy would have been the wrong approach. On almost every
occasion where I needed to represent a group of classes abstractly, a single interface was
sufficient to meet my goals. But that last 5% of the time *still counts*. There are simply some
areas where a "deep" type hierarchy (meaning, at the very least, more than a single interface to be
sure) *is* the right choice. Two that readily come to mind from my own experience lie within the
development of graphical user interfaces and games. Sometimes I adopted deep hierarchies because it
naturally fit my problem space and sometimes I did so because the platform I needed to consume had
done so. In either case, though, I could not have proceeded using a language that was incapable of
modeling objects in this manner, and so I could not have chosen Rust.

I do want to emphasize that, overall, I have great respect for the goals and priorities of the Rust
Programming Language. Full memory management without garbage collection offers all the performance
of traditional natively compiled languages while still yielding far *safer* code than any of them
can provide. I remember well scrambling when the Code Red worm sprang up in July of 2001, one of
many high-profile exploits that surfaced that year. I also remember dropping absolutely every other
priority the following February for the entire month in favor of a seek-and-destroy mission to find
and squash potential buffer overruns within Internet Explorer, something also done by every single
other tester and developer, company-wide. That Rust's approach to memory management takes this
chore out of the hands of individual developers greatly decreases the chance that exploitable bugs
such as these could exist within the code it produces. Despite its truly epic learning curve, Rust
could perhaps offer the answer to the decades-long question of how to build applications that are
performant, robust, and *fully* secure.

Well, perhaps *safe* Rust could do so anyway.

This is where my "overall" qualifier comes into play. A great amount of emphasis within Rust is
placed upon memory safety, and yet it seems the language's designers also understood that forcing
developers to always write safe Rust code would limit the domains within which the language could
be used. At the same time, a great amount of *discussion* in Rust forums regarding type inheritance
can essentially be boiled down to an almost zealously dogmatic "Type Inheritance Considered
Harmful" mantra. It seems clear, however, that between direct pointer manipulation and type
hierarchies, the former holds the potential for *far* more disaster than the latter ever could. Yet
the former is an incorporated language feature while the latter is eschewed. I find this quite
frankly baffling. As much as Rust needed to embrace the occasional use of unsafe code in order to
meet the needs of low-level systems developers, for the language to grow I believe it must also
embrace the occasional use of type hierarchies in order to meet the needs of high-level application
developers.

To employ an age-old metaphor, in the end, type inheritance is just another tool within a well-
rounded toolbox. For any given task, some tools are appropriate and some are not. Just as one would
(hopefully) not try to drive a nail into drywall with a sledgehammer, one would also (hopefully)
not build a deep type hierarchy when a single simple interface would suffice. In my opinion,
precluding the use of type hierarchies to safeguard code from overengineering makes no more sense
than banning the sale of sledgehammers to protect drywall from overzealous carpentry.

Again, I wholly agree that type hierarchies have been overused and that the potential for abuse of
such a feature does exist. I would strongly urge anyone thinking about implementing a deep
hierarchy via type inheritance to reconsider, to look for alternative modalities. From experience,
however, I also know that occasionally a type hierarchy *is* the correct approach. Blindly
refraining from doing something is hardly better than blindly proceeding. The Rust Development
Hierarchy (RDH) is offered within this light, not as a suggested course of action, but as a means
to an end for developers who want to leverage the benefits of Rust even within problem spaces best
modeled hierarchically.

Gene DeClark
January 2023 


## Goals

1. Provide a framework through which
   a. Developers who are already familiar with consuming the inheritance features within
      programming languages such as C++ and C# can, in as familiar a manner as possible, *consume*
      Rust-based type hierarchies
      * including abstract instance representation, casting between interfaces implemented by an
        instance's concrete type, and casting from an interface to a concrete type
   b. Developers who are extensively experienced in building type hierarchies within such
      inheritance-based languages can *produce* similar type hierarchies within Rust

   While other, less familiar, means of representing type hierarchies have been suggested, it seems
   that no consensus on their idiomatic representation has yet surfaced. (See, for example, this
   forum [post].)

[post]: https://users.rust-lang.org/t/how-to-model-inheritance-hierarchy/33380

2. Adopt no *external* dependencies (at least for the initial commit).
   * No external crates are leveraged within the RDH framework. Only std:: and our own bespoke
     crates are used.


## Crates

While this repo includes several crates, only one of them is of fundamental importance:

* rdh
  * Contains the RDH framework and its base type, Construct (planned as similar to the Object class
    in .NET).

Additional crates:

* rdh_extension_example
  * Contains an example extension to the RDH framework (analogous to a .NET assembly within which a
    third-party developer would implement bespoke types derived from Object).
    * At this time, code within this crate is example, placeholder, or both and will be almost
      entirely replaced going forward.
  * Also contains templates that can be used to implement new types.
* rust_development_hierarchy
  * A placeholder for the application that ultimately consumes the type hierarchy built within
    rdh_extension_example (or real extension crate).
  * Also contains project documentation beyond this ReadMe.


## Documentation:

Additional documentation is listed below in recommended reading order for each development role.
These documents can all be found within the /docs folder.

### Developers extending the RDH framework into bespoke type hierarchies

  * RDH Inheritance Model.md
    * Largely using terms from inheritance-based languages, explains how inheritance concepts can
      and can't be realized through RDH and if/how these map to native Rust concepts
      * This includes the kinds of casts RDH can and can't support
  * Extenders' Handbook.md
    * Reference written to help developers extending the RDH framework to build their own type 
      hierarchies
  * BASE API Reference for RDH Consumers.md
    * BASE API reference written to help developers who implement applications that consume RDH
      type hierarchies
    * *To be forked for each type hierarchy into a single API document reflecting both base RDH*
      *features and the bespoke type hierarchy itself*
  * Knowledge Base.md
    * Known issues, either under investigation or retained for tracking and informational reasons
  * Road Map.md
    * Next steps for RDH development

  As many questions that may arise from reviewing the project's code may be answered within them,
  it would be advisable to at least read the knowledge base and road map before reviewing the RDH
  code base.

### Developers consuming type hierarchies implemented by other developers

  * RDH Inheritance Model.md
    * Largely using terms from inheritance-based languages, explains how inheritance concepts can
      and can't be realized through RDH and if/how these map to native Rust concepts
      * This includes the kinds of casts RDH can and can't support
  * API Reference for *type hierarchy* Consumers.md
    * API reference written to help developers implementing applications that consume bespoke type
      hierarchies
    * *Specific document to be produced by those building such type hierarchies*
  * Knowledge Base.md
    * Known issues, either under investigation or retained for tracking and informational reasons
  * Road Map.md
    * Next steps for RDH development

  As many questions that may arise when consuming an RDH type hierarchy may be answered within
  them, it would be advisable to at least read the knowledge base and road map before reviewing the
  RDH code base.
