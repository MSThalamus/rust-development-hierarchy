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
//! checkbox.rs
//!
//! ICheckbox (Checkbox)
//!   The public interface and struct for checkboxes of all varieties.
//!
//! THIS IS A WORK IN PROGRESS. All current functionality is placeholder only and *will* change.
//! (This type will be removed altogether.)
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// Checkbox struct
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Checkbox: The implementation of (Checkbox) functionality.
///
pub struct Checkbox
{
    //
    // ui_element: Stores this (Checkbox's) UIElement instance.
    //
    ui_element: UIElement
}
impl Checkbox
{
    ///
    /// new: Creates a new instance of the Checkbox struct.
    ///
    /// # Example
    ///
    /// ```
    /// use rdh_extension_example::_hierarchy::checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new();
    /// ```
    ///
    pub fn new() -> Checkbox
    {
        Checkbox
        {
            ui_element: UIElement::new()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// ICheckbox public interface trait and component / supplemental traits
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// ICheckbox: The interface implemented by all (Checkboxes).
//             * Non-virtual functions are directly declared and implemented below.
//             * Virtual functions are declared within the CheckboxVirtuals trait.
//               Their base implementation is below-- encompassing types override as needed.
//             * Casting functionality provided by additional component and supplemental traits.
///
pub trait ICheckbox : CheckboxVirtuals + UpcastsToICheckbox +
                      IUIElement + IConstruct
{
}
pub trait CheckboxVirtuals
{
}
impl<T> ICheckbox for T where T: CheckboxVirtuals + CheckboxInstances +
                                 UIElementVirtuals + UIElementInstances +
                                 ConstructVirtuals + ConstructInstances +
                                 ConcreteDivinator
{
}
impl CheckboxVirtuals for Checkbox
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Inherited virtual function overrides
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// IUIElement overrides.
//
impl UIElementVirtuals for Checkbox
{
}

//
// IConstruct overrides.
//
impl ConstructVirtuals for Checkbox
{
    ///
    /// on_click: Handles click events for this (Checkbox), according to its concrete type.
    ///
    /// THIS METHOD IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdh_extension_example::_hierarchy::checkbox::Checkbox;
    ///
    /// let mut checkbox_mut = Checkbox::new();
    /// checkbox_mut.on_click();
    /// ```
    ///
    fn on_click(&mut self)
    {
        // Carry out base type actions.
        self.ui_element_mut().on_click();

        // Perform additional work.
        println!("on_click handled by Checkbox implementation! \"Inheritance\" FTW!");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Macro Invocations ***

// Implement upcasting and downcasting to ICheckbox and between interfaces Checkbox implements.
provision_transmutation!
{
    for Checkbox: ICheckbox + [IUIElement IConstruct]
    {
        add Fn[as_icheckbox, as_icheckbox_mut] to
        (
            UpcastsToICheckbox,
            DowncastsToICheckbox,
            ICheckboxDowncaster,
            DowncastsRdhExtensionTypesToICheckbox: DowncastsLibraryTypesToICheckbox
        )
    }
}

// Provide access to this Checkbox instance and the encompassed UIElement and Construct instances.
define_instances! { CheckboxInstances for Checkbox : Fn[checkbox, checkbox_mut] -> Checkbox }
define_instances! { UIElementInstances for Checkbox : Fn[ui_element, ui_element_mut] ->
                      UIElement (ui_element) }
define_instances! { ConstructInstances for Checkbox : Fn[construct, construct_mut] ->
                      Construct (ui_element {.construct} {.construct_mut}) }

// *** Internal (i.e. public in crate) constants ***

//
// ICHECKBOX_TYPE_UUID: The unique identifier for the ICheckbox type.
// CHECKBOX_TYPE_UUID: The unique identifier for the Checkbox type.
//
pub(crate) const ICHECKBOX_TYPE_UUID: u128 = 72533741563326067592427286561735479339;
pub(crate) const CHECKBOX_TYPE_UUID: u128 = 20792203623865871595846094854671916392;

// *** Minutiae ***

// Traits and types
use rdh::_hierarchy::construct::{ Construct, ConstructInstances, ConstructVirtuals, IConstruct };
use rdh::_infrastructure::thaumaturgy::ConcreteDivinator;
use crate::_hierarchy::ui_element::{ IUIElement, UIElement, UIElementInstances,
                                     UIElementVirtuals };

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
