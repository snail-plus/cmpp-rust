
struct Str<'a> {
    content: &'a str
}

impl<'a> Str<'a> {
    fn get_content(&self) -> &'a str {
        self.content
    }

}
fn main() {

    let s = Str {
        content: "string_slice"
    };
    println!("s.content = {}", s.get_content());
}