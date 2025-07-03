mod helios;
mod point;
mod selene;

pub use helios::HeliosPoint;
pub use selene::SelenePoint;

#[cfg(test)]
mod tests {
    use group::Group;

    use super::{HeliosPoint, SelenePoint};

    // Checks random won't infinitely loop
    #[test]
    fn random() {
        HeliosPoint::random(&mut rand_core::OsRng);
        SelenePoint::random(&mut rand_core::OsRng);
    }
}
