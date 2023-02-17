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

// ************************************************************************************************
// BARE_NEW_TYPE_TEMPLATE: The template to follow when adding a new RDH type once you know what
//                          you're doing. :)
//                         Copy and rename this file, swap out placeholders, remember to update the
//                          lib module, delete this header, and go!
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
//! new_type.rs
//!
//! INewInterface (NewType)
//!   The NewType public interface and type.
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// NewType struct
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// NewType: The implementation of (NewType) functionality.
///
pub struct NewType
{
    //
    // construct: Stores this (NewType's) Construct instance.
    //
    construct: Construct
}
impl NewType
{
    ///
    /// new: Creates a new instance of the NewType struct.
    ///
    /// # Example
    ///
    /// ```
    /// use rdh_extension_example::_hierarchy::new_type::NewType;
    ///
    /// let new_type = NewType::new();
    /// ```
    ///
    pub fn new() -> NewType
    {
        NewType
        {
            construct: Construct::new()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// INewInterface public interface trait and component / supplemental traits
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// INewInterface: The interface implemented by all (NewTypes).
//                 * Non-virtual functions are directly declared and implemented below.
//                 * Virtual functions are declared within the NewTypeVirtuals trait.
//                   Their base implementation is below-- encompassing types override as needed.
//                 * Casting functionality provided by additional component and supplemental
//                   traits.
///
pub trait INewInterface : NewTypeVirtuals + UpcastsToINewInterface +
                          IConstruct
{
}
pub trait NewTypeVirtuals
{
}
impl<T> INewInterface for T where T: NewTypeVirtuals + NewTypeInstances +
                                     ConstructVirtuals + ConstructInstances +
                                     ConcreteDivinator
{
}
impl NewTypeVirtuals for NewType
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Inherited virtual function overrides
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// IConstruct overrides.
//
impl ConstructVirtuals for NewType
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Macro Invocations ***

// Implement upcasting and downcasting to INewInterface and between interfaces NewType implements.
provision_transmutation!
{
    for NewType: INewInterface + [IConstruct]
    {
        add Fn[as_inew_interface, as_inew_interface_mut] to
        (
            UpcastsToINewInterface,
            DowncastsToINewInterface,
            INewInterfaceDowncaster,
            DowncastsRdhExtensionTypesToINewInterface: DowncastsLibraryTypesToINewInterface
        )
    }
}

// Provide access to the current NewType instance and the encompassed (Construct) instances.
define_instances! { NewTypeInstances for NewType : Fn[new_type, new_type_mut] -> NewType }
define_instances! { ConstructInstances for NewType : Fn[construct, construct_mut] ->
                    Construct (construct) }

// *** Internal (i.e. public in crate) constants ***

//
// INEW_INTERFACE_TYPE_UUID: The unique identifier for the INewInterface type.
// NEW_TYPE_TYPE_UUID: The unique identifier for the NewType type.
//
pub(crate) const INEW_INTERFACE_TYPE_UUID: u128 = -1;
pub(crate) const NEW_TYPE_TYPE_UUID: u128 = -1;

// *** Minutiae ***

// Traits and types
use rdh::_hierarchy::construct::{ Construct, ConstructInstances, ConstructVirtuals, IConstruct };
use rdh::_infrastructure::thaumaturgy::ConcreteDivinator;

// Macros and dependencies
use rdh::define_instances;
use rdh::extend_downcasting;
use rdh::impl_concrete_divinator;
use rdh::provision_transmutation;
use rdh::provision_upcasting;
use std::collections::HashMap;
use std::sync::{ Once, RwLock };
use rdh::_infrastructure::thaumaturgy::{ Divinator, LibraryIdentifier, Necromances, TypeIdentifier,
                                         TypeRegistry };
