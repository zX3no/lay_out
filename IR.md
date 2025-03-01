```rs
//Not sure how I'm going to implement nested macros.
flex!(text(), text(), v!(text(), h!(text(), text())))
flex!(text(), text(), flex!(text(), flex!(text(), text())))

Flex {
    Text
    Text
    Vertical {
        Text
        Horizontal {
            Text
            Text
        }
    }
}
```
