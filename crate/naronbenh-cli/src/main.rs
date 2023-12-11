use {
    image::{
        ImageError,
        ImageFormat,
    },
    naronbenh::*,
};

#[derive(clap::Parser)]
enum Args {
    CheckPerimeter {
        x: i16,
        z: i16,
    },
    CheckBuilding {
        x: i16,
        y: i16,
        z: i16,
    },
    DrawPerimeter,
    DrawBuilding,
}

#[wheel::main]
fn main(args: Args) -> Result<bool, ImageError> {
    Ok(match args {
        Args::CheckPerimeter { x, z } => {
            let contained = is_in_perimeter(x, z);
            if contained {
                println!("{x} {z} is within the Naron Benh perimeter")
            } else {
                println!("{x} {z} is OUTSIDE the Naron Benh perimeter")
            }
            contained
        }
        Args::CheckBuilding { x, y, z } => {
            let contained = is_in_building(x, y, z);
            if contained {
                println!("{x} {z} is within the Naron Benh building")
            } else {
                println!("{x} {z} is OUTSIDE the Naron Benh building")
            }
            contained
        }
        Args::DrawPerimeter => {
            perimeter_image().save_with_format("assets/perimeter.png", ImageFormat::Png)?;
            true
        }
        Args::DrawBuilding => {
            for y in -36..140 {
                building_image(y).save_with_format(format!("assets/building/y{y}.png"), ImageFormat::Png)?;
            }
            true
        }
    })
}
