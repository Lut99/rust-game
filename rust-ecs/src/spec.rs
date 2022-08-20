//  SPEC.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 10:32:55
//  Last edited:
//    06 Aug 2022, 16:20:39
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the interfaces to the library: common types, structs, etc.
// 

use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};


/***** CUSTOM TYPES *****/
/// Defines the type used for all entitites.
#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity(u64);

impl Hash for Entity {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<u64> for Entity {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Entity> for u64 {
    #[inline]
    fn from(value: Entity) -> Self {
        value.0
    }
}

/// Defines the base Component trait.
pub trait Component {}



/// Defines a type-agnostic base for a ComponentList.
pub trait ComponentListBase {
    /// Allows the ComponentListBase to be downcasted.
    fn as_any(&self) -> &dyn Any;

    /// Allows the ComponentListBase to be (muteable) downcasted.
    fn as_any_mut(&mut self) -> &mut dyn Any;



    /// Returns the identifier for this specific generic type
    fn id(&self) -> TypeId;

    /// Returns the name of the ComponentList's type.
    fn type_name(&self) -> &'static str;



    /// Get the index from an entity.  
    /// This is useful to iterate more easily through the list.
    /// 
    /// **Arguments**
    ///  * `entity`: The entity to get the index of.
    /// 
    /// **Returns**  
    /// The index of the given entity in the list if the entity has a component in this list, or None otherwise.
    fn get_index(&self, entity: Entity) -> Option<usize>;

    /// Get the entity from an index.  
    /// This is useful when iterating through the list.
    /// 
    /// **Arguments**
    ///  * `index`: The index to match an entity with.
    /// 
    /// **Returns**  
    /// The entity that resides at the given index if the index is in range, or None otherwise.
    fn get_entity(&self, index: usize) -> Option<Entity>;



    /// Deletes the given entity if it existed from the internal list.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to remove the data of.
    fn delete(&mut self, entity: Entity);
}
