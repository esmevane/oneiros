`oneiros remote add <name> --ticket <uri>`

Register a remote host by providing its project-scoped ticket URI.

```bash
# On the remote host, issue a ticket:
oneiros ticket issue --permission read,write --project my-project

# On the local host, add the remote:
oneiros remote add dreamforge --ticket oneiros://dreamforge/link:...
```
