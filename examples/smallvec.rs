use lay_out::*;

const MAX_ELEMENTS: usize = 1024;

fn main() {
    //Now that I'm thinking about it, It might acutally be better
    //allocate to the heap after a certain amount of elements.
    //That way 99% of users never encouter any heap allocations.
    //But they are still available for large widget trees.
    let _sm: SmallVec<usize, MAX_ELEMENTS> = SmallVec::new();
}
