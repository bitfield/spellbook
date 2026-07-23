use std::str::FromStr;

use anyhow::{Error, bail};

#[non_exhaustive]
#[derive(Debug)]
pub struct Breakfast {
    pub name: String,
}

impl TryFrom<&str> for Breakfast {
    type Error = Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.contains("bacon") {
            Ok(Self {
                name: name.to_owned(),
            })
        } else {
            bail!("breakfast must contain bacon")
        }
    }
}

impl FromStr for Breakfast {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if name.contains("bacon") {
            Ok(Self {
                name: name.to_owned(),
            })
        } else {
            bail!("breakfast must contain bacon")
        }
    }
}

fn main() {
    println!("{:?}", Breakfast::try_from("sausage & eggs"));
    // Err(breakfast must contain bacon)
    println!("{:?}", Breakfast::try_from("chunky bacon"));
    // Ok(Breakfast { name: "chunky bacon" })

    println!("{:?}", Breakfast::from_str("bacon & eggs"));
    // Ok(Breakfast { name: "bacon & eggs" })

    println!("{:?}", "muesli".parse::<Breakfast>());
    // Err(breakfast must contain bacon)
    println!("{:?}", "bacon sandwich".parse::<Breakfast>());
    // Ok(Breakfast { name: "bacon sandwich" })
}
