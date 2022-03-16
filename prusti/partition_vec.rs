//! A [disjoint-sets/union-find] implementation of a vector partitioned in sets.
//!
//! See [`PartitionVec<T>`] for more information.
//!
//! [disjoint-sets/union-find]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
//! [`PartitionVec<T>`]: struct.PartitionVec.html

extern crate prusti_contracts;
use prusti_contracts::*;

use {
    std::{
        cmp::Ordering,
    },
};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "proptest")]
use proptest::prelude::*;

/// Inline definition from metadata.rs
#[derive(Copy, Clone, PartialEq)]
pub struct Metadata {
    /// The parent of the value in its sets tree.
    /// These form an upside down tree where each child has the index of its parent.
    parent: usize,
    /// A link to another index.
    /// These form a circular linked list in its subset.
    link: usize,
    /// A maximum to the size of the tree of the set.
    rank: usize,
}

impl Metadata {
    /// Create a new `Metadata` for an element with the given index.
    #[trusted]
    #[ensures(result.parent() == index && result.rank() == 0 && result.link == index)]
    pub(crate) fn new(index: usize) -> Self {
        Self {
            parent: index,
            link: index,
            rank: 0,
        }
    }

    /// Return the `parent` variable.
    #[pure]
    pub(crate) fn parent(&self) -> usize {
        self.parent
    }

    /// Set the `parent` variable.
    #[ensures(self.parent() == value)]
    #[ensures(self.link() == old(self.link()))]
    #[ensures(self.rank() == old(self.rank()))]
    pub(crate) fn set_parent(&mut self, value: usize) {
        self.parent = value;
    }

    /// Return the `link` variable.
    #[pure]
    pub(crate) fn link(&self) -> usize {
        self.link
    }

    /// Set the `link` variable.
    #[ensures(self.link() == value)]
    #[ensures(self.parent() == old(self.parent()))]
    #[ensures(self.rank() == old(self.rank()))]
    pub(crate) fn set_link(&mut self, value: usize) {
        self.link = value;
    }

    /// Return the `rank` variable.
    #[pure]
    pub(crate) fn rank(&self) -> usize {
        self.rank
    }

    /// Set the `rank` variable.
    #[ensures(self.rank() == value)]
    #[ensures(self.parent() == old(self.parent()))]
    #[ensures(self.link() == old(self.link()))]
    pub(crate) fn set_rank(&mut self, value: usize) {
        self.rank = value;
    }

    #[pure]
    pub fn eq(&self, other: Self) -> bool {
        self.parent == other.parent && self.link == other.link && self.rank == other.rank
    }
}

pub struct VecWrapperI32 {
    v: Vec<i32>
}

impl VecWrapperI32 {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { v: Vec::new(), }
    }

    /// A ghost function for getting the length of the vector
    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> i32 {
        self.v[index]
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()) && forall(
        |i: usize| (i < self.len() && i != index) ==>
        self.lookup(i) == old(self.lookup(i))
    ) && self.lookup(index) == value)]
    pub fn store(&mut self, index: usize, value: i32) {
        self.v[index] = value;
    }
}

pub struct VecWrapperMetadata {
    v: Vec<Metadata>
}

impl VecWrapperMetadata {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { v: Vec::new(), }
    }

    /// A ghost function for getting the length of the vector
    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> Metadata {
        self.v[index]
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()) && forall(
        |i: usize| (i < self.len() && i != index) ==>
        self.lookup(i).eq(old(self.lookup(i)))
    ) && self.lookup(index).eq(value))]
    pub fn store(&mut self, index: usize, value: Metadata) {
        self.v[index] = value;
    }
}

/// A [disjoint-sets/union-find] implementation of a vector partitioned in sets.
///
/// Most methods that are defined on a `Vec` also work on a `PartitionVec`.
/// In addition to this each element stored in the `PartitionVec` is a member of a set.
/// Initially each element has its own set but sets can be joined with the `union` method.
///
/// In addition to the normal implementation we store an additional index for each element.
/// These indices form a circular linked list of the set the element is in.
/// This allows for fast iteration of the set using the `set` method
/// and is used to speed up the performance of other methods.
///
/// This implementation chooses not to expose the `find` method and instead has a `same_set` method.
/// This is so that the representative of the set stays an implementation detail which gives
/// us more freedom to change it behind the scenes for improved performance.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate partitions;
/// #
/// # fn main() {
/// let mut partition_vec = partition_vec!['a', 'b', 'c', 'd'];
/// partition_vec.union(1, 2);
/// partition_vec.union(2, 3);
///
/// assert!(partition_vec.same_set(1, 3));
///
/// for (index, &value) in partition_vec.set(1) {
///     assert!(index >= 1);
///     assert!(index <= 3);
///     assert!(value != 'a');
/// }
/// # }
/// ```
///
/// [disjoint-sets/union-find]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
pub struct PartitionVec {
    /// Each index has a value.
    /// We store these in a separate `Vec` so we can easily dereference it to a slice.
    data: VecWrapperI32,
    /// The metadata for each value, this `Vec` will always have the same size as `values`.
    meta: VecWrapperMetadata,
}

/// Creates a [`PartitionVec`] containing the arguments.
///
/// There are tree forms of the `partition_vec!` macro:
///
/// - Create a [`PartitionVec`] containing a given list of elements all in distinct sets:
///
/// ```
/// # #[macro_use]
/// # extern crate partitions;
/// #
/// # fn main() {
/// let partition_vec = partition_vec!['a', 'b', 'c'];
///
/// assert!(partition_vec[0] == 'a');
/// assert!(partition_vec[1] == 'b');
/// assert!(partition_vec[2] == 'c');
///
/// assert!(partition_vec.is_singleton(0));
/// assert!(partition_vec.is_singleton(1));
/// assert!(partition_vec.is_singleton(2));
/// # }
/// ```
///
/// - Create a [`PartitionVec`] containing a given list of elements in the sets specified:
///
/// ```
/// # #[macro_use]
/// # extern crate partitions;
/// #
/// # fn main() {
/// let partition_vec = partition_vec![
///     'a' => 0,
///     'b' => 1,
///     'c' => 2,
///     'd' => 1,
///     'e' => 0,
/// ];
///
/// assert!(partition_vec[0] == 'a');
/// assert!(partition_vec[1] == 'b');
/// assert!(partition_vec[2] == 'c');
/// assert!(partition_vec[3] == 'd');
/// assert!(partition_vec[4] == 'e');
///
/// assert!(partition_vec.same_set(0, 4));
/// assert!(partition_vec.same_set(1, 3));
/// assert!(partition_vec.is_singleton(2));
/// # }
/// ```
///
/// You can use any identifiers that implement `Hash` and `Eq`.
/// Elements with the same set identifiers will be placed in the same set.
/// These identifiers will only be used when constructing a [`PartitionVec`]
/// and will not be stored further.
/// This means `println!("{:?}", partition_vec![3 => 'a', 1 => 'a'])` will display `[3 => 0, 1 => 0]`.
///
/// - Create a [`PartitionVec`] of distinct sets from a given element and size:
///
/// ```
/// # #[macro_use]
/// # extern crate partitions;
/// #
/// # fn main() {
/// let partition_vec = partition_vec!['a'; 3];
///
/// assert!(partition_vec[0] == 'a');
/// assert!(partition_vec[1] == 'a');
/// assert!(partition_vec[2] == 'a');
///
/// assert!(partition_vec.is_singleton(0));
/// assert!(partition_vec.is_singleton(1));
/// assert!(partition_vec.is_singleton(2));
/// # }
/// ```
///
/// [`PartitionVec`]: partition_vec/struct.PartitionVec.html
///
/*
#[macro_export]
macro_rules! partition_vec {
    ($elem: expr; $len: expr) => {
        $crate::PartitionVec::from_elem($elem, $len);
    };
    ($($elem: expr),*) => {
        {
            let len = partitions_count_expr![$($elem),*];
            let mut partition_vec = $crate::PartitionVec::with_capacity(len);

            $(
                partition_vec.push($elem);
            )*

            partition_vec
        }
    };
    ($($elem: expr,)*) => {
        partition_vec![$($elem),*];
    };
    ($($elem: expr => $set: expr),*) => {
        {
            let len = partitions_count_expr![$($elem),*];
            let mut partition_vec = $crate::PartitionVec::with_capacity(len);
            let mut map = ::std::collections::HashMap::new();

            $(
                let last_index = partition_vec.len();
                partition_vec.push($elem);

                if let Some(&index) = map.get(&$set) {
                    partition_vec.union(index, last_index);
                } else {
                    map.insert($set, last_index);
                }
            )*

            partition_vec
        }
    };
    ($($elem: expr => $set: expr,)*) => {
        partition_vec![$($elem => $set),*];
    }
}*/

impl PartitionVec {
    /// Constructs a new, empty `PartitionVec<T>`.
    ///
    /// The `PartitionVec<T>` will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use partitions::PartitionVec;
    ///
    /// let mut partition_vec: PartitionVec<()> = PartitionVec::new();
    /// ```
    #[inline]
    #[ensures(result.data.len() == 0 && result.meta.len() == 0)]
    pub fn new() -> Self {
        Self {
            data: VecWrapperI32::new(),
            meta: VecWrapperMetadata::new(),
        }
    }

    /// Joins the sets of the `first_index` and the `second_index`.
    ///
    /// This method will be executed in `O(α(n))` time where `α` is the inverse
    /// Ackermann function. The inverse Ackermann function has value below 5
    /// for any value of `n` that can be written in the physical universe.
    ///
    /// # Panics
    ///
    /// If `first_index` or `second_index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// #
    /// # fn main() {
    /// let mut partition_vec = partition_vec![(); 4];
    ///
    /// // All elements start out in their own sets.
    /// assert!(partition_vec.len_of_set(0) == 1);
    /// assert!(partition_vec.len_of_set(1) == 1);
    /// assert!(partition_vec.len_of_set(2) == 1);
    /// assert!(partition_vec.len_of_set(3) == 1);
    ///
    /// partition_vec.union(1, 2);
    ///
    /// // Now 1 and 2 share a set.
    /// assert!(partition_vec.len_of_set(0) == 1);
    /// assert!(partition_vec.len_of_set(1) == 2);
    /// assert!(partition_vec.len_of_set(2) == 2);
    /// assert!(partition_vec.len_of_set(3) == 1);
    ///
    /// partition_vec.union(2, 3);
    ///
    /// // We added 3 to the existing set with 1 and 2.
    /// assert!(partition_vec.len_of_set(0) == 1);
    /// assert!(partition_vec.len_of_set(1) == 3);
    /// assert!(partition_vec.len_of_set(2) == 3);
    /// assert!(partition_vec.len_of_set(3) == 3);
    /// # }
    /// ```
    #[requires(first_index < self.meta.len())]
    #[requires(second_index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    #[ensures(self.data.len() == self.meta.len())]
    #[ensures(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[ensures(self.data.len() == self.meta.len())]
    pub fn union(&mut self, first_index: usize, second_index: usize) {
        let i = self.find(first_index);
        let j = self.find(second_index);

        if i == j {
            return
        }

        // We swap the values of the links.
        let link_i = self.meta.lookup(i).link();
        let link_j = self.meta.lookup(j).link();
        self.meta.lookup(i).set_link(link_j);
        self.meta.lookup(j).set_link(link_i);

        // We add to the tree with the highest rank.
        match Ord::cmp(&self.meta.lookup(i).rank(), &self.meta.lookup(j).rank()) {
            Ordering::Less => {
                self.meta.lookup(i).set_parent(j);
            },
            Ordering::Equal => {
                // We add the first tree to the second tree.
                self.meta.lookup(i).set_parent(j);
                // The second tree becomes larger.
                self.meta.lookup(j).set_rank(self.meta.lookup(j).rank() + 1);
            },
            Ordering::Greater => {
                self.meta.lookup(j).set_parent(i);
            },
        }
    }

    /// Returns `true` if `first_index` and `second_index` are in the same set.
    ///
    /// This method will be executed in `O(α(n))` time where `α` is the inverse
    /// Ackermann function.
    ///
    /// # Panics
    ///
    /// If `first_index` or `second_index` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// # fn main() {
    /// let mut partition_vec = partition_vec![(); 4];
    ///
    /// partition_vec.union(1, 3);
    /// partition_vec.union(0, 1);
    ///
    /// assert!(partition_vec.same_set(0, 1));
    /// assert!(!partition_vec.same_set(0, 2));
    /// assert!(partition_vec.same_set(0, 3));
    /// assert!(!partition_vec.same_set(1, 2));
    /// assert!(partition_vec.same_set(1, 3));
    /// assert!(!partition_vec.same_set(2, 3));
    /// # }
    /// ```
    #[inline]
    #[requires(first_index < self.meta.len())]
    #[requires(second_index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    pub fn same_set(&self, first_index: usize, second_index: usize) -> bool {
        self.find(first_index) == self.find(second_index)
    }

    /// Returns `true` if `first_index` and `second_index` are in different sets.
    ///
    /// This method will be executed in `O(α(n))` time where `α` is the inverse
    /// Ackermann function.
    ///
    /// # Panics
    ///
    /// If `first_index` or `second_index` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// # fn main() {
    /// let mut partition_vec = partition_vec![(); 4];
    ///
    /// partition_vec.union(1, 3);
    /// partition_vec.union(0, 1);
    ///
    /// assert!(!partition_vec.other_sets(0, 1));
    /// assert!(partition_vec.other_sets(0, 2));
    /// assert!(!partition_vec.other_sets(0, 3));
    /// assert!(partition_vec.other_sets(1, 2));
    /// assert!(!partition_vec.other_sets(1, 3));
    /// assert!(partition_vec.other_sets(2, 3));
    /// # }
    /// ```
    #[inline]
    #[requires(first_index < self.meta.len())]
    #[requires(second_index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    pub fn other_sets(&self, first_index: usize, second_index: usize) -> bool {
        self.find(first_index) != self.find(second_index)
    }

    /// Will remove `index` from its set while leaving the other members in it.
    ///
    /// After this `index` will be the only element of its set.
    /// This won't change the `PartitionVec<T>` if `index` is already the only element.
    /// This method will be executed in `O(m)` time where `m` is the size of the set of `index`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// #
    /// # fn main() {
    /// let mut partition_vec = partition_vec![
    ///     () => 'a',
    ///     () => 'a',
    ///     () => 'a',
    ///     () => 'b',
    /// ];
    ///
    /// // 0, 1, and 2 share a set.
    /// assert!(partition_vec.len_of_set(0) == 3);
    /// assert!(partition_vec.len_of_set(1) == 3);
    /// assert!(partition_vec.len_of_set(2) == 3);
    /// assert!(partition_vec.len_of_set(3) == 1);
    ///
    /// partition_vec.make_singleton(2);
    ///
    /// // Now 2 has its own set and 1, and 2 still share a set.
    /// assert!(partition_vec.len_of_set(0) == 2);
    /// assert!(partition_vec.len_of_set(1) == 2);
    /// assert!(partition_vec.len_of_set(2) == 1);
    /// assert!(partition_vec.len_of_set(3) == 1);
    /// # }
    /// ```
    #[requires(index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    #[trusted]
    pub fn make_singleton(&mut self, index: usize) {
        let mut current = self.meta.lookup(index).link();

        if current != index {
            // We make this the new root.
            let root = current;
            self.meta.lookup(root).set_rank(1);

            // Change to use local variable as workaround based on
            // https://github.com/viperproject/prusti-dev/issues/786
            let mut current_meta = self.meta.lookup(current);

            // All parents except for the last are updated.
            while current_meta.link() != index {
                current_meta.set_parent(root);

                current_meta = self.meta.lookup(current_meta.link());
            }

            // We change the last parent and link.
            current_meta.set_parent(root);
            current_meta.set_link(root);
        }

        self.meta.store(index, Metadata::new(index));
    }

    /// Returns `true` if `index` is the only element of its set.
    ///
    /// This will be done in `O(1)` time.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// #
    /// # fn main() {
    /// let mut partition_vec = partition_vec![(); 4];
    ///
    /// partition_vec.union(1, 3);
    ///
    /// assert!(partition_vec.is_singleton(0));
    /// assert!(!partition_vec.is_singleton(1));
    /// assert!(partition_vec.is_singleton(2));
    /// assert!(!partition_vec.is_singleton(3));
    /// # }
    /// ```
    #[inline]
    #[requires(index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    pub fn is_singleton(&self, index: usize) -> bool {
        self.meta.lookup(index).link() == index
    }

    /// Returns the amount of elements in the set that `index` belongs to.
    ///
    /// This will be done in `O(m)` time where `m` is the size of the set that `index` belongs to.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate partitions;
    /// #
    /// # fn main() {
    /// let mut partition_vec = partition_vec![true; 3];
    ///
    /// assert!(partition_vec.len_of_set(0) == 1);
    /// assert!(partition_vec.len_of_set(1) == 1);
    /// assert!(partition_vec.len_of_set(2) == 1);
    ///
    /// partition_vec.union(0, 2);
    ///
    /// assert!(partition_vec.len_of_set(0) == 2);
    /// assert!(partition_vec.len_of_set(1) == 1);
    /// assert!(partition_vec.len_of_set(2) == 2);
    /// # }
    /// ```
    /// #[requires(first_index < self.meta.len())]
    #[requires(index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    pub fn len_of_set(&self, index: usize) -> usize {
        let mut current = self.meta.lookup(index).link();
        let mut count = 1;

        while current != index {
            body_invariant!(self.data.len() == old(self.data.len()) && self.meta.len() == old(self.meta.len()));
            body_invariant!(current < self.meta.len());

            current = self.meta.lookup(current).link();
            count += 1;
        }

        count
    }

    /// Gives the representative of the set that `index` belongs to.
    ///
    /// This method will be executed in `O(α(n))` time where `α` is the inverse
    /// Ackermann function. Each index of a set
    /// will give the same value. To see if two indexes point to values in
    /// the same subset compare the results of `find`.
    ///
    /// This method is private to keep the representative of the set an implementation
    /// detail, this gives greater freedom to change the representative of the set.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    #[requires(index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    #[ensures(result < self.meta.len())]
    pub(crate) fn find(&self, index: usize) -> usize {
        // If the node is its own parent we have found the root.
        if self.meta.lookup(index).parent() == index {
            index
        } else {
            // This method is recursive so each parent on the way to the root is updated.
            let root = self.find(self.meta.lookup(index).parent());

            // We update the parent to the root for a lower tree.
            self.meta.lookup(index).set_parent(root);

            root
        }
    }

    /// Gives the representative of the set that `index` belongs to.
    ///
    /// This method is slightly faster than `find` but still `O(a(n))` time.
    /// This method wont update the parents while finding the representative and should
    /// only be used if the parents will be updated immediately afterwards.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    #[inline]
    #[requires(index < self.meta.len())]
    #[requires(forall(|x: usize| x < self.meta.len() ==> self.meta.lookup(x).parent < self.meta.len() && self.meta.lookup(x).link < self.meta.len()))]
    #[requires(self.data.len() == self.meta.len())]
    #[ensures(result < self.meta.len())]
    pub(crate) fn find_final(&self, mut index: usize) -> usize {
        while index != self.meta.lookup(index).parent() {
            body_invariant!(self.data.len() == old(self.data.len()) && self.meta.len() == old(self.meta.len()));
            body_invariant!(index < self.meta.len());

            index = self.meta.lookup(index).parent();
        }

        index
    }
}

pub fn main() {

}