use std::fmt::Debug;
use std::iter;

#[cfg(test)]
mod tests;

/// A result of a `contains_at_least_ordered` ordered, skipping requirement.
#[must_use]
pub enum ContainsAtLeastOrdered<T> {
    Yes,
    No { not_found: Vec<T> },
}

impl<T: Debug> ContainsAtLeastOrdered<T> {
    /// Assert the outcome. Prints a Debug message if the assertion fails.
    // TBD: a better name;  `assert_result` implies it asserts a `Result` type, `assert_debug`
    // variations imply they may only work when the compiler is in debug mode (instead of release mode),
    // like the `debug_assert!` macro.
    pub fn assert_this(&self) {
        match self {
            Self::Yes => assert!(true),
            Self::No {
                not_found: remaining_elements,
            } => {
                assert!(false, "Missing elements: {:?}", remaining_elements)
            }
        }
    }

    /// Assert that the outcome is not valid.
    pub fn assert_invalid(&self) {
        match self {
            Self::Yes => assert!(
                false,
                "Expected assertion to be rejected, but it was accepted. Asserted on type {:?}.",
                core::any::type_name::<ContainsAtLeastOrdered<T>>()
            ),
            Self::No {
                not_found: remaining_elements,
            } => {
                assert!(true, "Missing elements: {:?}", remaining_elements)
            }
        }
    }
}

impl<T> ContainsAtLeastOrdered<T> {
    /// Return the remainder of the elements which were expected, but were not detected in order.
    /// If all elements were found, returns `None`. Otherwise returns `Some(&[T])`.
    pub fn elements_not_found(&self) -> Option<&[T]> {
        match self {
            Self::Yes => None,
            Self::No { not_found } => Some(not_found.as_slice()),
        }
    }
}

impl<T> From<ContainsAtLeastOrdered<T>> for bool {
    fn from(item: ContainsAtLeastOrdered<T>) -> Self {
        matches!(item, ContainsAtLeastOrdered::Yes)
    }
}

/// Test whether the `expected` elements are found in `iter`, and require them to be found
/// in the same order as defined by `expected`.
pub fn contains_at_least_ordered<I1, I2, T>(iter: I1, expected: I2) -> ContainsAtLeastOrdered<T>
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    T: PartialEq,
{
    let mut expected = expected.into_iter().peekable();

    // If the `expected` set contains no elements, i.e. is the empty set, by definition all elements
    // are in `iter`.
    if expected.peek().is_none() {
        return ContainsAtLeastOrdered::Yes;
    }

    // We can unwrap here since `peek.is_none()` must be `false`.
    let mut expected_next = expected.next().unwrap();

    // Would use a combinator here if we could short circuit; and not allocate unnecessarily
    for elem in iter {
        // Case 1:
        //   If the next element is expected, and there are more elements in the expected set, continue
        if elem == expected_next && expected.peek().is_some() {
            // Unwrap safety: peek().is_some() must be true here
            expected_next = expected.next().unwrap();
            continue;
        }

        // Case 2:
        //   If the next element is expected, and the expected set has been fulfilled, return true.
        if elem == expected_next && expected.peek().is_none() {
            return ContainsAtLeastOrdered::Yes;
        }

        // Case 3:
        //   If the next element is not an expected element, continue
    }

    // We iterated all elements, but our expected set is not empty
    let current = iter::once(expected_next);

    ContainsAtLeastOrdered::No {
        not_found: current.chain(expected).collect(),
    }
}
