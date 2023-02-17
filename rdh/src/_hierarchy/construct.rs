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
//! * (IConstruct) means an IConstruct trait object or any trait object encompassing IConstruct.
//! * (Construct) means the Construct struct or any concrete type that implements IConstruct.
//! Whereas:
//! * IConstruct (no parentheses) means the literal IConstruct trait or its trait objects.
//! * Construct (no parentheses) means the literal Construct struct or its instances.
//!

//!
//! construct.rs
//!
//! IConstruct (Construct)
//!   The public interface and struct that serve as the root of the RDH type hierarchy.
//!
//! THIS IS A WORK IN PROGRESS. Although this type will be retained, current functionality is
//! placeholder only and *will* change.
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// Construct struct
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Construct: The implementation of base (Construct) functionality.
///
pub struct Construct
{
    //
    // name: Stores this (Construct's) current name.
    //
    name: String
}
impl Construct
{
    ///
    /// new: Creates a new instance of the Construct struct.
    ///
    /// # Example
    ///
    /// ```
    /// use rdh::_hierarchy::construct::Construct;
    ///
    /// let construct = Construct::new();
    /// ```
    ///
    pub fn new() -> Construct
    {
        Construct
        {
            name: "my construct".to_string()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// IConstruct public interface trait and component / supplemental traits
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// IConstruct: The interface implemented by all (Constructs).
//              * Non-virtual functions are directly declared and implemented below.
//              * Virtual functions are declared within the ConstructVirtuals trait.
//                Their base implementation is below-- encompassing types override as needed.
//              * Casting functionality provided by additional component and supplemental traits.
///
pub trait IConstruct : ConstructVirtuals + UpcastsToIConstruct + ConcreteDivinator
{
    // *** Properties ***

    ///
    /// name: Gets and sets this (Construct's) name
    ///        via get_name and set_name.
    ///
    /// THIS PROPERTY IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdh::_hierarchy::construct::Construct;
    ///
    /// let mut construct_mut = Construct::new();
    /// construct_mut.set_name("Nifty new name!");
    /// println!("{}", construct_mut.get_name());
    /// ```
    ///
    fn get_name(&self) -> &str;
    fn set_name(&mut self, value: &str);
}
pub trait ConstructVirtuals
{
    ///
    /// on_click: Handles click events for this (Construct), according to its concrete type.
    ///
    /// THIS METHOD IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdh::_hierarchy::construct::Construct;
    ///
    /// let mut construct_mut = Construct::new();
    /// construct_mut.on_click();
    /// ```
    ///
    fn on_click(&mut self);
}
impl<T> IConstruct for T where T: ConstructVirtuals + ConstructInstances + ConcreteDivinator
{
    // *** Properties ***

    //
    // name: Gets and sets this (Construct's) name.
    //
    // THIS PROPERTY IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    //
    fn get_name(&self) -> &str
    {
        &(self.construct().name)
    }
    fn set_name(&mut self, value: &str)
    {
        self.construct_mut().name = value.to_string();
    }
}
impl ConstructVirtuals for Construct
{
    //
    // on_click: Handles click events for this (Construct), according to its concrete type.
    //
    // THIS METHOD IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    //
    fn on_click(&mut self)
    {
        println!("on_click handled by Construct implementation!");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Macro Invocations ***

// Implement upcasting to IConstruct.
provision_transmutation!
{
    for Construct: IConstruct
    {
        add Fn[as_iconstruct, as_iconstruct_mut] to UpcastsToIConstruct
    }
}

// Provide access to the current Construct instance.
define_instances! { ConstructInstances for Construct : Fn[construct, construct_mut] -> Construct }

// *** Internal (i.e. public in crate) constants ***

//
// ICONSTRUCT_TYPE_UUID: The unique identifier for the IConstruct type.
// CONSTRUCT_TYPE_UUID: The unique identifier for the Construct type.
//
pub(crate) const ICONSTRUCT_TYPE_UUID: u128 = 50657587353538260646247596146820191381;
pub(crate) const CONSTRUCT_TYPE_UUID: u128 = 276154362193470083307838638451583829221;


// *** Macro Definitions ***

///
/// define_instances: Declares and/or implements a trait used to grant generic trait
///                   implementations direct access to their corresponding concrete (Construct)
///                   instances.
///
/// Parameters:
/// * $trait_name (ident): The name to assign to the trait.
/// * $current_type (ty): The concrete type for which the trait is being implemented.
/// * $get (ident): The name of the method used to obtain an immutable reference to the concrete
///   (Construct) instance.
/// * $get_mut (ident): The name of the method used to obtain a mutable reference to the concrete
///   (Construct) instance.
/// * $expressed_type (ident): The concrete type expressed by these methods.
/// * $parent_field (ident): The field within $current_type at which the parent type and through
///   which all ancestor concrete instances can be obtained.
/// * $method (ident): The .-delimited list of remaining method calls within the chain to the
///   immutable instance, if any.
/// * $method_mut (ident): The .-delimited list of remaining method calls within the chain to the
///   mutable instance, if any.
///
/// # Example invocations:
///
///  To declare and implement the trait for a type in the same module in which it's defined:
///
/// ```
///      define_instances!
///      {
///          ConstructInstances for Construct : Fn[construct, construct_mut] -> Construct
///      }
/// ```
///
///  To implement the parent type's trait for a type that encompasses that existing type and stores
///   it within its "self.construct" field:
///
/// ```
///      define_instances!
///      {
///          ConstructInstances for Vehicle : Fn[construct, construct_mut] -> Construct (construct)
///      }
/// ```
///
///  To implement a grandparent type's trait for a type that encompasses that existing type and
///   accesses it through "self.vehicle.construct[_mut]()":
///
/// ```
///      define_instances!
///      {
///          ConstructInstances for Car : Fn[construct, construct_mut] ->
///            Construct (vehicle {.construct} {.construct_mut})
///      }
/// ```
///
///  To implement a further ancestor type's trait for a type that encompasses that existing type
///   and accesses it through "self.car.vehicle.construct[_mut]()" (or deeper):
///
/// ```
///      define_instances!
///      {
///          ConstructInstances for Sedan : Fn[construct, construct_mut] ->
///            Construct (Car {.vehicle.construct} {.vehicle_mut.construct_mut})
///      }
/// ```
///
/// Notes:
/// * The recommended format for $trait_name is <concrete type>Instances, e.g. VehicleInstances.
/// * To evade Rust compiler warnings, the recommended format for $get and $get_mut are <snake case
///    of concrete type> and <snake case of concrete type>_mut.
///
#[macro_export]
macro_rules! define_instances
{
    // Within the module in which the type is defined...
    {
        $trait_name: ident for $current_type: ty : Fn[$get: ident, $get_mut: ident] ->
          $expressed_type: ident
    } =>
    {
        ///
        /// $trait_name: The trait through which applicable public interface implementations can
        ///               gain access to their concrete $current_type instance.
        ///
        pub trait $trait_name
        {
            //
            // Implementation notes:
            // * This trait *must* be locked down to allow access only to developers extending,
            //   rather than consuming, the type hierarchy, but Rust has no way to model that
            //   across libraries.
            //   * This is a serious concern, but there's no known way to grant access within
            //     extending libraries that does not also grant access within consuming libraries
            //     and binaries.
            //   * At this time, the best (though clearly inadequate) known option at our disposal
            //     is to say to consuming developers:
            //     "If you call the methods defined for this trait and use the returned instances,
            //      your program will behave incorrectly."
            // * For further information, see Issue #2 in "Knowledge Base.md"
            //

            ///
            /// $get: Returns an immutable reference to the $concrete_type instance.
            ///
            /// # Examples
            ///
            /// ```
            /// println!(self.$get().name);  // Assuming name is a defined String field....
            /// ```
            ///
            fn $get(&self) -> &$expressed_type;

            ///
            /// $get_mut: Returns a mutable reference to the $concrete_type instance.
            ///
            /// # Examples
            ///
            /// ```
            /// self.$get_mut().name = value.to_string();  // Assuming name is a defined String
            ///                                            // field....
            /// ```
            ///
            fn $get_mut(&mut self) -> &mut $expressed_type;
        }
        impl $trait_name for $current_type
        {
            fn $get(&self) -> &$expressed_type { self }
            fn $get_mut(&mut self) -> &mut $expressed_type { self }
        }
    };

    // When type is defined within the current module's parent module (and therefore the trait has
    //  already been declared)...
    {
        $trait_name: ident for $current_type: ty : Fn[$get: ident, $get_mut: ident] ->
          $expressed_type: ident ($parent_field: ident)
    } =>
    {
        impl $trait_name for $current_type
        {
            fn $get(&self) -> &$expressed_type { &(self.$parent_field) }
            fn $get_mut(&mut self) -> &mut $expressed_type { &mut(self.$parent_field) }
        }
    };

    // When type is defined within the current module's grandparent or earlier ancestor module (and
    //  therefore the trait has already been declared)...
    {
        $trait_name: ident for $current_type: ty : Fn[$get: ident, $get_mut: ident] ->
          $expressed_type: ident ($parent_field: ident
            { $(.$method: ident)* } { $(.$method_mut: ident)* })
    } =>
    {
        impl $trait_name for $current_type
        {
            fn $get(&self) -> &$expressed_type { self.$parent_field$(.$method())* }
            fn $get_mut(&mut self) -> &mut $expressed_type { self.$parent_field$(.$method_mut())* }
        }
    }
}


// *** Minutiae ***

// Traits and types
use crate::_infrastructure::thaumaturgy::ConcreteDivinator;

// Macros and dependencies
use crate::define_instances;
use crate::impl_concrete_divinator;
use crate::provision_transmutation;
use crate::provision_upcasting;
use crate::_infrastructure::thaumaturgy::{ Divinator, LibraryIdentifier, TypeIdentifier,
                                           TypeRegistry };
