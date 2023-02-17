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
//! thaumaturgy.rs
//!
//! Thaumaturgy: The power to work magic.
//!
//!  Contains:
//!  * Divination: Traits, structs, and impls used to distinguish between different RDH types and
//!                to gather information about the concrete (Constructs) underlying various
//!                (IConstructs).
//!  * Necromancy: Traits, structs, and impls through which casts from (IConstruct) objects to
//!                (Construct) instances may be requested.
//!  * Transmutation: Macros that create traits, structs, and impls to enable casting between
//!                   (IConstructs) implemented by the same (Construct), even when the type of that
//!                   (Construct) is unknown.
//! 
//! THIS IS A WORK IN PROGRESS.
//!   Although broad strokes will be retained, at this time, everything is subject to change
//!   without notice.
//!

//
// Implementation notes:
//
// * Singletons: Static data and thread-safety:
//   * A number of data pools are maintained to support casting: 
//     * One to track general type information across all RDH types, and
//     * One per RDH type that's used to determine the library to which downcast requests will be
//       sent given the underlying concrete type.
//   * As these data pools must be accessible from anywhere in the code base, they are modeled as
//     public singleton structs.
//     * This inherently requires maintaining and updating static data.
//   * To ensure memory safety, Rust requires thread safety for all static mutable data access
//     events.
//     * In the interests of performance, we use std::sync::rwlock rather than a simple mutex.
//       * This allows us to lock down access only when a data pool must be updated (i.e. only
//         during initialization), and to otherwise support any number of simulataneous read
//         events.
//     * Unfortunately, we're presented with only two options out of the box for handling
//       contested access requests: wait until the end of time, or don't wait *at all*.
//       * Waiting forever is obviously not ideal, but expecting to never have to wait is
//         obviously unrealistic.
//         * (I mean, if we could expect to not wait at all, we wouldn't have to worry about
//           thread safety... right?)
//     * This puts the burden on us to implement bespoke code to handle retries, surface
//       reasonable errors to callers, and so on.
//     * While we already have a bespoke reader/writer lock implementation that handles all of
//       this, porting it to Rust from its current C++ and D versions would be a non-trivial
//       undertaking.
//     * Until this porting work is done, our implementation is kept purposely simple:
//       * Threads block indefinitely while waiting for contested locks.
//       * The application crashes if we're ever unable to obtain access to a singleton instance
//         for any reason.
//

///////////////////////////////////////////////////////////////////////////////////////////////////
// Divination: The power to know the unknowable.
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// *** Public traits ***
//

///
/// Divinator: The interface through which an RDH type's identifier and the identifier of the
///            library in which it was defined can be retrieved.
///
pub trait Divinator
{
    ///
    /// type_identifier: Retrieves the type identifier for the current Self type.
    ///
    fn type_identifier(&self) -> TypeIdentifier;

    ///
    /// library_identifier: Retrieves the identifier for the library in which the current Self type
    ///                     was defined.
    ///
    fn library_identifier(&self) -> LibraryIdentifier;
}
impl<T> Divinator for T where T: IConstruct + ?Sized
{
    fn type_identifier(&self) -> TypeIdentifier
    {
        TypeRegistry::type_identifier_of::<T>()
    }
    fn library_identifier(&self) -> LibraryIdentifier
    {
        TypeRegistry::library_identifier_of::<T>()
    }
}

///
/// ConcreteDivinator: The trait that exposes information about (underlying) concrete types and the
///                    interfaces they implement.
///
pub trait ConcreteDivinator
{
    ///
    /// concrete_type_identifier: Retrieves the identifier for the current instance's concrete
    //                            type.
    ///
    fn concrete_type_identifier(&self) -> TypeIdentifier;

    ///
    /// concrete_library_identifier: Retrieves the identifier for the library in which the current
    ///                              instance's concrete type is defined.
    ///
    fn concrete_library_identifier(&self) -> LibraryIdentifier;

    ///
    /// implements: Determines whether this object's concrete type implements the indicated public
    ///             interface.
    ///
    // Parameters:
    /// * interface_type (TypeIdentifier): The type identifier of the interface type to look for.
    ///
    /// Expresses:
    /// * If this object's concrete type implements the indicated public interface, expresses true.
    /// * Otherwise, expresses false.
    ///
    fn implements(&self, interface_type: TypeIdentifier) -> bool;

    ///
    /// is: Determines whether this object's concrete type *literally* is the indicated type.
    ///
    /// Parameters:
    /// * test_type (TypeIdentifier): The type identifier of the type being tested.
    ///
    /// Expresses:
    /// * If this object's concrete type *literally* is the indicated type, expresses true.
    /// * Otherwise, expresses false.
    ///
    /// IMPORTANT DISTINCTIONS FROM INHERITANCE-BASED LANGUAGES:
    /// * Although in languages such as C#, "is" will also return true if test_type is an ancestor
    ///   of this object's type or one of its implemented interfaces, this function will return
    ///   *false*.
    ///   * The *only* time "is" will return true is when the specified TypeIdentifier is the
    ///     *exact* type identifier for this object's concrete type.
    /// * To determine whether a type implements a given interface, call implements() rather than
    ///   is().
    /// * To determine whether a type is "derived" from another type (or, really, *encompasses*
    ///   it), call implements(), specifying the type's corresponding public interface, e.g.:
    ///
    /// ```
    ///    if car.implements(TypeRegistry::type_identifier_of::<IVehicle>())
    ///    {
    ///        println!("A Car IS a Vehicle!");
    ///    }
    /// ```
    ///
    /// Discussion points:
    /// * While returning true may be correct for other "is" queries on a conceptual level, Rust's
    ///   use of composition over inheritance makes doing so problematic, especially in casting
    ///   scenarios.
    ///   * In C# "is" returning true for the same Car before and after an upcast to Vehicle is
    ///     correct on both conceptual and literal levels-- they're the same object at the same
    ///     memory address.
    ///   * In Rust, however, "is" returning true after such an upcast would *only* be correct
    ///     *conceptually*-- Car and Vehicle are different objects at different memory addresses!
    ///
    fn is(&self, test_type: TypeIdentifier) -> bool;
}

//
// *** Public structs ***
//

///
/// TypeRegistry: The singleton casting support struct used to track information about RDH types.
///
pub struct TypeRegistry
{
    // NOTE: Code outside this module remains unable to instantiate this struct only while at least
    //       one field is kept private and no public constructor or Default implementation is
    //       added.

    // The table of type identifiers, indexed by their names.
    type_map: HashMap<String, TypeIdentifier>
}
impl TypeRegistry
{
    // *** Public functions ***

    ///
    /// register_type: Registers the indicated RDH interface or type with the specified type UUID
    ///                and library UUID.
    ///
    /// Generic parameters:
    /// * T (IConstruct + ?Sized): The RDH type to register.
    ///
    /// Parameters:
    /// * type_uuid (u128): The unique identifier to assign to the type being registered.
    /// * library_uuid (u128): The unique identifier of the library in which the type is defined.
    ///
    /// Panics:
    /// * If unable to acquire the lock that guards the TypeRegistry singleton instance.
    /// * If unable to acquire the mutable TypeRegistry singleton instance.
    /// * If the indicated type has already been registred.
    /// * If the specified type UUID has already been registered to a different type.
    ///
    /// Notes:
    /// * This function will block until exclusive singleton instance access can be obtained,
    ///   potentially forever.
    ///   * See this module's Implementation Notes.
    ///
    pub fn register_type<T>(type_uuid: u128,
                            library_uuid: u128)
        where T: IConstruct + ?Sized
    {
        // Implementation note: It would be better to allow access only to developers extending,
        //                      rather than consuming, the type hierarchy, but Rust has no way to
        //                      model that across libraries.

        let type_name = TypeRegistry::get_type_name::<T>();
        let mut instance = unsafe { TypeRegistry::rwlock() }.write().expect(
          "TypeRegistry::register_type<T>(): Unable to acquire mutable TypeRegistry singleton \
           instance!");

        // Make sure the type's name hasn't already been registered....
        if instance.type_map.contains_key(&type_name) == false
        {
            // Also make sure the type's UUID hasn't already been registered....
            if instance.type_map.values().any(|registered_type|
                                              registered_type.type_uuid == type_uuid)
            {
                panic!("TypeRegistry::register_type<T>(): Specified UUID ({}) already registered \
                        to a different type!", type_uuid);
            }

            instance.type_map.insert(type_name, TypeIdentifier
                                                {
                                                    type_uuid,
                                                    library_identifier: LibraryIdentifier
                                                                        {
                                                                            uuid: library_uuid
                                                                        }
                                                });
        }
        else
        {
            panic!("TypeRegistry::register_type<T>(): Indicated type ({}) has already been \
                    registered!", type_name);
        }
    }

    ///
    /// type_identifier_of: Retrieves the type identifier assigned to the indicated RDH type.
    ///
    /// Generic parameters:
    /// * T (IConstruct + ?Sized): The RDH type whose type identifier to retrieve.
    ///
    /// Expresses: The type's identifier.
    ///
    /// Panics:
    /// * If unable to acquire the lock that guards the TypeRegistry singleton instance.
    /// * If unable to acquire the immutable TypeRegistry singleton instance.
    /// * If the indicated type has not been registered.
    ///
    pub fn type_identifier_of<T>() -> TypeIdentifier
        where T: IConstruct + ?Sized
    {
        let type_name = TypeRegistry::get_type_name::<T>();
        let instance = unsafe { TypeRegistry::rwlock() }.read().expect(
          "TypeRegistry::type_identifier_of<T>(): Unable to acquire immutable TypeRegistry \
           singleton instance!");
        if instance.type_map.contains_key(&type_name) == true
        {
            instance.type_map[&type_name]
        }
        else
        {
            panic!("TypeRegistry::type_identifier_of<T>(): Unable to obtain TypeIdentifier for \
                    {}!", type_name);
        }
    }

    ///
    /// library_identifier_of: Retrieves the identifier assigned to the library in which the
    ///                        indicated RDH type is defined.
    ///
    /// Generic parameters:
    /// * T (IConstruct + ?Sized): The RDH type whose library identifier to retrieve.
    ///
    /// Expresses: The library's identifier.
    ///
    /// Panics:
    /// * If unable to acquire the lock that guards the TypeRegistry singleton instance.
    /// * If unable to acquire the immutable TypeRegistry singleton instance.
    /// * If the indicated type has not been registered.
    ///
    pub fn library_identifier_of<T>() -> LibraryIdentifier
        where T: IConstruct + ?Sized
    {
        TypeRegistry::type_identifier_of::<T>().library_identifier
    }

    // *** Private functions ***

    //
    // get_type_name: Gets the type name for the indicated type.
    //
    // Generic types:
    // * T (IConstruct + ?Sized): The RDH type whose type name to retrieve.
    //
    // Expresses:
    // * The name of the indicated type.
    //
    // Panics:
    // * If unable to acquire the type name.
    //
    fn get_type_name<T>() -> String
        where T: IConstruct + ?Sized
    {
        //
        // Implementation notes:
        //
        // * To understand why this method is needed and the ways in which it's currently limited,
        //   see Issue #8 and Issue #9 in "Knowledge Base.md."
        //
        // * Because all casting operations call this method at least once, it must be as
        //   performant as possible.
        //   * While str offers search functionality and there is a regex crate, manually parsing
        //     the string one time will likely be more performant than employing multiple str
        //     operations and doesn't require the use of an additional crate, something that
        //     conflicts with the initial design goals of RDH.
        //

        // ASSUMPTION: At this time, std::any::type_name<T>() is expected to yield strings with
        //             interspersed legal segments only on the *left* side of the intended payload.
        //             * To optimize performance, we assume this pattern will hold.
        //             * If a legal string segment were found to the right of the intended payload,
        //               the unwanted legal string would be extracted instead.
        //             * If such a string were encounterd, we would need to issue an update pronto.

        // Get the type's name per Rust.
        let rust_type_name = any::type_name::<T>();
        let rust_name_length = rust_type_name.chars().count();
        if rust_name_length == 0
        {
            panic!("TypeRegistry::get_type_name<T>(): Unable to acquire the type name for an \
                   (IConstruct)!");
        }

        // Find the index of the first invalid symbol after the last valid symbol.
        let mut index = rust_name_length - 1;
        let mut slice_end = 0;
        for char in rust_type_name.chars().rev()
        {
            if (char >= 'A' && char <= 'Z') || (char >= 'a' && char <= 'z') ||
               (char >= '0' && char <= '9') || char == '_'
            {
                if slice_end == 0
                {
                    slice_end = rust_name_length;
                }
                break;
            }

            slice_end = index;

            if index == 0
            {
                panic!("TypeRegistry::get_type_name<T>(): No legal characters found within \
                        the type name for the specified (IConstruct) as reported by Rust! \
                        (\"{}\")", rust_type_name);
            }
            index -= 1;
        }

        // Find the index of the first valid symbol after the last invalid symbol prior to those
        // found above.
        index = 0;
        let mut slice_start = 0;
        for char in rust_type_name.chars()
        {
            if (char < 'A' || char > 'Z') && (char < 'a' || char > 'z') &&
               (char < '0' || char > '9') && char != '_' && char != ':'
            {
                slice_start = index + 1;
            }
            index += 1;

            if index == slice_end
            {
                break;
            }
        }

        // Extract the type's name.
        rust_type_name[slice_start .. slice_end].to_string()
    }

    //
    // rwlock: Obtains a reference to the reader/writer lock that guards access to the singleton
    //         TypeRegistry instance (creating both when necessary).
    //
    // Expresses: A reference to the lock that guards the singleton instance.
    //
    // Panics:
    // * If unable to acquire the lock that guards the TypeRegistry singleton instance.
    //
    unsafe fn rwlock<'a>() -> &'a RwLock<TypeRegistry>
    {
        // Implementation notes: Structs may not contain static fields, so we have to declare the
        // singleton instance and the rwlock that guards it here (which has its advantages).

        // The RwLock, embedded within an Option, that guards the singleton instance of the
        // TypeRegistry struct.
        static mut RWLOCK: Option<RwLock<TypeRegistry>> = None;

        // Create the rwlock and singleton instance the first time this function is called (while
        // all other calling threads are blocked).
        static ONCE: Once = Once::new();
        ONCE.call_once(||
                       {
                           RWLOCK = Some(RwLock::new(TypeRegistry { type_map: HashMap::new() }));
                       });

        RWLOCK.as_ref().expect("TypeRegistry::rwlock(): Unable to acquire the lock that guards \
                                the TypeRegistry singleton instance!")
    }
}

///
/// TypeIdentifier: Represents an RDH type's identifier.
///
/// Notes:
/// * TypeIdentifier instances should be treated as opaque blobs, requested only when needed and passed on blindly.
//  * That UUIDs can accomodate up to 18.5 quintillion values is not meant to recommend this as a
//    reasonable number of hirarchy types for any application! :)
//    * Instead, using u128s for identifiers is intended to ease parallel development and
//      integration work across large teams with multiple dependencies.
//    * Just as with UUIDs in other languages, randomly selected u128 values are exceedingly
//      unlikely to conflict.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeIdentifier
{
    // The unique identifier for the RDH type.
    type_uuid: u128,

    // The unique identifier for the library in which the RDH type is defined.
    library_identifier: LibraryIdentifier
}

///
/// LibraryIdentifier: The unique identifier for an RDH library.
///
/// Notes:
/// * LibraryIdentifier instances should be treated as opaque blobs, requested only when needed and passed on blindly.
//  * That UUIDs can accomodate up to 18.5 quintillion values is not meant to recommend this as a
//    reasonable number of RDH libraries for any application! :)
//    * Instead, using u128s for identifiers is intended to ease parallel development and
//      integration work across large teams with multiple dependencies.
//    * Just as with UUIDs in other languages, randomly selected u128 values are exceedingly
//      unlikely to conflict.
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LibraryIdentifier
{
    uuid: u128
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Necromancy: The power to raise the dead.
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// *** Public traits ***
//

///
/// Necromances: The trait through which casts from (IConstruct) objects to (Construct) instances
///              may be requested.
///
/// Notes:
/// * Because Rust currently offers no way to specify a constraint that excludes concrete types,
///   the Necromances trait is implemented for concrete types even though such functionality has no
///   real meaning for them.
///   * While no harm is done if a concrete type is cast to itself in this way, doing so is a waste
///     of CPU cycles.
///   * In addition, that such a call can be made may be confusing.
///     * It's important to keep in mind that Necromancer will *not* cast between concrete types,
///       even though it may appear as though it could.
///     * Only calls such as vehicle.as_concrete::<Vehicle>() will succeed.
///     * Other calls, such as vehicle.as_concrete::<Construct>(), will fail, even though Vehicle
///       encompasses Construct and the corresponding upcast for their interfaces will work through
///       Transmutation.
///     * Please see the "Casting scenarios" section within "RDH Inheritance Model.md."
///
pub trait Necromances : IConstruct
{
    ///
    /// as_concrete: Attempts to cast this immutable (IConstruct) object into an immutable
    ///              (Construct) instance.
    ///
    /// Generic parameters:
    /// * T (IConstruct): The immutable (Construct) type to which this immutable (IConstruct)
    ///                   object will be cast.
    ///
    /// Expresses:
    /// * If successful, an immutable reference to the underlying concrete (Construct) type
    ///   instance within an Option<&T>.
    /// * Otherwise expresses None.
    ///
    fn as_concrete<T>(&self) -> Option<&T> where T: IConstruct;

    ///
    /// as_concrete_mut: Attempts to cast this mutable (IConstruct) object into a mutable
    ///                  (Construct) instance.
    ///
    /// Generic parameters:
    /// * T (IConstruct): The mutable (Construct) type to which this mutable (IConstruct) object
    ///                   will be cast.
    ///
    /// Expresses:
    /// * If successful, a mutable reference to the underlying concrete (Construct) type instance
    ///   within an Option<&mut T>.
    /// * Otherwise expresses None.
    ///
    fn as_concrete_mut<T>(&mut self) -> Option<&mut T> where T: IConstruct;
}
impl<U> Necromances for U where U: IConstruct + ?Sized
{
    fn as_concrete<T>(&self) -> Option<&T> where T: IConstruct
    {
        Necromancer::unearth_concrete::<_, T>(self)
    }
    
    fn as_concrete_mut<T>(&mut self) -> Option<&mut T> where T: IConstruct
    {
        Necromancer::unearth_concrete_mut::<_, T>(self)
    }
}

//
// *** Public structs ***
//

///
/// Necromancer: The struct through which casts from (IConstruct) objects to (Construct) instances
///              may be requested.
///
///
/// Notes:
/// * Because Rust currently offers no way to specify a constraint that excludes concrete types,
///   Necromancer functions will accept concrete types as function arguments, even though such
///   functionality has no real meaning for them.
///   * While no harm is done if a concrete type is unearthed from itself, doing so is a waste of
///     CPU cycles.
///   * In addition, that such a call can be made may be confusing.
///     * It's important to keep in mind that Necromancer will *not* cast between concrete types,
///       even though it may appear as though it could.
///     * Only calls such as vehicle.as_concrete::<Vehicle>() will succeed.
///     * Other calls, such as vehicle.as_concrete::<Construct>(), will fail, even though Vehicle
///       encompasses Construct and the corresponding upcast for their interfaces will work through
///       Transmutation.
///     * Please see the "Casting scenarios" section within "RDH Inheritance Model.md."
///
pub struct Necromancer {}
impl Necromancer
{
    //
    // Implementation notes:
    // * While Necromancer and its functions are all public and involve unsafe code, neither the
    //   struct nor the functions are labelled unsafe because type checking should provide
    //   sufficient guarantee that the "unsafe" code is, in fact, safe to run.
    //

    ///
    /// unearth_concrete: Attempts to cast the specified immutable (IConstruct) object into an
    ///                   immutable (Construct) instance.
    ///
    /// Generic parameters:
    /// * T (IConstruct + ?Sized): The type of immutable (IConstruct) object to cast to an
    ///                            immutable (Construct) type.
    /// * U (IConstruct): The immutable (Construct) type to which the immutable (IConstruct) object
    ///                   will be cast.
    ///
    /// Expresses:
    /// * If successful, an immutable reference to the underlying concrete (Construct) type
    ///   instance within an Option<&T>.
    /// * Otherwise expresses None.
    ///
    pub fn unearth_concrete<T, U>(iconstruct: &T) -> Option<&U>
        where T: IConstruct + ?Sized,
              U: IConstruct
    {
        if iconstruct.is(TypeRegistry::type_identifier_of::<U>())
        {
            Some(unsafe { &*(iconstruct as *const T as *const U) })
        }
        else
        {
            None
        }
    }

    ///
    /// unearth_concrete_mut: Attempts to cast the specified mutable (IConstruct) object into a
    ///                       mutable (Construct) instance.
    ///
    /// Generic parameters:
    /// * T (IConstruct + ?Sized): The type of mutable (IConstruct) object to cast to a mutable
    ///                            (Construct) type.
    /// * U (IConstruct): The mutable (Construct) type to which the mutable (IConstruct) object
    ///                   will be cast.
    ///
    /// Expresses:
    /// * If successful, a mutable reference to the underlying concrete (Construct) type instance
    ///   within an Option<&mut T>.
    /// * Otherwise expresses None.
    ///
    pub fn unearth_concrete_mut<T, U>(iconstruct: &mut T) -> Option<&mut U>
        where T: IConstruct + ?Sized,
              U: IConstruct
    {
        if iconstruct.is(TypeRegistry::type_identifier_of::<U>())
        {
            Some(unsafe { &mut *(iconstruct as *mut T as *mut U) })
        }
        else
        {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Transmutation: The power to transform.
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// *** Public macros ***
//

///
/// provision_transmutation: Provisions the traits, structs, and impls needed to support upcasting
///                          and downcasting between interfaces, even when the underlying concrete
///                          type is unknown.
///
/// Parameters:
///
/// * For upcasting-only or both upcasting and downcasting:
///
///   * $concrete_type (ident): The concrete type for which transmutation is being provisioned.
///   * $target_interface (ident): The public interface to which transmuation methods will cast the
///                                (IConstructs) sent to them.
///   * $upcast_trait (ident): The name to assign to the trait that enables upcasting to
///                            $target_interface for types that encompass $concrete_type.
///   * $cast (ident): The name of the method that casts immutable (IConstruct) objects to
///                    immutable $target_interface objects, within all provisioned traits for both
///                    upcasting and downcasting.
///   * $cast_mut (ident): The name of the method that casts mutable (IConstruct) objects to
///                        mutable $target_interface objects, within all provisioned traits for
///                        both upcasting and downcasting.
///
/// * For downcasting only:
///
///   * $upstream_interface (ident): The space-delimited list of upstream interfaces, all of which
///                                  will be made able to downcast to $target_interface (one or
///                                  more).
///   * $downcast_trait (ident): The name to assign to the trait that enables downcasting to
///                              $target_interface for types encompassed by $concrete_type.
///   * $global_downcasting_struct (ident): The name to assign to the struct that coordinates
///                                         downcasting to $target_interface across all RDH
///                                         libraries.
///   * $library_downcasting_trait (ident): The name to assign to the trait implemented by structs
///                                         that handle downcasting to $target_interface for
///                                         individual RDH libraries.
///   * $library_downcasting_struct (ident): The name to assign to the struct that handles
///                                          downcasting to $target_interface for the current RDH
///                                          library.
///   * $library_concrete_type (ident): The space-delimited list of downstream concrete types
///                                     within the current RDH library in addition to
///                                     $concrete_type whose (IConstruct) objects may be downcast
///                                     to $target_interface, if any.
///
/// Runtime panics:
/// * If a $library_downcasting_trait handler for the specified library has already been registered
///   within $global_downcasting_struct.
/// * If unable to acquire the mutable $global_downcasting_struct singleton instance.
/// * If unable to acquire the immutable $global_downcasting_struct singleton instance.
/// * If unable to acquire the lock that guards the $global_downcasting_struct singleton instance.
///
/// # Example invocations:
///
///  To provision only upcasting support (likely only appropriate for Construct, the hierarchy
///  root):
///
/// ```
///      provision_transmutation!
///      {
///          for Construct: IConstruct
///          {
///              add Fn[as_iconstruct, as_iconstruct_mut] to UpcastsToIConstruct
///          }
///      }
/// ```
///
///  To provision both upcasting and downcasting support:
///
/// ```
///      provision_transmutation!
///      {
///          for Car: ICar + [IVehicle IConstruct]
///          {
///              add Fn[as_icar, as_icar_mut] to
///              (
///                  UpcastsToICar,
///                  DowncastsToICar,
///                  ICarDowncaster,
///                  DowncastsRdhExtensionTypesToICar: DowncastsLibraryTypesToICar include [Sedan]
///              )
///          }
///      }
/// ```
///
/// Notes:
///
/// * For downcasting support:
///
///   * While invoking this macro for each RDH type will wire all casting support that must exist
///     at compile time, some runtime wiring is required as well:
///     * Each library must register new RDH types and their downcast handlers via a pub init()
///       function and constant UUIDs.
///     * For example, within lib.rs for a hypothetical RDH extension library:
///
/// ```
///    pub(crate) const ICAR_TYPE_UUID: u128 = 997210989131515082813199923745508083862;
///    pub(crate) const CAR_TYPE_UUID: u128 = 161761007873234949512912432941437765752;
///     ...
///    const RDH_EXTENSION_UUID: u128 = 13776724624830763780350194868997388084;
///    pub fn init()
///    {
///        vehicle_library::init();  // This in turn will invoke rdh::init().
///        TypeRegistry::register_type::<dyn ICar>(ICAR_TYPE_UUID, RDH_EXTENSION_UUID);
///        TypeRegistry::register_type::<Car>(CAR_TYPE_UUID, RDH_EXTENSION_UUID);
///        ICarDowncaster::register_handler(TypeRegistry::library_identifier_of::<Car>(),
///                                         Box::new(DowncastsRdhExtensionTypesToICar{}));
///    }
///
/// ```
///
///   * When subsequent RDH libraries add new types downstream to $concrete_type, the
///     extend_downcasting! macro must be invoked and extensions to previous downcast handlers must
///     be added to init().
///     * For example, within lib.rs for an additional hypothetical RDH extension library:
///
/// ```
///    pub fn init()
///    {
///        // <library initialization as shown above>
///         ...
///        ICarDowncaster::register_handler(TypeRegistry::library_identifier_of::<FullSizeSedan>(),
///                                         Box::new(DowncastsSecondExtensionTypesToICar{}));
///    }
/// ```
///
#[macro_export]
macro_rules! provision_transmutation
{
    // *** Upcasting-only... ***
    {
        for $concrete_type: ident: $target_interface: ident
        {
            add Fn[$cast: ident, $cast_mut: ident] to $upcast_trait: ident
        }
    } =>
    {
        // Implement the ConcreteDivinator trait for $concrete_type.
        impl_concrete_divinator!($concrete_type, $target_interface);

        // Provision upcasting support for the public interfaces of all encompassing types.
        provision_upcasting!($upcast_trait, $target_interface, $cast, $cast_mut);
    };

    // *** The whole #!... ***
    {
        for $concrete_type: ident: $target_interface: ident + [$( $upstream_interface: ident )+]
        {
            add Fn[$cast: ident, $cast_mut: ident] to
            (
                $upcast_trait: ident,
                $downcast_trait: ident,
                $global_downcasting_struct: ident,
                $library_downcasting_struct: ident: $library_downcasting_trait: ident
                  $( include [$( $library_concrete_type: ident )+] )?
            )
        }
    } =>
    {
        // Implement the ConcreteDivinator trait for $concrete_type.
        impl_concrete_divinator!($concrete_type, $target_interface $(, $upstream_interface )+);

        // Provision upcasting support for the public interfaces of all encompassing types.
        provision_upcasting!($upcast_trait, $target_interface, $cast, $cast_mut);

        ///
        /// $downcast_trait: The trait that enables downcasting to $target_interface for all more
        ///                  abstract types.
        ///
        pub trait $downcast_trait
        {
            ///
            /// $cast: When possible, downcasts this immutable (IConstruct) to an immutable
            ///        $target_interface.
            ///
            fn $cast(&self) -> Option<&dyn $target_interface>;

            ///
            /// $cast_mut: When possible, downcasts this mutable (IConstruct) to a mutable
            ///            $target_interface.
            ///
            fn $cast_mut(&mut self) -> Option<&mut dyn $target_interface>;
        }
        $( impl $downcast_trait for dyn $upstream_interface
        {
            fn $cast(&self) -> Option<&dyn $target_interface>
            {
                // Send this instance to $target_interface's global downcast handler to coordinate
                // the downcast attempt.
                $global_downcasting_struct::$cast(self)
            }

            fn $cast_mut(&mut self) -> Option<&mut dyn $target_interface>
            {
                // Send this instance to $target_interface's global downcast handler to coordinate
                // the downcast attempt.
                $global_downcasting_struct::$cast_mut(self)
            }
        } )+

        ///
        /// $global_downcasting_struct: The singleton struct that manages downcasting to
        ///                             $target_interface from more abstract (IConstructs) across
        ///                             all RDH library boundaries.
        ///
        /// Notes:
        /// * This struct is primarily used implicitly in downcast calls to
        ///   (IConstruct).$cast(_mut), e.g. iconstruct.as_icar().
        /// * While such syntax will always be available for upcasts, there are situations where
        ///   downcast calls can't be made in this manner because downcasts are implemented for
        ///   *dyn* (IConstruct).
        ///   * In generic functions, for exmaple, there's no way to indicate to Rust that these
        ///     objects are really &dyn IConstruct (or any other downcast prerequisite.)
        ///   * In these cases, $global_downcasting_struct can be invoked directly, e.g.
        ///       $global_downcasting_struct::$cast(iconstruct);
        /// * For further information, see Issue #1 and Issue #3 in "Knowledge Base.md".
        ///
        pub struct $global_downcasting_struct
        {
            // NOTE: Code outside this module remains unable to instantiate this struct only while
            //       at least one field is kept private and no public constructor or Default
            //       implementation is added.

            // Per-library downcasting handlers, indexed by the identifiers of the libraries they
            // represent.
            handlers: HashMap<LibraryIdentifier, Box<dyn $library_downcasting_trait>>
        }
        impl $global_downcasting_struct
        {
            //
            // Implementation notes:
            //
            // * The $cast(_mut) methods call .as_iconstruct(_mut)() to cast iconstruct to an
            //   &(mut) dyn IConstruct before it's sent onto its appropriate handler.
            //   * This is needed because Rust provides no means of constraining a generic
            //     parameter to "anything that can be represented by &(mut) dyn IConstruct".
            //     * You can get: *only* &(mut) dyn IConstruct, any *concrete type* that implements
            //       IConstruct, or *anything* that implements IConstruct, but you can't get any
            //       &(mut) *dyn* IConstruct.
            // * For further information, see Issue #4 in "Knowledge Base.md".
            //
            
            //
            // *** Public functions ***
            //

            ///
            /// register_handler: Registers the specified $library_downcasting_trait object to
            ///                   handle downcasting more abstract (IConstructs) to
            ///                   $target_interfaces for the RDH library it represents.
            ///
            /// Parameters:
            /// * library_identifier (LibraryIdentifier):
            ///   The identifier of the library whose downcasting handler is being registered.
            /// * handler (Box<dyn $library_downcasting_trait>):
            ///   The $library_downcasting_trait object that will handle downcasting requests for
            ///   the indicated RDH library.
            ///
            /// Notes:
            /// * Callers yield ownership of both items sent to this function.
            /// * This function will block until exclusive singleton instance access can be
            ///   obtained, potentially forever.
            ///   * See this module's Implementation Notes.
            ///
            pub fn register_handler(library_identifier: LibraryIdentifier,
                                    handler: Box<dyn $library_downcasting_trait>)
            {
                // Implementation note: It would be better to allow access only to developers
                //                      extending, rather than consuming, the type hierarchy, but
                //                      Rust has no way to model that across libraries.

                let mut instance = unsafe { $global_downcasting_struct::rwlock() }.write().expect(
                  "$global_downcasting_struct::register_handler(): Unable to acquire mutable \
                   $global_downcasting_struct singleton instance!");
                if instance.handlers.contains_key(&library_identifier) == false
                {
                    instance.handlers.insert(library_identifier, handler);
                }
                else
                {
                    panic!("$global_downcasting_struct::register_handler(): A \
                           $library_downcasting_trait handler for the specified library has \
                           already been registered within $global_downcasting_struct!");
                }
            }

            ///
            /// $cast: Downcasts the specified more abstract immutable (IConstruct) object to an
            ///        immutable $target_interface object.
            ///
            /// Generic types:
            /// * T (IConstruct + ?Sized): Represents all (IConstructs).
            ///
            /// Parameters:
            /// * iconstruct (&T): The immutable more abstract (IConstruct) object to downcast to
            ///                    an immutable $target_interface.
            ///
            /// Expresses:
            /// * When successful, an Option<&dyn $target_interface> whose immutable object
            ///   corresponds to the same concrete instance represented by the immutable
            ///   (IConstruct) object.
            /// * Otherwise, expresses None.
            /// 
            pub fn $cast<T>(iconstruct: &T) -> Option<&dyn $target_interface>
                where T: IConstruct + ?Sized
            {
                // Route the downcast request to the handler registered for the RDH library in
                // which the specified (IConstruct's) underlying concrete type was defined.
                let instance = unsafe { $global_downcasting_struct::rwlock() }.read().expect(
                  "$global_downcasting_struct::$cast(): Unable to acquire immutable \
                   $global_downcasting_struct singleton instance!");
                let library_identifier = iconstruct.concrete_library_identifier();
                if instance.handlers.contains_key(&library_identifier) == true
                {
                    instance.handlers[&library_identifier].$cast(iconstruct.as_iconstruct())
                }
                else
                {
                    None
                }
            }

            ///
            /// $cast_mut: Downcasts the specified more abstract mutable (IConstruct) object to a
            ///            mutable $target_interface object.
            ///
            /// Generic types:
            /// * T (IConstruct + ?Sized): Represents all (IConstructs).
            ///
            /// Parameters:
            /// * iconstruct (&mut T): The mutable more abstract (IConstruct) object to downcast to
            ///                        a mutable $target_interface.
            ///
            /// Expresses:
            /// * When successful, an Option<&mut dyn $target_interface> whose mutable object
            ///   corresponds to the same concrete instance represented by the mutable (IConstruct)
            ///   object.
            /// * Otherwise, expresses None.
            ///
            pub fn $cast_mut<T>(iconstruct: &mut T) -> Option<&mut dyn $target_interface>
                where T: IConstruct + ?Sized
            {
                // Route the downcast request to the handler registered for the RDH library in
                // which the specified (IConstruct's) underlying concrete type was defined.
                let instance = unsafe { $global_downcasting_struct::rwlock() }.read().expect(
                  "$global_downcasting_struct::$cast_mut(): Unable to acquire immutable \
                   $global_downcasting_struct singleton instance!");
                let library_identifier = iconstruct.concrete_library_identifier();
                if instance.handlers.contains_key(&library_identifier) == true
                {
                    instance.handlers[&library_identifier]
                      .$cast_mut(iconstruct.as_iconstruct_mut())
                }
                else
                {
                    None
                }
            }

            //
            // *** Private functions ***
            //

            //
            // rwlock: Obtains a reference to the reader/writer lock that guards access to the
            //         singleton $global_downcasting_struct instance (creating both when
            //         necessary).
            //
            // Expresses: A reference to the lock that guards the singleton instance.
            //
            unsafe fn rwlock<'a>() -> &'a RwLock<$global_downcasting_struct>
            {
                // Implementation note: Structs may not contain static fields, so we have to
                //                      declare the singleton instance and the rwlock that guards
                //                      it here (which has its advantages).

                // The RwLock, embedded within an Option, that guards the singleton
                //  $global_downcasting_struct instance.
                static mut RWLOCK: Option<RwLock<$global_downcasting_struct>> = None;
        
                // Create the rwlock and singleton instance the first time this function is called
                // (while all other calling threads are blocked).
                static ONCE: Once = Once::new();
                ONCE.call_once(||
                               {
                                   RWLOCK = Some(RwLock::new($global_downcasting_struct
                                                 {
                                                     handlers: HashMap::new()
                                                 }));
                               });
        
                RWLOCK.as_ref().expect("$global_downcasting_struct::rwlock(): Unable to acquire \
                                        the lock that guards the $global_downcasting_struct \
                                        singleton instance!")
            }
        }

        ///
        /// $library_downcasting_trait: The trait that enables a struct to handle downcasting to
        ///                             $target_interface for those underlying concrete types that
        ///                             are defined within the library they represent.
        ///
        pub trait $library_downcasting_trait
        {
            // Implementation note: It would be better to allow access only to developers
            //                      extending, rather than consuming, the type hierarchy, but Rust
            //                      has no way to model that across libraries.

            ///
            /// $cast: Downcasts the specified immutable (IConstruct) to an immutable
            ///        $target_interface object when possible.
            ///
            /// Parameters:
            /// * &self: Necessary to allow instance to be addressed abstractly by the global
            ///          downcast handler (but not used).
            /// * iconstruct (&dyn IConstruct): The immutable (IConstruct) to downcast into an
            ///                                 immutable $target_interface.
            ///
            /// Expresses:
            /// * If successful, the concrete instance underlying the immutable (IConstruct) as an
            ///   Option<&$target_interface>.
            /// * Otherwise expresses None.
            ///
            fn $cast<'a>(&self, iconstruct: &'a dyn IConstruct) ->
              Option<&'a dyn $target_interface>;

            ///
            /// $cast_mut: Downcasts the specified mutable (IConstruct) to a mutable
            ///            $target_interface object when possible.
            ///
            /// Parameters:
            /// * &self: Necessary to allow instance to be addressed abstractly by the global
            ///          downcast handler (but not used).
            /// * iconstruct (&mut dyn IConstruct): The mutable (IConstruct) to downcast into a
            ///                                     mutable $target_interface.
            ///
            /// Expresses:
            /// * If successful, the concrete instance underlying the mutable (IConstruct) as an
            ///   Option<&mut $target_interface>.
            /// * Otherwise expresses None.
            ///
            fn $cast_mut<'a>(&self, iconstruct: &'a mut dyn IConstruct) ->
              Option<&'a mut dyn $target_interface>;
        }
        // Add the current library's $library_downcasting_struct and its implementation of
        // $library_downcasting_trait via a companion public macro:
        extend_downcasting!
        {
            for $target_interface
            {
                add Fn[$cast, $cast_mut] to $library_downcasting_struct: $library_downcasting_trait
                  include [$concrete_type $( $( $library_concrete_type )+ )?]
            }
        }
    }
}

///
/// extend_downcasting: Extends downcasting support to types downstream from $concrete_type within
///                     each RDH library.
///
/// Parameters:
/// * $concrete_type (ident): The concrete type for which downcasting is being extended.
/// * $target_interface (ident): The public interface to which downcasting methods will cast the
///                              (IConstructs) sent to them.
/// * $cast (ident): The name of the method that casts immutable (IConstruct) objects to immutable
///                  $target_interfaces objects, within all provisioned traits for both upcasting
///                  and downcasting.
/// * $cast_mut (ident): The name of the method that casts mutable (IConstruct) objects to mutable
///                      $target_interfaces objects, within all provisioned traits for both
///                      upcasting and downcasting.
/// * $library_downcasting_trait (ident): The name to assign to the trait implemented by structs
///                                       that handle downcasting to $target_interface for
///                                       individual RDH libraries.
/// * $library_downcasting_struct (ident): The name to assign to the struct that handles
///                                        downcasting to $target_interface for the current RDH
///                                        library.
/// * $library_concrete_type (ident): The space-delimited list of downstream concrete types within
///                                   the current RDH library whose (IConstruct) objects may be
///                                   downcast to $target_interface, if any.
///
/// # Example invocation:
///
/// ```
///  extend_downcasting!
///  {
///      for ICar
///      {
///          add Fn[as_icar, as_icar_mut] to DowncastsSecondExtensionTypesToICar:
///            DowncastsLibraryTypesToICar include [FullSizeSedan MidSizeSedan CompactSedan]
///      }
///  }
/// ```
///
/// Notes:
/// * Each of these extensions must also be complemented by a runtime call within library init
///   function, e.g.:
///
/// ```
///    pub fn init()
///    {
///         ...
///        ICarDowncaster::register_handler(TypeRegistry::library_identifier_of::<FullSizeSedan>(),
///                                         Box::new(DowncastsSecondExtensionTypesToICar{}));
///    }
/// ```
///
#[macro_export]
macro_rules! extend_downcasting
{
    // Implementation note: This macro is also used privately by provision_transmuation! to
    //                      provision downcasting for each type within its own library.

    {
        for $target_interface: ident
        {
            add Fn[$cast: ident, $cast_mut: ident] to $library_downcasting_struct: ident:
              $library_downcasting_trait: ident include [$concrete_type: ident
              $( $library_concrete_type: ident )*]
        }
    } =>
    {
        ///
        /// The downcasting struct for the current library and its $library_downcasting_trait
        /// implementation.
        ///
        pub struct $library_downcasting_struct
        {
            // Implementation note: It would be better to allow access only to developers
            // extending, rather than consuming, the type hierarchy, but Rust has no way to model
            // that across libraries.
        }
        impl $library_downcasting_trait for $library_downcasting_struct
        {
            //
            // $cast: Downcasts the specified immutable (IConstruct) to an immutable
            //        $target_interface object when possible.
            //
            // Parameters:
            // * &self: Necessary to allow instance to be addressed abstractly by the global
            //          downcast handler (but not used).
            // * iconstruct (&dyn IConstruct): The immutable (IConstruct) to downcast into an
            //                                 immutable $target_interface.
            //
            // Expresses:
            // * If successful, the concrete instance underlying the immutable (IConstruct) as an
            //   Option<&$target_interface>.
            // * Otherwise expresses None.
            //
            fn $cast<'a>(&self, iconstruct: &'a dyn IConstruct) ->
              Option<&'a dyn $target_interface>
            {
                if iconstruct.is(TypeRegistry::type_identifier_of::<$concrete_type>())
                {
                    if let Some(construct) = iconstruct.as_concrete::<$concrete_type>()
                    {
                        Some(construct as &dyn $target_interface)
                    }
                    else
                    {
                        None
                    }
                } $(
                else if iconstruct.is(TypeRegistry::type_identifier_of::<$library_concrete_type>())
                {
                    if let Some(construct) = iconstruct.as_concrete::<$library_concrete_type>()
                    {
                        Some(construct as &dyn $target_interface)
                    }
                    else
                    {
                        None
                    }
                } )*
                else
                {
                    None
                }
            }

            //
            // $cast_mut: Downcasts the specified mutable (IConstruct) to a mutable
            //            $target_interface object when possible.
            //
            // Parameters:
            // * &self: Necessary to allow instance to be addressed abstractly by the global
            //          downcast handler (but not used).
            // * iconstruct (&mut dyn IConstruct): The mutable (IConstruct) to downcast into a
            //                                     mutable $target_interface.
            //
            // Expresses:
            // * If successful, the concrete instance underlying the mutable (IConstruct) as an
            //   Option<&mut $target_interface>.
            // * Otherwise expresses None.
            //
            fn $cast_mut<'a>(&self, iconstruct: &'a mut dyn IConstruct) ->
              Option<&'a mut dyn $target_interface>
            {
                if iconstruct.is(TypeRegistry::type_identifier_of::<$concrete_type>())
                {
                    if let Some(construct) = iconstruct.as_concrete_mut::<$concrete_type>()
                    {
                        Some(construct as &mut dyn $target_interface)
                    }
                    else
                    {
                        None
                    }
                } $(
                else if iconstruct.is(TypeRegistry::type_identifier_of::<$library_concrete_type>())
                {
                    if let Some(construct) = iconstruct.as_concrete_mut::<$library_concrete_type>()
                    {
                        Some(construct as &mut dyn $target_interface)
                    }
                    else
                    {
                        None
                    }
                } )*
                else
                {
                    None
                }
            }
        }
    }
}

//
// *** Private macros ***
//
// Notes:
// * The following macros are part of the provision_transmutation! macro, but are used in two
//   different cases (upcast only and both upcast and downcast).
// * To avoid quite a lot of repeated code, these features were pulled out into their own macros.
// * While the following macros are therefore meant to be private, Rust has no means of hydrating
//   them and then injecting them into their parent macro before hydrating and injecting the parent
//   macro into the position from which it was invoked.
// * Consequently, although placed within a section of the module labelled "private," we have no
//   choice but to export these macros anyway.
//

//
// impl_concrete_divinator: Implements the ConcreteDivinator trait for the specified concrete type.
//
// Parameters:
// * $concrete_type (ty): The type of the concrete (Construct) instance for which ConcreteDivinator
//                        will be implemented.
// * $introduced_interface (ident): The public interface introduced by the concrete type to
//                                  represent it abstractly.
// * $implemented_interface (ident): Additional public interfaces implemented by the concrete
//                                   (Construct) type, if any.
//
// Example invocation:
//  impl_concrete_divinator!(Car, ICar, IVehicle, IConstruct);
//
#[macro_export]
macro_rules! impl_concrete_divinator
{
    ($concrete_type: ty, $introduced_interface: ident $( , $implemented_interface: ident)*) =>
    {
        //
        // ConcreteDivinator implementation for $concrete_type.
        //
        impl ConcreteDivinator for $concrete_type
        {
            //
            // concrete_type_identifier: Retrieves the identifier for the current instance's
            //                           concrete type.
            //
            fn concrete_type_identifier(&self) -> TypeIdentifier
            {
                self.type_identifier()
            }

            //
            // concrete_library_identifier: Retrieves the identifier for the library in which the
            //                              current instance's concrete type is defined.
            //
            fn concrete_library_identifier(&self) -> LibraryIdentifier
            {
                self.library_identifier()
            }

            //
            // implements: Determines whether this object's concrete type implements the indicated
            //             public interface.
            //
            // Parameters:
            // * interface_type (TypeIdentifier): The type identifier of the interface type to look
            //                                    for.
            //
            // Expresses:
            // * If this object's concrete type implements the indicated public interface,
            //   expresses true.
            // * Otherwise, expresses false.
            //
            fn implements(&self, interface_type: TypeIdentifier) -> bool  
            {
                interface_type == TypeRegistry::type_identifier_of::<dyn $introduced_interface>()
                $(
                || interface_type ==
                     TypeRegistry::type_identifier_of::<dyn $implemented_interface>()
                )*
            }

            //
            // is: Determines whether this object's concrete type *literally* is the indicated
            //     type.
            //
            // Parameters:
            // * test_type (TypeIdentifier): The type identifier of the type being tested.
            //
            // Expresses:
            // * If this object's concrete type *literally* is the indicated type, expresses true.
            // * Otherwise, expresses false.
            //
            // IMPORTANT DISTINCTIONS FROM INHERITANCE-BASED LANGUAGES:
            // * Although in languages such as C#, "is" will also return true if test_type is an
            //   ancestor of this object's type or one of its implemented interfaces, this function
            //   will return *false*.
            //   * The *only* time "is" will return true is when the specified TypeIdentifier is
            //     the *exact* type identifier for this object's concrete type.
            // * To determine whether a type implements a given interface, call implements() rather
            //   than is().
            // * To determine whether a type is "derived" from another type (or, really,
            //   *encompasses* it), call implements(), specifying the type's corresponding public
            //   interface, e.g.
            //
            //   if car.implements(TypeRegistry::type_identifier_of::<IVehicle>())
            //   {
            //       println!("A Car IS a Vehicle!");
            //   }
            //
            // Discussion points:
            // * While returning true may be correct for other "is" queries on a conceptual level,
            //   Rust's use of composition over inheritance makes doing so problematic, especially
            //   in casting scenarios.
            //   * In C# "is" returning true for the same Car before and after an upcast to Vehicle
            //     is correct on both conceptual and literal levels-- they're the same object at
            //     the same memory address.
            //   * In Rust, however, "is" returning true after such an upcast would *only* be
            //     correct *conceptually*-- Vehicle and Car are different objects at different
            //     memory addresses!
            //   * That these objects are not literally the same would also break polymorphism.
            //     * Were calls made to a Vehicle instance created via an upcast from Car, the
            //       *Vehicle* instance of all virtual functions would be incorrectly invoked
            //       rather than the Car version.
            //   * Considering both of these problems, RDH will *not* support casting between
            //     concrete types.
            //
            fn is(&self, test_type: TypeIdentifier) -> bool
            {
                self.type_identifier() == test_type
            }
        }
    }
}

//
// provision_upcasting: Provisions the traits needed to support upcasting between interfaces, even
//                      when the underlying concrete type is unknown.
//
// Parameters:
// * $upcast_trait (ident): The name to assign to the trait that enables upcasting to
//                          $target_interface for types that encompass the corresponding concrete
//                          type.
// * $target_interface (ident): The public interface to which upcasting methods will cast the
//                              (IConstructs) sent to them.
// * $cast (ident): The name of the $upcast_trait method that upcasts immutable (IConstruct)
//                  objects to immutable $target_interfaces objects.
// * $cast_mut (ident): The name of the $upcast_trait method that upcasts mutable (IConstruct)
//                      objects to mutable $target_interfaces objects.
//
// Example invocation:
//  provision_upcasting!(UpcastsToIConstruct, IConstruct, as_iconstruct, as_iconstruct_mut);
//
// Notes:
// * Unlike downcasts, upcasting functions do not return Options.
//   * Upcasts will always succeed as all necessary validation happens at compile time.
//
// * This macro will implement upcasting for all applicable (IConstructs) as well as all applicable
//   (Constructs).
//   * For (Constructs), this is redundant with Rust's ability to directly cast a concrete type to
//     a trait object, and for (IConstructs), this is redundant with the forthcoming feature to
//     support trait object upcasting.
//     * See https://github.com/rust-lang/rust/issues/65991 for further information on Rust's
//       upcoming native support, currently flagged "ready to stabilize."
//   * While developers who consume the RDH framework should probably use Rust's built-in syntax,
//     consider:
//
//        let icar = &sedan as &dyn ICar;
//        let iconstruct = icar as &dyn IConstruct;  // Will work in the Nightly Channel / when the
//                                                   // trait object upcasting feature is
//                                                   // integrated into the Stable Channel.
//        // Downcasts are *not* supported by Rust in any channel.
//
//      vs.
//
//        let icar = sedan.as_icar();
//        let iconstruct = icar.as_iconstruct();  // Will always work.
//        // Downcasts: let isedan = icar.as_sedan();  // (where icar is a reference to at least a
//        //                                           //  Sedan)-- also works today!
//
#[macro_export]
macro_rules! provision_upcasting
{
    ($upcast_trait: ident, $target_interface: ident, $cast: ident, $cast_mut: ident) =>
    {
        //
        // $upcast_trait: The trait added to (IConstruct) types that encompass $target_interface to
        //                enable upcasting.
        //
        pub trait $upcast_trait
        {
            //
            // $cast: Upcasts this immutable (IConstruct) to an immutable $target_interface.
            //
            fn $cast(&self) -> &dyn $target_interface;

            //
            // $cast_mut: Upcasts this mutable (IConstruct) to a mutable $target_interface.
            //
            fn $cast_mut(&mut self) -> &mut dyn $target_interface;
        }
        impl<T> $upcast_trait for T where T: $target_interface
        {
            fn $cast(&self) -> &dyn $target_interface
            {
                self
            }

            fn $cast_mut(&mut self) -> &mut dyn $target_interface
            {
                self
            }
        }
    }
}

//
// *** Minutiae ***
//

// Traits and types
use std::any;
use std::collections::HashMap;
use std::sync::{ Once, RwLock };
use crate::_hierarchy::construct::IConstruct;
