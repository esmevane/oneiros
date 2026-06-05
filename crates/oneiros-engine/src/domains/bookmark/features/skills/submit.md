`oneiros bookmark submit --ticket <uri> <name>`

Push a bookmark to a peer who has issued a submit-scoped ticket.

```bash
# Receiver issues a submit ticket
oneiros ticket issue --permission write --actor alice --project my-project

# Sender creates and shares the bookmark
oneiros bookmark create my-change
oneiros bookmark share my-change

# Sender submits to the receiver
oneiros bookmark submit --ticket <receiver-ticket-uri> my-change
```
