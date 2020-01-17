use std::any::{Any, TypeId};
use std::fmt;

use fxhash::FxHashMap;

#[derive(Default)]
/// The typemap container
pub struct TypeMap {
    map: Option<FxHashMap<TypeId, Box<dyn Any>>>,
}

impl TypeMap {
    /// Create an empty `TypeMap`.
    #[inline]
    pub fn new() -> Self {
        Self { map: None }
    }

    /// Insert a value into this `TypeMap`.
    ///
    /// If a value of this type already exists, it will be returned.
    pub fn insert<T: 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .get_or_insert_with(|| FxHashMap::default())
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| (boxed as Box<dyn Any>).downcast().ok().map(|boxed| *boxed))

    }

    /// Check if container contains value for type
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.as_ref().and_then(|m| m.get(&TypeId::of::<T>())).is_some()
    }

    /// Get a reference to a value previously inserted on this `TypeMap`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|m| m.get(&TypeId::of::<T>()))
            .and_then(|boxed| (&**boxed as &(dyn Any)).downcast_ref())
    }

    /// Get a mutable reference to a value previously inserted on this `TypeMap`.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|m| m.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| (&mut **boxed as &mut (dyn Any)).downcast_mut())
    }

    /// Remove a value from this `TypeMap`.
    ///
    /// If a value of this type exists, it will be returned.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map
            .as_mut()
            .and_then(|m| m.remove(&TypeId::of::<T>()))
            .and_then(|boxed| (boxed as Box<dyn Any>).downcast().ok().map(|boxed| *boxed))
    }

    /// Clear the `TypeMap` of all inserted values.
    #[inline]
    pub fn clear(&mut self) {
        self.map = None;
    }
}

impl fmt::Debug for TypeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TypeMap").finish()
    }
}

/// Provides the same `TypeMap` container, but with `Send` + `Sync` bounds on values
pub mod concurrent {

    use std::any::{Any, TypeId};
    use std::fmt;

    use fxhash::FxHashMap;

    #[derive(Default)]
    /// The typemap container
    pub struct TypeMap {
        map: Option<FxHashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    }

    impl TypeMap {
        /// Create an empty `TypeMap`.
        #[inline]
        pub fn new() -> Self {
            Self { map: None }
        }

        /// Insert a value into this `TypeMap`.
        ///
        /// If a value of this type already exists, it will be returned.
        pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
            self.map
                .get_or_insert_with(|| FxHashMap::default())
                .insert(TypeId::of::<T>(), Box::new(val))
                .and_then(|boxed| (boxed as Box<dyn Any>).downcast().ok().map(|boxed| *boxed))

        }

        /// Check if container contains value for type
        pub fn contains<T: 'static>(&self) -> bool {
            self.map.as_ref().and_then(|m| m.get(&TypeId::of::<T>())).is_some()
        }

        /// Get a reference to a value previously inserted on this `TypeMap`.
        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|m| m.get(&TypeId::of::<T>()))
                .and_then(|boxed| (&**boxed as &(dyn Any)).downcast_ref())
        }

        /// Get a mutable reference to a value previously inserted on this `TypeMap`.
        pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|m| m.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (&mut **boxed as &mut (dyn Any)).downcast_mut())
        }

        /// Remove a value from this `TypeMap`.
        ///
        /// If a value of this type exists, it will be returned.
        pub fn remove<T: 'static>(&mut self) -> Option<T> {
            self.map
                .as_mut()
                .and_then(|m| m.remove(&TypeId::of::<T>()))
                .and_then(|boxed| (boxed as Box<dyn Any>).downcast().ok().map(|boxed| *boxed))
        }

        /// Clear the `TypeMap` of all inserted values.
        #[inline]
        pub fn clear(&mut self) {
            self.map = None;
        }
    }

    impl fmt::Debug for TypeMap {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("TypeMap").finish()
        }
    }
}

#[test]
fn test_type_map() {
    #[derive(Debug, PartialEq)]
    struct MyType(i32);

    let mut map = TypeMap::new();

    map.insert(5i32);
    map.insert(MyType(10));

    assert_eq!(map.get(), Some(&5i32));
    assert_eq!(map.get_mut(), Some(&mut 5i32));

    assert_eq!(map.remove::<i32>(), Some(5i32));
    assert!(map.get::<i32>().is_none());

    assert_eq!(map.get::<bool>(), None);
    assert_eq!(map.get(), Some(&MyType(10)));
}
