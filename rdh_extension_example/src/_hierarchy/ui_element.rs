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
//! * (IUIElement) means an IUIElement trait object or any trait object encompassing IUIElement.
//! * (UIElement) means the UIElement struct or any concrete type that implements IUIElement.
//! Whereas:
//! * IUIElement (no parentheses) means the literal IUIElement trait or its trait objects.
//! * UIElement (no parentheses) means the literal UIElement struct or its instances.
//!

//!
//! ui_element.rs
//!
//! IUIElement (UIElement)
//!   The public interface and struct that serve as the root for UI elements.
//!
//! THIS IS A WORK IN PROGRESS. All current functionality is placeholder only and *will* change.
//! (This type will be removed altogether.)
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// UIElement struct
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// UIElement: The implementation of (UIElement) functionality.
///
pub struct UIElement
{
    //
    // construct: Stores this (UIElement's) Construct instance.
    //
    construct: Construct
}
impl UIElement
{
    ///
    /// new: Creates a new instance of the UIElement struct.
    ///
    /// # Example
    ///
    /// ```
    /// use rdh_extension_example::_hierarchy::ui_element::UIElement;
    ///
    /// let ui_element = UIElement::new();
    /// ```
    ///
    pub fn new() -> UIElement
    {
        UIElement
        {
            construct: Construct::new()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// IUIElement public interface trait and component / supplemental traits
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// IUIElement: The interface implemented by all (UIElements).
//              * Non-virtual functions are directly declared and implemented below.
//              * Virtual functions are declared within the UIElementVirtuals trait.
//                Their base implementation is below-- encompassing types override as needed.
//              * Casting functionality provided by additional component and supplemental traits.
///
pub trait IUIElement : UIElementVirtuals + UpcastsToIUIElement +
                       IConstruct
{
}
pub trait UIElementVirtuals
{
}
impl<T> IUIElement for T where T: UIElementVirtuals + UIElementInstances +
                                  ConstructVirtuals + ConstructInstances +
                                  ConcreteDivinator
{
}
impl UIElementVirtuals for UIElement
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Inherited virtual function overrides
///////////////////////////////////////////////////////////////////////////////////////////////////

//
// IConstruct overrides.
//
impl ConstructVirtuals for UIElement
{
    ///
    /// on_click: Handles click events for this (UIElement), according to its concrete type.
    ///
    /// THIS METHOD IS PLACEHOLDER ONLY AND WILL BE REMOVED AT A LATER DATE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdh_extension_example::_hierarchy::ui_element::UIElement;
    ///
    /// let mut ui_element_mut = UIElement::new();
    /// ui_element_mut.on_click();
    /// ```
    ///
    fn on_click(&mut self)
    {
        // Carry out base type actions.
        self.construct_mut().on_click();

        // Perform additional work.
        println!("on_click handled by UIElement implementation! \"Inheritance\" FTW!");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Macro Invocations ***

// Implement upcasting and downcasting to IUIElement and between interfaces UIElement implements.
provision_transmutation!
{
    for UIElement: IUIElement + [IConstruct]
    {
        add Fn[as_iui_element, as_iui_element_mut] to
        (
            UpcastsToIUIElement,
            DowncastsToIUIElement,
            IUIElementDowncaster,
            DowncastsRdhExtensionTypesToIUIElement: DowncastsLibraryTypesToIUIElement
              include [Checkbox]
        )
    }
}

// Provide access to the current UIElement instance and the encompassed Construct instance.
define_instances! { UIElementInstances for UIElement : Fn[ui_element, ui_element_mut] ->
                      UIElement }
define_instances! { ConstructInstances for UIElement : Fn[construct, construct_mut] ->
                      Construct (construct) }

// *** Internal (i.e. public in crate) constants ***

//
// IUI_ELEMENT_TYPE_UUID: The unique identifier for the IUIElement type.
// UI_ELEMENT_TYPE_UUID: The unique identifier for the UIElement type.
//
pub(crate) const IUI_ELEMENT_TYPE_UUID: u128 = 299721098913151508281319992374550808386;
pub(crate) const UI_ELEMENT_TYPE_UUID: u128 = 216176100787323494951291243294143776575;

// *** Minutiae ***

// Traits and types
use rdh::_hierarchy::construct::{ Construct, ConstructInstances, ConstructVirtuals, IConstruct };
use rdh::_infrastructure::thaumaturgy::ConcreteDivinator;
use crate::_hierarchy::checkbox::Checkbox;

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
