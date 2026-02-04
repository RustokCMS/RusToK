use rhai::{Dynamic, Map};

#[derive(Debug, Clone, Default)]
pub struct EntityProxy {
    data: Map,
    changes: Map,
}

impl EntityProxy {
    pub fn new(data: Map) -> Self {
        Self {
            data,
            changes: Map::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Dynamic> {
        self.changes
            .get(key)
            .cloned()
            .or_else(|| self.data.get(key).cloned())
    }

    pub fn set(&mut self, key: impl Into<String>, value: Dynamic) {
        self.changes.insert(key.into().into(), value);
    }

    pub fn data(&self) -> &Map {
        &self.data
    }

    pub fn changes(&self) -> &Map {
        &self.changes
    }

    pub fn into_changes(self) -> Map {
        self.changes
    }
}
