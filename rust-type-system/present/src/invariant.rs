mod external {
    use std::io::Error;
    struct Item;

    /// Fetches the data from server returning a vector of items
    /// with the given length when successful.
    fn get_items(items_count: usize) -> Result<Vec<Item>, Error> {
        // ... //
    }

    fn example() -> Result<(), Error> {
        let request_count = 10;
        let mut items = get_items(request_count)?;
        // The type system can't guarantee that the count of return
        // vector is 10 items.
        // An empty list of a vector of thousands of items won't
        // violate the type system contract.

        let return_count = items.len();
        if return_count > request_count {
            log::warn!(
                "Got more items than requested. Requested: \
                {request_count}, Returned: {return_count}"
            );
            items.truncate(return_count);
        }

        // Send items to micro-controller... //

        Ok(())
    }
}

mod internal {

    /// Performs a binary search on the provided sorted slice.
    ///
    /// Note:
    /// The provided list must be sorted.
    fn binary_search<T>(sorted_items: &[T], target: T)
    where
        T: Ord,
    {
        // This check will be executed in debug build only.
        debug_assert!(sorted_items.is_sorted());

        // Another way to do the checks, which can be useful
        // when the check includes multiple steps.
        if cfg!(debug_assertions) {
            assert!(sorted_items.is_sorted());
        }

        // ... //
    }
}
