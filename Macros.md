I'm at a loss what I should do with the layout code.
I cannot seem to write it in a way that would be ergonomic.

The real question is if the types can be flattened out using a macro for example

```rust
v!(rect(), text(), v!(image(), svg()))
//Flatten into nodes
Node {}, Node {}, Node { Node {}, Node {} }
```

