use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

type Callback<T> = Box<dyn Fn(T) -> T + Send + Sync>;

pub struct Hook<T> {
    handlers: Arc<RwLock<BTreeMap<i32, HashMap<String, Callback<T>>>>>,
}

impl<T> Default for Hook<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Hook<T> {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add(&self, name: &str, priority: i32, callback: impl Fn(T) -> T + Send + Sync + 'static) {
        let mut handlers = self.handlers.write().unwrap();
        let priority_group = handlers.entry(priority).or_default();
        priority_group.insert(name.to_string(), Box::new(callback));
    }

    pub fn remove(&self, name: &str, priority: i32) {
        let mut handlers = self.handlers.write().unwrap();
        if let Some(priority_group) = handlers.get_mut(&priority) {
            priority_group.remove(name);
        }
    }

    pub fn apply(&self, mut value: T) -> T {
        let handlers = self.handlers.read().unwrap();
        for priority_group in handlers.values() {
            for callback in priority_group.values() {
                value = callback(value);
            }
        }
        value
    }
}

pub struct Hooks<T> {
    hooks: RwLock<HashMap<String, Hook<T>>>,
}

impl<T> Default for Hooks<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Hooks<T> {
    pub fn new() -> Self {
        Self {
            hooks: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_filter(
        &self,
        hook_name: &str,
        namespace: &str,
        priority: i32,
        callback: impl Fn(T) -> T + Send + Sync + 'static,
    ) {
        let mut hooks = self.hooks.write().unwrap();
        let hook = hooks.entry(hook_name.to_string()).or_insert_with(Hook::new);
        hook.add(namespace, priority, callback);
    }

    pub fn apply_filters(&self, hook_name: &str, value: T) -> T {
        let hooks = self.hooks.read().unwrap();
        if let Some(hook) = hooks.get(hook_name) {
            hook.apply(value)
        } else {
            value
        }
    }

    pub fn add_action(
        &self,
        hook_name: &str,
        namespace: &str,
        priority: i32,
        callback: impl Fn(T) -> T + Send + Sync + 'static,
    ) {
        self.add_filter(hook_name, namespace, priority, callback);
    }

    pub fn do_action(&self, hook_name: &str, value: T) {
        self.apply_filters(hook_name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_apply_filters() {
        let hooks = Hooks::<String>::new();
        
        hooks.add_filter("the_content", "my_plugin", 10, |content| {
            format!("{} - modified", content)
        });

        let result = hooks.apply_filters("the_content", "original".to_string());
        assert_eq!(result, "original - modified");
    }

    #[test]
    fn test_priority_sorting() {
        let hooks = Hooks::<String>::new();
        
        // Priority 20 should run after priority 10
        hooks.add_filter("the_content", "plugin_20", 20, |content| {
            format!("{} - plugin_20", content)
        });

        hooks.add_filter("the_content", "plugin_10", 10, |content| {
            format!("{} - plugin_10", content)
        });

        let result = hooks.apply_filters("the_content", "original".to_string());
        // First 10, then 20
        assert_eq!(result, "original - plugin_10 - plugin_20");
    }
}
