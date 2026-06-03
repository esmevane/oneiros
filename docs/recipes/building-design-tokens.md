# Building the tokens in @oneiros/tokens

The setup in `@oneiros/tokens` is managed by a rust crate named `tangible`, which turns config files like the `token.json` into a stylesheet of design tokens. That means that if that file changes, and you need to update it, you'll need the crate:

```sh
cargo install tangible
```

Afterwards, just run tangible to generate a new `token.css`!

## Why isn't this automated?

It should be! It just isn't - yet. To come?
