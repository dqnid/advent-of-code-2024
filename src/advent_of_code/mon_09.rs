use super::{utils::read_disk_map_input, DiskMap};

pub fn disk_fragmenter(input: &str) -> usize {
    let disk_map = read_disk_map_input(input);

    let organized_disk_map = organize_disk_map(&disk_map);
    println!("{:?}", disk_map);
    println!("{:?}", organized_disk_map);

    0
}

pub fn organize_disk_map(disk_map: &DiskMap) -> DiskMap {
    let organized_disk_map: DiskMap = vec![];

    for block in disk_map {}

    organized_disk_map
}
