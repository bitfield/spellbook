#[non_exhaustive]
#[derive(Debug)]
pub struct Customer {
    pub email: &'static str,
    pub name: &'static str,
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}

fn main() {
    assert_eq!(
        Customer {
            email: "eatme@example.com",
            name: "Hazelnut Swirl",
        },
        Customer {
            email: "eatme@example.com",
            name: "Truffle Heart",
        },
        "customers with same email should be equal"
    );
    assert_ne!(
        Customer {
            email: "eatme@example.com",
            name: "Honey Crunch",
        },
        Customer {
            email: "yumyum@example.com",
            name: "Honey Crunch",
        },
        "customers with different emails shouldn't be equal"
    );
}
