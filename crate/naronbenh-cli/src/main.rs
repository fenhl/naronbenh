use {
    image::{
        ImageError,
        ImageFormat,
    },
    naronbenh::*,
};

#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
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
fn main(Args { verbose, subcommand }: Args) -> Result<bool, ImageError> {
    Ok(match subcommand {
        Subcommand::CheckPerimeter { x, z } => {
            let contained = is_in_perimeter(x, z);
            if contained {
                println!("{x} {z} is within the Naron Benh perimeter")
            } else {
                println!("{x} {z} is OUTSIDE the Naron Benh perimeter")
            }
            contained
        }
        Subcommand::CheckBuilding { x, y, z } => {
            let contained = is_in_building(x, y, z);
            if contained {
                println!("{x} {z} is within the Naron Benh building")
            } else {
                println!("{x} {z} is OUTSIDE the Naron Benh building")
            }
            contained
        }
        Subcommand::DrawPerimeter => {
            perimeter_image(verbose).save_with_format("assets/perimeter.png", ImageFormat::Png)?;
            true
        }
        Subcommand::DrawBuilding => {
            for y in -36..140 {
                building_image(y).save_with_format(format!("assets/building/y{y}.png"), ImageFormat::Png)?;
            }
            true
        }
    })
}
