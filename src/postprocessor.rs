use addr::DomainName;
use std::collections::HashSet;
use std::hash::Hash;


enum Filter {
    SubOnly,
    RootOnly,
}

impl Default for Filter {
    fn default() -> Self {
        Self::RootOnly
    }
}