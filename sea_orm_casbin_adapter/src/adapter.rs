use std::time::Duration;

use crate::actions as adapter;
use crate::models::Model as CasbinRule;
use crate::models::*;
use async_trait::async_trait;
use casbin::{Adapter, Filter, Model, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub struct SeaOrmAdapter {
    pool: DatabaseConnection,
    is_filtered: bool,
}

impl<'a> SeaOrmAdapter {
    pub async fn new<U: Into<String>>(url: U) -> Result<Self> {
        let mut opt = ConnectOptions::new(url.into().to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8));
        let pool = Database::connect(opt).await.expect("数据库打开失败");
        adapter::new(&pool).await.map(|_| Self {
            pool,
            is_filtered: false,
        })
    }

    pub(crate) fn save_policy_line(
        &self,
        ptype: &'a str,
        rule: &'a [String],
    ) -> Option<NewCasbinRule> {
        if ptype.trim().is_empty() || rule.is_empty() {
            return None;
        }

        let mut new_rule = NewCasbinRule {
            ptype: ptype.to_string(),
            v0: "".to_string(),
            v1: Some("".to_string()),
            v2: Some("".to_string()),
            v3: Some("".to_string()),
            v4: Some("".to_string()),
            v5: Some("".to_string()),
        };

        new_rule.v0 = (&rule[0]).clone();

        if rule.len() > 1 {
            new_rule.v1 = Some((&rule[1]).clone());
        }

        if rule.len() > 2 {
            new_rule.v2 = Some((&rule[2]).clone());
        }

        if rule.len() > 3 {
            new_rule.v3 = Some((&rule[3]).clone());
        }

        if rule.len() > 4 {
            new_rule.v4 = Some((&rule[4]).clone());
        }

        if rule.len() > 5 {
            new_rule.v5 = Some((&rule[5]).clone());
        }

        Some(new_rule)
    }

    pub(crate) fn load_policy_line(&self, casbin_rule: &CasbinRule) -> Option<Vec<String>> {
        if casbin_rule.ptype.chars().next().is_some() {
            return self.normalize_policy(casbin_rule);
        }

        None
    }

    fn normalize_policy(&self, casbin_rule: &CasbinRule) -> Option<Vec<String>> {
        let mut result = vec![
            &casbin_rule.v0,
            &casbin_rule.v1,
            &casbin_rule.v2,
            &casbin_rule.v3,
            &casbin_rule.v4,
            &casbin_rule.v5,
        ];

        while let Some(last) = result.last() {
            if last.is_empty() {
                result.pop();
            } else {
                break;
            }
        }

        if !result.is_empty() {
            return Some(result.iter().map(|&x| x.to_owned()).collect());
        }

        None
    }
}

#[async_trait]
impl Adapter for SeaOrmAdapter {
    async fn load_policy(&self, m: &mut dyn Model) -> Result<()> {
        let rules = adapter::load_policy(&self.pool).await?;

        for casbin_rule in &rules {
            let rule = self.load_policy_line(casbin_rule);

            if let Some(ref sec) = casbin_rule.ptype.chars().next().map(|x| x.to_string()) {
                if let Some(t1) = m.get_mut_model().get_mut(sec) {
                    if let Some(t2) = t1.get_mut(&casbin_rule.ptype) {
                        if let Some(rule) = rule {
                            t2.get_mut_policy().insert(rule);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> Result<()> {
        let rules = adapter::load_filtered_policy(&self.pool, &f).await?;
        self.is_filtered = true;

        for casbin_rule in &rules {
            if let Some(policy) = self.normalize_policy(casbin_rule) {
                if let Some(ref sec) = casbin_rule.ptype.chars().next().map(|x| x.to_string()) {
                    if let Some(t1) = m.get_mut_model().get_mut(sec) {
                        if let Some(t2) = t1.get_mut(&casbin_rule.ptype) {
                            t2.get_mut_policy().insert(policy);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let mut rules = vec![];

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| self.save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| self.save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }
        adapter::save_policy(&self.pool, rules).await
    }

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        if let Some(new_rule) = self.save_policy_line(ptype, rule.as_slice()) {
            return adapter::add_policy(&self.pool, new_rule).await;
        }

        Ok(false)
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let new_rules = rules
            .iter()
            .filter_map(|x| self.save_policy_line(ptype, x))
            .collect::<Vec<NewCasbinRule>>();

        adapter::add_policies(&self.pool, new_rules).await
    }

    async fn remove_policy(&mut self, _sec: &str, pt: &str, rule: Vec<String>) -> Result<bool> {
        adapter::remove_policy(&self.pool, pt, rule).await
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        pt: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        adapter::remove_policies(&self.pool, pt, rules).await
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        pt: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_index <= 5 && !field_values.is_empty() && field_values.len() >= 6 - field_index {
            adapter::remove_filtered_policy(&self.pool, pt, field_index, field_values).await
        } else {
            Ok(false)
        }
    }

    async fn clear_policy(&mut self) -> Result<()> {
        adapter::clear_policy(&self.pool).await
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }
}
