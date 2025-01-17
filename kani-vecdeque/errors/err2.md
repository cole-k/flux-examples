# Change

Added signatures to `len` and `cap` to avoid an underflow in `is_full`.

Made `cap` trusted because of ZST stuff

# Error

The signature for `len` is not provable.
