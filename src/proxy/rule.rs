use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::config::config::Location;

#[derive(Debug, Clone)]
pub struct RuleCondition {
    // match_path: String,
    // proxy_pass: HashMap<String, Option<String>>,
    rule: Vec<Location>,
}

// static  RULES: OnceLock<RuleCondition> = OnceLock::new();

static RULES: OnceLock<RwLock<RuleCondition>> = OnceLock::new();

impl RuleCondition {
    pub fn global() -> &'static RwLock<RuleCondition> {
        // 获取或初始化 Logger
        RULES.get_or_init(|| {
            println!("RuleCondition is being created..."); // 初始化打印
            RwLock::new(RuleCondition { rule: vec![] })
        })
    }

    pub fn set_rule(
        &mut self,
        match_path: String,
        proxy_host: Option<String>,
        proxy_url: Option<String>,
    ) {
        let location = Location {
            match_path,
            proxy_host,
            proxy_url,
        };
        // let a = self.deref_mut();
        self.rule.push(location);
    }

    pub fn get_rule(&self) -> Vec<Location> {
        self.rule.clone()
    }

    pub fn get_read_self() -> RwLockReadGuard<'static, RuleCondition> {
        Self::global().read().unwrap()
    }

    pub fn get_mut_self() -> RwLockWriteGuard<'static, RuleCondition> {
        Self::global().write().unwrap()
    }

    pub fn load_rule(rules: Vec<Location>) {
        for location in rules {
            let mut rule = Self::get_mut_self();
            rule.set_rule(location.match_path, location.proxy_host, location.proxy_url);
        }
    }
}
