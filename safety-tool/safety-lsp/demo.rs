// Not defined tags:
#[safety { Tag: "Single tag." }]
#[safety { Tag1: "Some comments for Tag 1:"; Tag2, Alias(pointer): "Comments for Tag 2 and Alias:" }]
#[safety { Align(ptr, T), Owning(p) }]
fn main() {
    let a = 1;
}
