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
//! lib.rs (rdh_extension_example)
//!
//! Contains module declarations for and initialization of this example extension to the Rust
//! Development Hierarchy.
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// External crates
///////////////////////////////////////////////////////////////////////////////////////////////////

extern crate rdh;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Public modules
///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod _extras;
pub mod _hierarchy;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Public initialization functions
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// init: Initializes this example RDH extension library and its ancestors.
///
/// Project main() function must invoke the init() function for the top-level RDH library.
/// 

//
// init: Initializes this example RDH extension library and its ancestors.
//
pub fn init()
{
    //
    // Implementation notes:
    //
    // * It would be better to allow access only to developers extending, rather than consuming,
    //   the type hierarchy, but Rust has no way to model that across libraries.
    //   * Alternative solutions are being considered at this time.
    //     * For further information, see Issue #5 in Knowledge Base.md.
    //
    // * The init() function for all RDH libraries must invoke init() from their parent library.
    //

    // Initialize the core RDH library.
    rdh::init();

    // Register new framework types added within this library.
    TypeRegistry::register_type::<dyn IUIElement>(IUI_ELEMENT_TYPE_UUID, RDH_EXTENSION_UUID);
    TypeRegistry::register_type::<UIElement>(UI_ELEMENT_TYPE_UUID, RDH_EXTENSION_UUID);
    TypeRegistry::register_type::<dyn ICheckbox>(ICHECKBOX_TYPE_UUID, RDH_EXTENSION_UUID);
    TypeRegistry::register_type::<Checkbox>(CHECKBOX_TYPE_UUID, RDH_EXTENSION_UUID);

    // Create and register downcast handlers for existing interfaces when the concrete type is
    // defined within this library.
    IUIElementDowncaster::register_handler(TypeRegistry::library_identifier_of::<UIElement>(),
                                           Box::new(DowncastsRdhExtensionTypesToIUIElement{}));
    ICheckboxDowncaster::register_handler(TypeRegistry::library_identifier_of::<Checkbox>(),
                                          Box::new(DowncastsRdhExtensionTypesToICheckbox{}));
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Private initialization infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Private constants ***

//
// RDH_EXTENSION_UUID: The unique identifier for this example RDH extension library.
//
const RDH_EXTENSION_UUID: u128 = 41377672462483076378035019486899738808;

// *** Minutiae ***

use rdh::_infrastructure::thaumaturgy::TypeRegistry;
use crate::_hierarchy::checkbox::{ Checkbox, CHECKBOX_TYPE_UUID,
                                   DowncastsRdhExtensionTypesToICheckbox, ICheckbox,
                                   ICheckboxDowncaster, ICHECKBOX_TYPE_UUID };
use crate::_hierarchy::ui_element::{ DowncastsRdhExtensionTypesToIUIElement, IUIElement,
                                     IUIElementDowncaster, IUI_ELEMENT_TYPE_UUID, UIElement,
                                     UI_ELEMENT_TYPE_UUID };
