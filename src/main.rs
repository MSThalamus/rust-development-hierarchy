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

extern crate rdh;
extern crate rdh_extension_example;

use rdh::_hierarchy::construct::{ Construct, IConstruct };
use rdh::_infrastructure::thaumaturgy::{ ConcreteDivinator, Divinator, TypeRegistry };
use rdh_extension_example::_hierarchy::ui_element::{ IUIElement, UIElement };

fn test_concrete_identifiers()
{
    println!("");
    println!("**********************************************************************************");
    println!("ConcreteDivinator::concrete_type_identifier() and concrete_library_identifier():");
    println!("**********************************************************************************");
    println!("");

    let construct = Construct::new();

    // Should be 276154362193470083307838638451583829221 / 88009063541924297814935272469493149666:
    println!("{:?}", construct.concrete_type_identifier());

    // Should be 88009063541924297814935272469493149666:
    println!("{:?}", construct.concrete_library_identifier());

    //TODO: When formalizing these tests, should use the Transmutation syntax?
    //      i.e. let iconstruct = construct.as_iconstruct();
    let iconstruct = &construct as &dyn IConstruct;

    // Should be 276154362193470083307838638451583829221 / 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct.concrete_type_identifier());

    // Should be 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct.concrete_library_identifier());

    println!("");

    let ui_element = UIElement::new();

    // Should be 216176100787323494951291243294143776575 / 41377672462483076378035019486899738808:
    println!("{:?}", ui_element.concrete_type_identifier());

    // Should be 41377672462483076378035019486899738808:
    println!("{:?}", ui_element.concrete_library_identifier());

    let iconstruct2 = &ui_element as &dyn IConstruct;

    // Should be 216176100787323494951291243294143776575 / 41377672462483076378035019486899738808:
    println!("{:?}", iconstruct2.concrete_type_identifier());

    // Should be 41377672462483076378035019486899738808:
    println!("{:?}", iconstruct2.concrete_library_identifier());

    let iui_element = &ui_element as &dyn IUIElement;

    // Should be 216176100787323494951291243294143776575 / 41377672462483076378035019486899738808:
    println!("{:?}", iui_element.concrete_type_identifier());

    // Should be 41377672462483076378035019486899738808:
    println!("{:?}", iui_element.concrete_library_identifier());

    println!("");
}

fn test_concrete_implements()
{
    println!("");
    println!("**********************************************************************************");
    println!("ConcreteDivinator::implements():");
    println!("**********************************************************************************");
    println!("");

    let construct = Construct::new();

    // Should be true:
    println!("{}", construct.implements(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", construct.implements(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", construct.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    // Should be false:
    println!("{}", construct.implements(TypeRegistry::type_identifier_of::<UIElement>()));

    let iconstruct = &construct as &dyn IConstruct;

    // Should be true:
    println!("{}", iconstruct.implements(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", iconstruct.implements(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", iconstruct.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    // Should be false:
    println!("{}", iconstruct.implements(TypeRegistry::type_identifier_of::<UIElement>()));

    println!("");
    
    let ui_element = UIElement::new();

    // Should be true:
    println!("{}", ui_element.implements(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", ui_element.implements(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be true:
    println!("{}", ui_element.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    // Should be false:
    println!("{}", ui_element.implements(TypeRegistry::type_identifier_of::<UIElement>()));

    let iconstruct2 = &ui_element as &dyn IConstruct;

    // Should be true:
    println!("{}", iconstruct2.implements(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", iconstruct2.implements(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be true:
    println!("{}", iconstruct2.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    // Should be false:
    println!("{}", iconstruct2.implements(TypeRegistry::type_identifier_of::<UIElement>()));

    let iui_element = &ui_element as &dyn IUIElement;

    // Should be true:
    println!("{}", iui_element.implements(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", iui_element.implements(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be true:
    println!("{}", iui_element.implements(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    // Should be false:
    println!("{}", iui_element.implements(TypeRegistry::type_identifier_of::<UIElement>()));

    println!("");
}

fn test_concrete_is()
{
    println!("");
    println!("**********************************************************************************");
    println!("ConcreteDivinator::is():");
    println!("**********************************************************************************");
    println!("");

    let construct = Construct::new();

    // Should be true:
    println!("{}", construct.is(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", construct.is(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", construct.is(TypeRegistry::type_identifier_of::<UIElement>()));

    // Should be false:
    println!("{}", construct.is(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    let iconstruct = &construct as &dyn IConstruct;

    // Should be true:
    println!("{}", iconstruct.is(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", iconstruct.is(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be false:
    println!("{}", iconstruct.is(TypeRegistry::type_identifier_of::<UIElement>()));

    // Should be false:
    println!("{}", iconstruct.is(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    println!("");
    
    let ui_element = UIElement::new();

    // Should be false:
    println!("{}", ui_element.is(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", ui_element.is(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be true:
    println!("{}", ui_element.is(TypeRegistry::type_identifier_of::<UIElement>()));

    // Should be false:
    println!("{}", ui_element.is(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    let iconstruct2 = &ui_element as &dyn IConstruct;

    // Should be false:
    println!("{}", iconstruct2.is(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", iconstruct2.is(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be true:
    println!("{}", iconstruct2.is(TypeRegistry::type_identifier_of::<UIElement>()));

    // Should be false:
    println!("{}", iconstruct2.is(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    let iui_element = &ui_element as &dyn IUIElement;

    // Should be false:
    println!("{}", iui_element.is(TypeRegistry::type_identifier_of::<Construct>()));

    // Should be false:
    println!("{}", iui_element.is(TypeRegistry::type_identifier_of::<dyn IConstruct>()));

    // Should be true:
    println!("{}", iui_element.is(TypeRegistry::type_identifier_of::<UIElement>()));

    // Should be false:
    println!("{}", iui_element.is(TypeRegistry::type_identifier_of::<dyn IUIElement>()));

    println!("");
}

fn test_divinator()
{
    println!("");
    println!("**********************************************************************************");
    println!("Divinator::type_identifier() and library_identifier():");
    println!("**********************************************************************************");
    println!("");

    let construct = Construct::new();

    // Should be 276154362193470083307838638451583829221 / 88009063541924297814935272469493149666:
    println!("{:?}", construct.type_identifier());

    // Should be 88009063541924297814935272469493149666:
    println!("{:?}", construct.library_identifier());

    let iconstruct = &construct as &dyn IConstruct;

    // Should be 50657587353538260646247596146820191381 / 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct.type_identifier());

    // Should be 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct.library_identifier());

    println!("");

    let ui_element = UIElement::new();

    // Should be 216176100787323494951291243294143776575 / 41377672462483076378035019486899738808:
    println!("{:?}", ui_element.type_identifier());

    // Should be 41377672462483076378035019486899738808:
    println!("{:?}", ui_element.library_identifier());

    let iconstruct2 = &ui_element as &dyn IConstruct;

    // Should be 50657587353538260646247596146820191381 / 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct2.type_identifier());

    // Should be 88009063541924297814935272469493149666:
    println!("{:?}", iconstruct2.library_identifier());

    let iui_element = &ui_element as &dyn IUIElement;

    // Should be 299721098913151508281319992374550808386 / 41377672462483076378035019486899738808:
    println!("{:?}", iui_element.type_identifier());

    // Should be 41377672462483076378035019486899738808:
    println!("{:?}", iui_element.library_identifier());

    println!("");
}

fn main()
{
    // Initialize the top layer of the Rust Development Hierarchy.
    rdh_extension_example::init();

    // Run, Tests, run!
    test_concrete_identifiers();
    test_concrete_implements();
    test_concrete_is();
    test_divinator();
}
