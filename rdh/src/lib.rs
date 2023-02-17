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
//! lib.rs (rdh)
//!
//! Contains module declarations for and initialization of the Rust Development Hierarchy (RDH)
//! core library.
//!

///////////////////////////////////////////////////////////////////////////////////////////////////
// Public modules
///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod _hierarchy;
pub mod _infrastructure;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Public initialization functions
///////////////////////////////////////////////////////////////////////////////////////////////////

///
/// init: Initializes the core RDH library.
///
/// Project main() function must invoke the init() function for the top-level RDH library.
/// 
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

    // Register framework types defined within the core library.
    TypeRegistry::register_type::<dyn IConstruct>(ICONSTRUCT_TYPE_UUID, RDH_LIBRARY_UUID);
    TypeRegistry::register_type::<Construct>(CONSTRUCT_TYPE_UUID, RDH_LIBRARY_UUID);
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Private initialization infrastructure
///////////////////////////////////////////////////////////////////////////////////////////////////

// *** Private constants ***

//
// RDH_LIBRARY_UUID: The unique identifier for the core RDH library.
//
const RDH_LIBRARY_UUID: u128 = 88009063541924297814935272469493149666;

// *** Minutiae ***

use crate::_hierarchy::construct::{ Construct, CONSTRUCT_TYPE_UUID, IConstruct, ICONSTRUCT_TYPE_UUID };
use crate::_infrastructure::thaumaturgy::TypeRegistry;
