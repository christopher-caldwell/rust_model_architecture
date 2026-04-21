use nanoid::nanoid;

// This is a standalone incase I want to change the alphabaet or length in one place
pub fn gen_ident() -> String {
    // Default: 21 chars, URL-safe alphabet
    // let id = nanoid!();

    // Custom length
    // let custom_id = nanoid!(10);

    // // Custom alphabet (similar to your requirement)
    // let alphabet: [char; 12] = ['a', 'b', 'c', '1', '2', '3', '!', '@', '#', '$', '%', '^'];
    nanoid!(10)
}
