use std::thread;

fn huge_tracts_of_land() {
    let builder = thread::Builder::new()
                  .name("swamp castle".into())
                  .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(|| {
        let bfa: [i32; 2_097_152] = [3i32; 2_097_152];
        let count: i32 = bfa[1_999_999];
        println!("First shalt thou take out the Holy Pin, then shalt thou count to {three}, no more, no less.\n{three} shall be the number thou shalt count, and the number of the counting shall be {three}.\n4 shalt thou not count, neither count thou 2, excepting that thou then proceed to {three}.\n5 is right out.\nOnce the number {three}, being the third number, be reached, then lobbest thou thy Holy Hand Grenade of Antioch towards thy foe, who being naughty in My sight, shall snuff it.", three=count);
    }).unwrap();

    handler.join().unwrap();
}

fn main() {
    println!("the narwhal bacons at midnight");
    huge_tracts_of_land();
}
