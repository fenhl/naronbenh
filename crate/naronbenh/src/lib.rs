use {
    std::collections::HashSet,
    chrono::prelude::*,
    image::{
        GrayImage,
        Luma,
    },
    rayon::prelude::*,
};

const UPPER: [[isize; 3]; 16] = [
    [4352, 68, -4096],
    [4360, 68, -4096],
    [4352, 68, -4090],
    [4360, 68, -4090],
    [4368, 68, -4240],
    [4374, 68, -4240],
    [4368, 68, -4232],
    [4374, 68, -4232],
    [4352, 64, -4096],
    [4360, 64, -4096],
    [4352, 64, -4090],
    [4360, 64, -4090],
    [4368, 64, -4240],
    [4374, 64, -4240],
    [4368, 64, -4232],
    [4374, 64, -4232],
];

const LOWER: [[isize; 3]; 8] = [
    [4352, 35, -4096],
    [4360, 35, -4096],
    [4352, 35, -4090],
    [4360, 35, -4090],
    [4368, 35, -4240],
    [4374, 35, -4240],
    [4368, 35, -4232],
    [4374, 35, -4232],
];

fn coords() -> impl Iterator<Item = [isize; 3]> {
    UPPER.into_iter().chain(LOWER)
}

pub fn is_in_building(x: i16, y: i16, z: i16) -> bool {
    let x = isize::from(x);
    let y = isize::from(y);
    let z = isize::from(z);
    for [cx, cy, cz] in coords() {
        if (((x - cx).pow(2) + (y - cy).pow(2) + (z - cz).pow(2)) as f64).sqrt() > 128.0 {
            return false
        }
    }
    for [cx, cy, cz] in UPPER {
        if (((x - cx).pow(2) + (y - cy).pow(2) + (z - cz).pow(2)) as f64).sqrt() < 24.0 {
            return false
        }
    }
    true
}

fn is_in_perimeter_impl(is_in_building: impl Fn(i16, i16) -> bool, x: i16, z: i16) -> bool {
    for dx in x - 128..x + 128 {
        for dz in z - 128..z + 128 {
            if ((isize::from(x - dx).pow(2) + isize::from(z - dz).pow(2)) as f64).sqrt() <= 128.0 && is_in_building(dx, dz) {
                return true
            }
        }
    }
    false
}

fn is_in_perimeter_of(main_building: &HashSet<[i16; 2]>, x: i16, z: i16) -> bool {
    is_in_perimeter_impl(|cx, cz| main_building.contains(&[cx, cz]), x, z)
}

pub fn is_in_perimeter(x: i16, z: i16) -> bool {
    is_in_perimeter_impl(|cx, cz| (-36..140).any(|y| is_in_building(cx, y, cz)), x, z)
}

pub fn building_image(y: i16) -> GrayImage {
    let min_x = 4200;
    let max_x = 4500;
    let min_z = -4300;
    let max_z = -4000;
    GrayImage::from_fn(
        u32::try_from(max_x - min_x).unwrap(),
        u32::try_from(max_z - min_z).unwrap(),
        |x, z| Luma([if is_in_building(min_x + i16::try_from(x).unwrap(), y, min_z + i16::try_from(z).unwrap()) { u8::MAX } else { u8::MIN }]),
    )
}

pub fn perimeter_image(verbose: bool) -> GrayImage {
    if verbose {
        eprintln!("{} Naron Benh: calculating main building", Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
    let min_x = 4000;
    let max_x = 4700;
    let min_z = -4500;
    let max_z = -3800;
    let mut main_building = HashSet::default();
    for z in min_z..max_z {
        for x in min_x..max_x {
            if (-36..140).any(|y| is_in_building(x, y, z)) {
                main_building.insert([x, z]);
            }
        }
    }
    if verbose {
        eprintln!("{} Naron Benh: calculating perimeter", Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
    let pixels = (min_z..max_z).into_par_iter()
        .flat_map(|z| (min_x..max_x).into_par_iter().map(move |x| [x, z]))
        .map(|[x, z]| if is_in_perimeter_of(&main_building, x, z) { u8::MAX } else { u8::MIN })
        .collect();
    if verbose {
        eprintln!("{} Naron Benh: creating image", Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
    GrayImage::from_vec(u32::try_from(max_x - min_x).unwrap(), u32::try_from(max_z - min_z).unwrap(), pixels).unwrap()
}
