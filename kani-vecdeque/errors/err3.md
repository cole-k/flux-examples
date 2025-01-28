# Change

Added a signature to `handle_capacity_increase`.

# Error

Various locations don't meet the precondition for this function. The fix is to
add more signatures (and some trusted signatures because we can't verify some
arithmetic operations easily).

No err3.out saved for this change because it spanned a few commits. Try d7fa1a9
for the error.
