# Public payload not accessible after keyload
## Issue
When an author sends a public payload after a keyload, subscribers who did not receive the keyload can't access the rest of the stream anymore. So what is the use of the public payload anyway?

## Actual behavior
Messages after a keyload are inaccessible to users not receiving the keyload.

## Expected behavior
Messages are expected to be accessible after a keyload, but masked payload can only be read by subscribers who received the keyload.