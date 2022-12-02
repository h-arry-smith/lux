use std::ops::Add;

const DMX_MAX_ADDRESS: u16 = 512;

#[derive(Debug, PartialEq)]
pub struct Address {
    universe: u16,
    address: u16,
}

impl Address {
    fn new(universe: u16, address: u16) -> Self {
        Self { universe, address }
    }
}

impl Add<Address> for Address {
    type Output = Address;

    fn add(self, rhs: Address) -> Self::Output {
        let mut universe = self.universe + rhs.universe;
        let mut address = self.address + rhs.address;

        if address > DMX_MAX_ADDRESS {
            universe += 1;
            address = 1;
        }

        Self::Output { universe, address }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_below_limit() {
        let a = Address::new(1, 1);
        let b = Address::new(0, 40);
        let c = Address::new(1, 41);

        assert_eq!(a + b, c);
    }

    #[test]
    fn add_above_limit() {
        let a = Address::new(1, 501);
        let b = Address::new(0, 40);
        let c = Address::new(2, 1);

        assert_eq!(a + b, c);
    }

    #[test]
    fn edge_case() {
        let a = Address::new(1, 510);
        let b = Address::new(0, 2);
        let c = Address::new(1, 512);

        assert_eq!(a + b, c);
    }

    #[test]
    fn adding_universes() {
        let a = Address::new(1, 234);
        let b = Address::new(2, 111);
        let c = Address::new(3, 345);

        assert_eq!(a + b, c);
    }
}