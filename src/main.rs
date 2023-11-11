mod enums;
mod collections;
mod control_flow;
mod scopes;
mod exceptions;

use enums::enums::test_enums;
use collections::vectors::test_vectors;
use collections::utf_strings::test_strings_in_utf;
use collections::tasks::task_tests;
use control_flow::control_flow::test_control_flow;
use collections::hash_maps::test_maps;
use exceptions::exceptions::exception_handler;


fn main() {
    test_vectors();
    test_strings_in_utf();
    test_enums();
    test_control_flow();
    test_maps();
    task_tests();
    exception_handler();
}
