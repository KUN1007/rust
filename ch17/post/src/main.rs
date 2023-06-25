use post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("KUN IS CUTEST!");

    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("KUN IS CUTEST!", post.content());
}
