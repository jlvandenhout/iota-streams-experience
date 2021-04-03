# Duplicate announce messages
## Issue
When an author sends duplicate announce messages, subscribers become unable to access the stream.

## Actual behavior
The author does not fail to duplicate the announce message, which causes the subscriber to fail with `More than one message found`.

## Expected behavior
Don't let the subscriber fail to read the stream. There are multiple expected behaviors here:
- Let the subscriber follow the original announce message.
- Prevent the author to resend an already existing announce message.
- Warn or fail the author when sending an already existing announce message.