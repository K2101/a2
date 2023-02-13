use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn rand(length: usize) -> String {
    let chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '*', '(', '_', '&', '+', '!', '@', '-', '$', ')',
        '<', '/', ':', '.', '%', '>', '}', '^', '?', '[', '#', ']', '=', '~', '{', '|',
    ];

    let mut rng = thread_rng();
    let mut rand_str = String::with_capacity(length);

    for _ in 0..length {
        rand_str.push(
            *chars
                .choose(&mut rng)
                .expect("sampling array should have values"),
        )
    }

    rand_str
}
