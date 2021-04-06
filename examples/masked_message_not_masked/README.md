# Masked message not masked
## Issue
When an author sends masked messages, they are not encrypted.

## Actual behavior
When the author sends a masked message, the contents do not seem to be encrypted on the tangle. This can be verified by searching for the message index and examining the data content.

## Expected behavior
The data in the masked message is expected to be encrypted, even before any keyload.