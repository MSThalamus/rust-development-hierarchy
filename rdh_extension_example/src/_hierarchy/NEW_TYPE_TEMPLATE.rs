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
// TODO: Read this:
//
// NEW_TYPE_TEMPLATE: The template to follow when adding a new RDH type.
//                    Change "[ ] DONE" to "[X] DONE" to keep track of completed TODOs.
//
// [ ] DONE
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

// ************************************************************************************************
// TODO: Copy and then rename this file to the new type name, e.g. pickup_truck.rs.
//       Update _hierarchy.rs to include this new file.
//       Swap placeholders for new type and interface names, e.g.:
//
//         NEW_TYPE        =>  PICKUP_TRUCK
//         INEW_INTERFACE  =>  IPICKUP_TRUCK
//         NewType         =>  PickupTruck
//         INewInterface   =>  IPickupTruck
//         new_type        =>  pickup_truck
//         inew_interface  =>  ipickup_truck
//
//       Note: Rust enforces its capitalization and underscore conventions, even in file names.
//
// [ ] DONE
// ************************************************************************************************

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
// ************************************************************************************************
// TODO: If not directly encompassing Construct, change the below to reflect this new type's parent
//       type.
//        For example, change construct: Construct to utility_vehicle: UtilityVehicle.
//       Add new fields here, separated by commas.
//
// [ ] DONE
// ************************************************************************************************
    //
    // construct: Stores this (NewType's) Construct instance.
    //
    construct: Construct
}
impl NewType
{
// ************************************************************************************************
// TODO: Update the Example below to show the correct use path.
//
// [ ] DONE
// ************************************************************************************************
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
// ************************************************************************************************
// TODO: For abstract base types only:
//       * In the declaration of new() below, change "pub" to "pub(crate)".
//       * Be sure at least one field remains private.
//         * NOTE: Best practice is for all to remain private!
//       * Do not implement the Default trait.
//
//       NOTES:
//       * It's necessary for all types to remain instantiable within their own RDH extension
//         library, as each type must directly hold an instances of its parent type.
//       * Because subsequent RDH extension libraries would be otherwise unable to instantiate
//         them, types at the leaf level in each library may not be abstract.
//
// [ ] DONE
// ************************************************************************************************
    pub fn new() -> NewType
    {
        NewType
        {
// ************************************************************************************************
// TODO: If not directly encompassing Construct, change the below to reflect this new type's parent
//       type.
//        For example, change construct: Construct::new() to utility_vehicle: UtilityVehicle::new()
//       Set default values for fields here, separated by commas.
//
// [ ] DONE
// ************************************************************************************************
            construct: Construct::new()
        }
    }
}

// ************************************************************************************************
// TODO: Read this:
//
// * Each (Construct) must include bespoke implementations for the *Virtuals traits of *all*
//   ancestor types.
//   * For example, all RDH types must include a bespoke implementation of the ConstructVirtuals
//     trait (as shown below).
//
// * While all (NewTypes) must implement all traits listed in both INewInterface's definition and
//   generic implementation, most are automated in some way.
//   * INewInterface is automatically implemented for any type that implements all other
//     prerequisite traits.
//   * UpcastsToINewInterface is implemented (along with all other casting infrastructure) by
//     calling the provision_transmutation! macro (as shown below).
//   * While NewTypeInstances must be implemented within each (NewType), this can be done by
//     calling the define_instances! macro (as shown below).
// * This leaves NewTypeVirtuals as the only new trait that requires a bespoke implementation
//   within each (NewType).
//
// [ ] DONE
// ************************************************************************************************

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
// ************************************************************************************************
// TODO: Add the public interfaces for all intermediate types below, each followed by a plus.
//
// [ ] DONE
// ************************************************************************************************
                          IConstruct
{
// ************************************************************************************************
// TODO: Add *non-virtual* method declarations here.
//       Note: Because INewInterface will be used to abstractly represent (NewType) instances, it
//              must be instantiable as a trait object.
//             As such, INewInterface may contain neither generic methods nor associated functions
//              (i.e. static methods).
//
// [ ] DONE
// ************************************************************************************************
}
pub trait NewTypeVirtuals
{
// ************************************************************************************************
// TODO: Add new *virtual* method declarations here.
//       Note: NewTypeVirtuals is a subtrait of INewInterface.
//               Because INewInterface will be used to abstractly represent (NewType) instances, it
//                (and all its subtraits) must be instantiable as a trait object.
//               As such, NewTypeVirtuals may contain neither generic methods nor associated
//                functions (i.e. static methods).
//
// [ ] DONE
// ************************************************************************************************
}
impl<T> INewInterface for T where T: NewTypeVirtuals + NewTypeInstances +
// ************************************************************************************************
// TODO: Add the *Virtuals and *Instances traits for all intermediate types below, each followed
//       by a plus.
//
// [ ] DONE
// ************************************************************************************************
                                     ConstructVirtuals + ConstructInstances +
                                     ConcreteDivinator
{
// ************************************************************************************************
// TODO: Add *non-virtual* method implementations here.
//
// [ ] DONE
// ************************************************************************************************
}
impl NewTypeVirtuals for NewType
{
// ************************************************************************************************
// TODO: Add new *virtual* method implementations here.
//
// [ ] DONE
// ************************************************************************************************
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Inherited virtual function overrides
///////////////////////////////////////////////////////////////////////////////////////////////////

// ************************************************************************************************
// TODO: Add virtual method overrides for all intermediate types here.
//       NOTES:
//       * Because we have to maintain our own v-tables, overrides are required for ALL methods, to
//         at least invoke parent implementations.
//       * When helpful, it's perfectly acceptable to invoke the implementations from previous
//         ancestors instead-- punting should be both unchained and uncommon!
//
// [ ] DONE
// ************************************************************************************************

//
// IConstruct overrides.
//
impl ConstructVirtuals for NewType
{
// ************************************************************************************************
// TODO: Add virtual method overrides for Construct here.
//       NOTES:
//       * Because we have to maintain our own v-tables, overrides are required for ALL methods, to
//         at least invoke parent implementations.
//
// [ ] DONE
// ************************************************************************************************
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Macro Invocations ***

// ************************************************************************************************
// TODO: Rename the "RdhExtension" portion of DowncastsRdhExtensionTypesToINewInterface to reflect
//        the library to which the new type is being added.
//       Add the public interfaces for all intermediate types, each followed by a space,
//        immediately prior to IConstruct below, e.g.:
//         ... + [ IUtilityVehicle IVehicle IConstruct ]
//
// [ ] DONE
// ************************************************************************************************
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
// ************************************************************************************************
// TODO: Add define_instances macro invocations for all intermediate types.
//         Also update the Construct invocation when necessary.
//       Triple check field and method paths!
//       See the define_instances! header for further details.
//
// For example:
//
// define_instances!
// {
//     UtilityVehicleInstances for NewType : Fn[utility_vehicle, utility_vehicle_mut] ->
//       UtilityVehicle (utility_vehicle)
// }
// define_instances! { VehicleInstances for NewType : Fn[vehicle, vehicle_mut] ->
//                     Vehicle (utility_vehicle {.vehicle} {.vehicle_mut}) }
// define_instances! { ConstructInstances for NewType : Fn[construct, construct_mut] ->
//                     Construct (utility_vehicle {.vehicle.construct}
//                                                {.vehicle_mut.construct_mut}) }
//
// [ ] DONE
// ************************************************************************************************
define_instances! { ConstructInstances for NewType : Fn[construct, construct_mut] ->
                    Construct (construct) }

// *** Internal (i.e. public in crate) constants ***

//
// INEW_INTERFACE_TYPE_UUID: The unique identifier for the INewInterface type.
// NEW_TYPE_TYPE_UUID: The unique identifier for the NewType type.
//
// ************************************************************************************************
// TODO: Populate UUIDs below with random u128 values, e.g. 246318330869446075964898337533927694308
//        Failing to do so will result in intended compiler errors.
//       As these are simply UUIDs (AKA GUIDs) in decimal format, the following online tools may be
//       helpful:
//       * Generate random GUIDs: Online GUID / UUID Generator
//          https://guidgenerator.com/online-guid-generator.aspx
//       * Convert GUIDs to u128: GUID Converter
//          https://toolslick.com/conversion/data/guid
//
// [ ] DONE
// ************************************************************************************************
pub(crate) const INEW_INTERFACE_TYPE_UUID: u128 = -1;
pub(crate) const NEW_TYPE_TYPE_UUID: u128 = -1;

// *** Minutiae ***

// ************************************************************************************************
// TODO: Update and add use statements below as needed.
//       Don't forget to look at thaumaturgy.rs for errors related to missing use statements here.
//
// [ ] DONE
// ************************************************************************************************
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

// ************************************************************************************************
// TODO: Update the current library's lib module.
//        Read carefully! This part is more involved than the rest.
//        If you have not read all the provided documentation, you should do so before proceeding.
//
//       NOTE: Construct is explicitly excluded in many places below because it isn't possible to
//              downcast to IConstruct.
//
//       1) Create or update downcasting extension macro invocations:
//          a) When NewType directly encompasses a type (other than Construct) defined in a
//             previous library:
//             * Invoke the macro that will enable downcasts to that type when NewType is the
//               underlying concrete type, e.g.:
//                 extend_downcasting!
//                 {
//                     for IEncompassedType
//                     {
//                         add Fn[as_iencompassed_type, as_iencompassed_type_mut] to
//                          DowncastsRdhExtensionTypesToIEncompassedType:
//                           DowncastsLibraryTypesToIEncompassedType include [NewType]
//                     }
//                 }
//             * In the above, be sure to 1) swap out IEncompassedType, etc., for the target type,
//                and 2) update DowncastsRdhExtensionTypesToIEncompassedType to reflect the naming
//                convention established within the provision_transmutation! invocation above.
//             * Repeat this for any types back to, but not including, Construct.
//
//          b) The next TODO covers situations where the TristateCheckbox encompasses a type
//              defined within the current library that in turn encompasses a type (other than
//              Construct) defined in a previous library.
//
//       2) Update the init() function:
//          a) Register NewType and INewInterface within the TypeRegistry, using the library's UUID
//             as defined within the lib module, e.g.:
//               TypeRegistry::register_type::<NewType>(NEW_TYPE_TYPE_UUID, RDH_EXTENSION_UUID);
//               TypeRegistry::register_type::<dyn INewInterface>(INEW_INTERFACE_TYPE_UUID,
//                                                                RDH_EXTENSION_UUID);
//
//          b) Create and register downcast handlers for NewType, (updating
//              DowncastsRdhExtensionTypesToINewInterface to reflect the naming convention
//              established within the provision_transmutation! invocation above), e.g:
//                INewInterfaceDowncaster::register_handler(
//                  TypeRegistry::library_identifier_of::<NewType>(),
//                  Box::new(DowncastsRdhExtensionTypesToINewInterface{}));
//
//          c) When NewType encompasses, directly or indirectly, a type (other than Construct)
//             defined in a previous library, register a new downcast handler per the macro created
//             earilier (updating names to match macro invocation), e.g.:
//               IEncompassedTypeDowncaster::register_handler(
//                 TypeRegistry::library_identifier_of::<NewType>(),
//                 Box::new(DowncastsRdhExtensionTypesToIEncompassedType{}));
//             * Repeat this for any types back to, but not including, Construct.
//
// [ ] DONE
// ************************************************************************************************

// ************************************************************************************************
// TODO: Unless NewType directly encompasses Construct, update the provision_transmutation! call
//       within all ancestor types defined within the current library.
//       1) When NewType encompasses a type defined within the current library that in turn
//           directly encompasses Construct, within that type's module:
//          Add a list of types to include when looking for an (IConstruct's) concrete type,
//           populating it only with NewType, e.g.:
//            ...
//            DowncastsRdhExtensionTypesToIParentType:
//              DowncastsLibraryTypesToIParentType include [NewType]
//            ...
//       2) Otherwise, within that type's module, add NewType to the end of the existing bracketed
//          type list, separated by a space, e.g.:
//            ...
//            DowncastsRdhExtensionTypesToIAncestorType:
//              DowncastsLibraryTypesToIAncestorType include [ParentType NewType]
//            ...
//
// [ ] DONE
// ************************************************************************************************

// ************************************************************************************************
// TODO: Remove all these TODOs! :)
//
// [ ] DONE (<== meant to be funny)
// ************************************************************************************************
