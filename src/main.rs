extern crate nix;

use nix::sys::resource::{
    Resource,
    getrlimit,
    setrlimit
};

fn huge_tracts_of_land() {
    println!("Approached the castle");
    let bfa: [i32; 2_097_152] = [3i32; 2_097_152];
    let count: i32 = bfa[1_999_999];
    println!("First shalt thou take out the Holy Pin, then shalt thou count to {three}, no more, no less.\n{three} shall be the number thou shalt count, and the number of the counting shall be {three}.\n4 shalt thou not count, neither count thou 2, excepting that thou then proceed to {three}.\n5 is right out.\nOnce the number {three}, being the third number, be reached, then lobbest thou thy Holy Hand Grenade of Antioch towards thy foe, who being naughty in My sight, shall snuff it.", three=count);
}

fn main() {
    println!("the narwhal bacons at midnight");
    let mut limit  = getrlimit(Resource::RLIMIT_STACK).unwrap();
    println!("Old value: {}", limit.rlim_cur);
    limit.rlim_cur = 32_768_000;
    setrlimit(Resource::RLIMIT_STACK, limit).unwrap();
    let newlimit  = getrlimit(Resource::RLIMIT_STACK).unwrap();
    println!("New value: {}", newlimit.rlim_cur);
    huge_tracts_of_land();
}
