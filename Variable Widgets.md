```rs
let files = winwalk("../");

let mut file_ui: Vec<Text> = files.into_iter().map(|file| text(file.path.into())).collect();

loop {
    //How will the layout library handle vectors and arrays?
    //Each Text widget will need to have a node with a size.
    //Or maybe widgets are nodes???
    //Ehhhhh? Each one will probably want to be able to specify padding/gap, those types of things.
    v!(&mut file_ui)
}
```
