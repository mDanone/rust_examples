mod enums;
mod vectors;
mod utf_strings;
mod control_flow;

use enums::enums::test_enums;
use vectors::vectors::test_vectors;
use utf_strings::utf_strings::test_strings_in_utf;
use control_flow::control_flow::test_control_flow;


fn test_if_let(){
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}


fn main() {
    test_vectors();
    test_strings_in_utf();
    test_enums();
    test_control_flow();
}
