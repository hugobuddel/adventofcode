// All warnings enabled, to learn what they do.
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    clashing_extern_declarations,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unreachable_pub,
    unsafe_code,
    // unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
)]

//! Advent of Code 2020 Day 2
// --- Day 2: Password Philosophy ---
//
// Your flight departs in a few days from the coastal airport; the easiest way
// down to the coast from here is via toboggan.
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
// "Something's wrong with our computers; we can't log in!" You ask if you can
// take a look.
//
// Their password database seems to be a little corrupted: some of the
// passwords wouldn't have been allowed by the Official Toboggan Corporate
// Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of
// passwords (according to the corrupted database) and the corporate policy
// when that password was set.
//
// For example, suppose you have the following list:
//
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// Each line gives the password policy and then the password. The password
// policy indicates the lowest and highest number of times a given letter must
// appear for the password to be valid. For example, 1-3 a means that the
// password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is
// not; it contains no instances of b, but needs at least 1. The first and
// third passwords are valid: they contain one a or nine c, both within the
// limits of their respective policies.
//
// How many passwords are valid according to their policies?


fn main() {
    println!("Advent of Code 2020 Day 2");

    let entries1 = vec![
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc",
    ];
    println!("{:?}", entries1);
    let entries2 = entries1.iter().map(
        |x| x.split(" ").collect::<Vec<&str>>()
    ).collect::<Vec<_>>();
    println!("{:?}", entries2);
    let entries3 = entries2.iter().map(
        |a| a
    ).collect::<Vec<_>>();
    println!("{:?}", entries3);
}
