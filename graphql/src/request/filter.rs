use super::name::RequestName;

#[must_use]
pub fn include_filter<const N: usize>(values: [RequestName; N]) -> RequestInclusions<N> {
    RequestInclusions(values)
}

#[must_use]
pub fn exclude_filter<const N: usize>(values: [RequestName; N]) -> RequestExclusions<N> {
    RequestExclusions(values)
}

pub trait RequestFilter {
    fn include(&self, name: RequestName) -> bool;
}

impl<F: Fn(RequestName) -> bool> RequestFilter for F {
    fn include(&self, name: RequestName) -> bool {
        self(name)
    }
}

pub struct RequestInclusions<const N: usize>([RequestName; N]);

impl<const N: usize> RequestFilter for RequestInclusions<N> {
    fn include(&self, name: RequestName) -> bool {
        self.0.contains(&name)
    }
}

pub struct RequestExclusions<const N: usize>([RequestName; N]);

impl<const N: usize> RequestFilter for RequestExclusions<N> {
    fn include(&self, name: RequestName) -> bool {
        !self.0.contains(&name)
    }
}

impl RequestFilter for () {
    fn include(&self, _name: RequestName) -> bool {
        true
    }
}
