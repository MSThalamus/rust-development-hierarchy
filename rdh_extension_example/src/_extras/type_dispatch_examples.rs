// ************************************************************************************************
// Copyright 2023 Gene DeClark and Contributors within this file's version control history
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
// ************************************************************************************************

//!
//! Within comments throughout the codebase, type names in parentheses indicate an "is a"
//!  relationship, i.e. that type and any type that encompasses that type.
//! For example:
//! * (ICheckbox) means an ICheckbox trait object or any trait object encompassing ICheckbox.
//! * (Checkbox) means the Checkbox struct or any concrete type that implements ICheckbox.
//! Whereas:
//! * ICheckbox (no parentheses) means the literal ICheckbox trait or its trait objects.
//! * Checkbox (no parentheses) means the literal Checkbox struct or its instances.
//!

//!
//! type_dispatch_examples.rs
//!
//!  Contains:
//!  * Examples for working with (IConstructs) of unknown concrete types through cast-and-dispatch
//!    approaches.
//!
//!  Notes:
//!  * These example functions work with mutable (IConstructs), but morphing them to support
//!    immutable references should be trivial.
//! 
//! THIS IS A WORK IN PROGRESS.
//!   At this time, function signatures are subject to change without notice.
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// Public example functions
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// dispatch_iconstruct_by_concrete_type:
///   Demonstrates a cast-and-dispatch approach for processing (IConstructs), whose concrete
///   (Construct) type is unknown at compile time, leveraging functions that expect a specific
///   concrete type.
///
/// Generic parameters:
/// * T (IConstruct + Necromances + ?Sized): Represents all (IConstructs).
///
/// Parameters:
/// * iconstruct (&mut T): The reference to the (IConstruct) object to process.
///
/// Panics:
/// * When the specified (IConstruct) object doesn't correspond to any supported (Construct) type.
/// * When downcasting fails after the concrete type was positively identified.
///   * Considering that this function determines the type of the concrete instance before calling
///     .as_concrete_mut(), this *should* never occur.
///
/// Notes:
/// * This function implements the rough equivalent to the following C# code:
///
/// ```
///     // C++-style casts below included for illustration as they're required within RDH code.
///     // They would not be required in C# as coercion would cast implicitly.
///     if (iconstruct is UIElement)
///     {
///         ProcessUIElement((UIElement)iconstruct);
///     }
///     else if (iconstruct is Construct)
///     {
///         ProcessConstruct((Construct)iconstruct);
///     }
///     else
///     {
///         throw new InvalidOperationException("The specified (IConstruct) object doesn't
///                                              correspond to a supported (Construct) type.");
///     }
/// ```
///
/// * The need to process collections of type hierarchy instances of mixed concrete types, unknown
///    concrete types, or both is common.
///   * Under such circumstances, the actual concrete type of at least a subset of these instances
///     is unknowable at compile time (something that presents some degree of challenge in a
///     language like Rust).
/// * For this example, the specified (IConstruct) trait object reference is cast to its actual
///   concrete type (discoverable at run time) and then dispatched to a handler for that struct
///   type.
///
/// * Whether casting is advisable, or required, depends on the work that needs to be performed.
///   * Casting to a concrete type improves performance, as functions will be invoked via static
///     rather than dynamic dispatch.
///     * Due to the overhead introduced by casting, more benefit will be afforded to more
///       extensive workflows.
///   * Invoking methods defined in downstream traits requires casting to at least the trait object
///     in which the required functionality is defined.
///     * For example, when a &mut dyn IConstruct is passed in whose underlying concrete type is
///       UIElement, casting to UIElement (or at least &mut dyn IUIElement) is required before new
///       functions contributed by IUIElement can be invoked.
///
/// * Because Rust currently offers no way to specify that a parameter must be a trait object, this
///   function will also accept concrete instances.
///   * While this will work just fine, it would be better to send such instances to a different
///     function that's designed to handle them without needlessly casting them to exactly what
///     they already are.
///
pub fn dispatch_iconstruct_by_concrete_type<T>(iconstruct: &mut T)
    where T: IConstruct + Necromances + ?Sized
{
    // Implementation notes:
    // * Should new types need to be supported, separate dispatch conditionals will need to be
    //   added to this function and corresponding handlers will need to be implemented below.
    //   * There must be a one-to-one relationship between supported types and handlers.

    if iconstruct.is(TypeRegistry::type_identifier_of::<UIElement>())
    {
        process_ui_element(iconstruct.as_concrete_mut::<UIElement>().expect(
          "dispatch_iconstruct_by_concrete_type<T>(): Unable unearth established UIElement!"));
    }
    else if iconstruct.is(TypeRegistry::type_identifier_of::<Construct>())
    {
        process_construct(iconstruct.as_concrete_mut::<Construct>().expect(
          "dispatch_iconstruct_by_concrete_type<T>(): Unable to unearth established Construct!"));
    }
    else
    {
        panic!("dispatch_iconstruct_by_concrete_type<T>(): The specified (IConstruct) object \
                doesn't correspond to a supported (Construct) type.");
    }
}

///
/// dispatch_iconstruct_by_implemented_interface:
///   Demonstrates a cast-and-dispatch approach for processing (IConstructs), whose concrete
///   (Construct) type is unknown at compile time, through functions that expect a trait object of
///   a different type (or a concrete instance that can be coerced into the that type).
///
/// Generic parameters:
/// * T (IConstruct + ?Sized): Represents all (IConstructs).
///
/// Parameters:
/// * iconstruct (&mut T): The reference to the (IConstruct) object to process.
///
/// Panics:
/// * When downcasting fails after an implemented interface was positively identified.
///   * Considering that this function checks implemented interfaces before casting,
///     this *should* never occur.
///
/// Notes:
/// * This function implements the rough equivalent to the following C# code:
///
/// ```
///     // C++-style casts below included for illustration as they're required within RDH code.
///     // They would not be required in C# as coercion would cast implicitly.
///     if (iconstruct is UIElement)
///     {
///         ProcessAnyIUIElement((IUIElement)iconstruct);
///     }
///     else
///     {
///         ProcessAnyIConstruct(iconstruct);
///     }
/// ```
///
/// * The need to process collections of type hierarchy instances of mixed concrete types, unknown
///   concrete types, or both is common.
///   * Under such circumstances, the actual concrete type of at least a subset of these instances
///     is unknowable at compile time (something that presents some degree of challenge in a
///     language like Rust).
/// * For this example, the specified (IConstruct) trait object reference is cast to another
///   interface implemented by its underlying concrete type and then dispatched to a handler for
///   that interface type.
///
/// * Whether casting is advisable, or required, depends on the work that needs to be performed.
///   * Casting to a concrete type improves performance, as functions will be invoked via static
///     rather than dynamic dispatch.
///     * Due to the overhead introduced by casting, more benefit will be afforded to more
///       extensive workflows.
///   * Invoking methods defined in downstream traits requires casting to at least the trait object
///     in which the required functionality is defined.
///     * For example, when a &mut dyn IConstruct is passed in whose underlying concrete type is
///       UIElement, casting to UIElement (or at least &mut dyn IUIElement) is required before new
///       functions contributed by IUIElement can be invoked.
///
/// * Note that because the generic type is constrained to IConstruct, only methods defined in
///   IConstruct are available before the object is cast to another type.
///   * This includes methods implemented for dyn IConstruct-- if it's not defined in IConstruct,
///     it can't be invoked without casting.
///   * Unfortunately, because a type so constrained is unsized, there's no way to cast to &mut dyn
///     IConstruct without casting it to a concrete type first.
///     * This makes invoking downcasts without knowing the concrete type impossible, but these
///       examples are intended to demonstrate how to proceed when the concrete type is not known
///       anyway.
///       * The "impossible" is made possible by discovering the concrete type at runtime.
///       * This unfortunately does not allow the caller to retain a references to the concrete
///         type, however.
///   * Because methods from traits implemented for &mut dyn IConstruct aren't available, this
///     function instead directly invokes the Transmutation struct responsible for downcasting to
///     IUIElement.
///
pub fn dispatch_iconstruct_by_implemented_interface<T>(iconstruct: &mut T)
    where T: IConstruct + ?Sized
{
    //
    // Implementation notes:
    //
    // * Should new types need to be supported, separate dispatch conditionals will need to be
    //   added to this function and corresponding handlers will need to be implemented IFF behavior
    //   must change.
    //   * Unlike in the previous example, this example uses bucketing-- we don't need a one-to-one
    //     relationship between supported types and handlers.
    // * Note that the order of the conditionals below is important.
    //   * If (UIElements) should be processed differently than (Constructs), the .implements call
    //     for IUIElement must come first, or the wrong handler will be invoked.
    //   * This is much the same as the order in which Exception catch statements must be arranged
    //     in C#.
    //

    if iconstruct.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>())
    {
        process_any_iui_element(IUIElementDowncaster::as_iui_element_mut(iconstruct).expect(
          "dispatch_iconstruct_by_implemented_interface<T>(): Unable to cast (IConstruct) to \
           IUIElement when concrete type is known to implement it!"));
    }
    // There's no need to check to see if iconstruct implements IConstruct-- *every* RDH type does.
    // There's also no need to cast, because the handler function will accept any object that
    //  implements IConstruct, coercing its type when necessary.
    // Finally, were we to cast to &mut dyn IConstruct by calling .as_iconstruct_mut() anyway, as
    //  doing so would constitute an upcast-- which always succeed--, there would be no need to add
    //  an .expect() clause. (The compiler wouldn't allow it anyway, since the return type isn't an
    //  Option.)
    else
    {
        process_any_iconstruct(iconstruct);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Private example functions
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// process_ui_element:
//   Example function to process a mutable UIElement as part of the
//   dispatch_iconstruct_by_concrete_type example.
//
// Parameters:
// * ui_element (&mut UIElement): The mutable reference to the UIElement to process.
//
// Notes:
// * This function also demonstrates how to constrain a parameter to accept *only* mutable
//   references to a specific concrete type for its argument.
//
fn process_ui_element(ui_element: &mut UIElement)
{
    println!("Processing a UIElement instance!");
    ui_element.on_click();
}

//
// process_construct:
//   Example function to process a mutable Construct as part of the
//   dispatch_iconstruct_by_concrete_type example.
//
// Parameters:
// * construct (&mut Construct): The reference to the Construct to process.
//
// Notes:
// * This function also demonstrates how to constrain a parameter to accept *only* mutable
//   references to a concrete type for its argument.
//
fn process_construct(construct: &mut Construct)
{
    println!("Processing a Construct instance!");
    construct.on_click();
}

//
// process_any_iui_element:
//   Example function to process a (UIElement) as part of the
//   dispatch_iconstruct_by_implemented_interface example.
//
// Generic parameters:
// * T (IUIElement + ?Sized): Represents all (UIElement) instances.
//
// Parameters:
// * ui_element (&mut T): The reference to the (UIElement) to process.
//
// Notes:
// * This function also demonstrates how to constrain a parameter to accept mutable references to
//   both trait objects and type instances whose types are *known* to implement IUIElement.
//   * In keeping with this example's intent, even when their underlying concrete type is mut
//     UIElement, this constraint specifically excludes &mut dyn IConstruct objects. Because their
//     explicit type does not implement IUIElement, they are not *known* to implement IUIElement.
//   * For this function to work with &mut dyn IConstruct objects with underlying mut UIElement
//     instances, either the constraint must be relaxed, or the object must be cast to a qualifying
//     type before being sent to this function, e.g.:
//
//       iconstruct.as_iui_element_mut().expect("What gives?")
//          -or-
//       iconstruct.as_concrete_mut::<UIElement>().expect("What gives?")
//
fn process_any_iui_element<T>(ui_element: &mut T)
    where T: IUIElement + ?Sized
{
    println!("Processing a UIElement instance OF ANY KIND!");
    ui_element.on_click();
}

//
// process_any_iconstruct:
//   Example function to process a (Construct) as part of the
//   dispatch_iconstruct_by_implemented_interface example.
//
// Generic parameters:
// * T (IConstruct + ?Sized): Represents all (Construct) instances.
//
// Parameters:
// * construct (&mut T): The reference to the (Construct) to process.
//
// Notes:
// * This function also demonstrates how to constrain a parameter to accept mutable references to
//   all framework trait objects and type instances, but to no outside types.
//
fn process_any_iconstruct<T>(construct: &mut T)
    where T: IConstruct + ?Sized
{
    println!("Processing a Construct instance OF ANY KIND!");
    construct.on_click();
}

//
// *** Minutiae ***
//

// Traits and types

use rdh::_hierarchy::construct::{ Construct, ConstructVirtuals, IConstruct };
use rdh::_infrastructure::thaumaturgy::{ TypeRegistry, Necromances };
use crate::_hierarchy::ui_element::{ IUIElement, UIElement, IUIElementDowncaster };
