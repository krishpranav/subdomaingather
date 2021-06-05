use addr::DomainName;
use std::collections::HashSet;
use std::hash::Hash;


enum Filter {
    SubOnly,
    RootOnly,
}