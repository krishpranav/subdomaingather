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

#[derive(Default)]
pub struct PostProcessor {
    roots: HashSet<String>,
    filter: Filter,
}


impl PostProcessor {
    pub fn any_root<I: IntoIterator<Item = String>>(&mut self, hosts: I) -> &mut Self {
        self.roots = hosts
            .into_iter()
            .filter_map(|d| d.parse::<DomainName>().ok())
            .map(|d| d.root().to_string())
            .collect();
        self.filter = Filter::RootOnly;
        self
    }

    pub fn any_subdomain<I: IntoIterator<Item = String>>(&mut self, hosts: I) -> &mut Self {
        self.roots.extend(hosts);
        self.filter = Filter::SubOnly;
        self
    }

    fn strip_invalid<T: AsRef<str> + std::fmt::Display>(domain: T) -> String {
        domain
            .as_ref()
            .strip_prefix('.')
            .unwrap_or_else(|| domain.as_ref())
            .replace(&['"', '\"', '\\', '*'][..], "")
            .to_lowercase()
    }

    fn is_relevant<T: AsRef<str>>(&self, result: T) -> bool {
        match self.filter {
            Filter::RootOnly => {
                if let Ok(d) = result.as_ref().parse::<DomainName>() {
                    self.roots.contains(d.root().to_str())
                } else {
                    false
                }
            }
            Filter::SubOnly => self
                .roots
                .iter()
                .any(|root| result.as_ref().ends_with(root) && !result.as_ref().eq(root)),
        }
    }
}

pub struct PostProcessorIter<'a, I>
where
    I: Iterator,
{
    cleaner: &'a PostProcessor,
    inner: I,
}
