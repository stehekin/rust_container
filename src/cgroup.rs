use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct CGroup {
    name: String,
    number: u32,
    parent: Option<Rc<CGroup>>,
}

#[derive(Default)]
pub(crate) struct CGroupCache {
    cgroups_by_number: HashMap<u32, Rc<CGroup>>,
    cgroups_by_name: HashMap<String, Rc<CGroup>>,
}

impl CGroupCache {
    pub(crate) fn add_cgroup(&mut self, number: u32, name: String, parent: Option<u32>) {
        let parent = match parent {
            Some(number) => match self.cgroups_by_number.get(&number) {
                None => None,
                Some(p) => Some(p.clone()),
            },
            None => None,
        };

        let cgrc = Rc::new(CGroup {
            name: name.clone(),
            number,
            parent,
        });

        self.cgroups_by_number.insert(number, cgrc.clone());
        self.cgroups_by_name.insert(name, cgrc.clone());
    }

    pub(crate) fn is_descendant(&self, cg1_number: u32, cg2_name: String) -> bool {
        let cg1 = self.cgroups_by_number.get(&cg1_number);
        if cg1.is_none() {
            return false;
        }

        let mut cg1 = cg1.unwrap();

        loop {
            match &cg1.parent {
                Some(p) => {
                    if p.name == cg2_name {
                        return true;
                    } else {
                        cg1 = p;
                    }
                }
                None => return false,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::CGroupCache;

    #[test]
    fn test_descendant() {
        let mut cache = CGroupCache::default();
        cache.add_cgroup(0, "0".into(), None);
        cache.add_cgroup(1, "1".into(), Some(0));
        cache.add_cgroup(2, "2".into(), Some(0));
        cache.add_cgroup(3, "3".into(), Some(1));
        cache.add_cgroup(4, "4".into(), Some(1));
        cache.add_cgroup(5, "5".into(), Some(2));
        cache.add_cgroup(6, "6".into(), Some(2));

        for i in 0..=6 {
            for j in 0..=6 {}
        }
    }
}
