use oop::rust_style::Post;

fn main() {
    state_pattern_post();
    rust_style_post();
}

fn rust_style_post() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());
}

fn state_pattern_post() {
    let mut post = oop::Post::new();
    post.add_text("Ahmed Bin Nasser");
    println!("{}", post.content());
    post.request_review();
    post.approve();
    println!("{}", post.content());
}
