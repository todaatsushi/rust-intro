mod binary_heap;
mod hash_map;
mod hash_sets;
mod vecs;

pub fn talk_about_collections() {
    println!("\nAbout collections and those data types");
    vecs::about_vecs();
    hash_map::about_hashmaps();
    hash_sets::about_hashsets();
    binary_heap::about_binary_heaps();
}
