extern crate diecast;
extern crate typemap;

use std::collections::HashMap;

use diecast::{Item, Handle};

pub struct Versions;

impl typemap::Key for Versions {
    type Value = HashMap<String, String>;
}

pub fn get(item: &Item, name: &str) -> Option<String> {
    item.extensions.get::<Versions>()
    .and_then(|versions| versions.get(name).cloned())
}

pub fn insert<N>(item: &mut Item, name: N, body: String) -> Option<String>
where N: Into<String> {
    item.extensions.get_mut::<Versions>()
    .and_then(|versions| versions.insert(name.into(), body))
}

pub fn remove(item: &mut Item, name: &str) -> Option<String> {
    item.extensions.get_mut::<Versions>()
    .and_then(|versions| versions.remove(name))
}

pub struct Save {
    name: String,
}

impl Handle<Item> for Save {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        item.extensions.entry::<Versions>()
            .or_insert_with(|| HashMap::new())
            .insert(self.name.clone(), item.body.clone());

        Ok(())
    }
}

pub fn save<S: Into<String>>(name: S) -> Save {
    Save {
        name: name.into()
    }
}

pub struct Load {
    name: String,
}

impl Handle<Item> for Load {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        // TODO
        // should this be get or remove?
        // remove would have the advantage of removing
        // the old version to conserve memory
        let version =
            item.extensions.get::<Versions>()
            .and_then(|versions| versions.get(&self.name));

        if let Some(version) = version {
            item.body = version.clone();
            Ok(())
        } else {
            Err(From::from(format!("No version `{}`", self.name)))
        }
    }
}

pub fn load<L: Into<String>>(name: L) -> Load {
    Load {
        name: name.into()
    }
}

pub struct LoadAndRemove {
    name: String,
}

impl Handle<Item> for LoadAndRemove {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        // TODO
        // should this be get or remove?
        // remove would have the advantage of removing
        // the old version to conserve memory
        let version =
            item.extensions.get::<Versions>()
            .and_then(|versions| versions.get(&self.name));

        if let Some(version) = version {
            item.body = version.clone();
            Ok(())
        } else {
            Err(From::from(format!("No version `{}`", self.name)))
        }
    }
}

pub fn load_and_remove<L: Into<String>>(name: L) -> LoadAndRemove {
    LoadAndRemove {
        name: name.into()
    }
}

