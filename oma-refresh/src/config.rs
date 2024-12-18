use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    env,
    path::Path,
};

use ahash::AHashMap;
use aho_corasick::AhoCorasick;
use oma_apt::config::{Config, ConfigTree};
use oma_fetch::CompressFile;

use crate::{db::RefreshError, inrelease::ChecksumItem};

fn modify_result(
    tree: ConfigTree,
    res: &mut HashMap<String, HashMap<String, String>>,
    root_path: String,
) {
    let mut stack = VecDeque::new();
    stack.push_back((tree, root_path));

    let mut first = true;

    while let Some((node, tree_path)) = stack.pop_back() {
        // 跳过要遍历根节点的相邻节点
        if !first {
            if let Some(entry) = node.sibling() {
                stack.push_back((entry, tree_path.clone()));
            }
        }

        let Some(tag) = node.tag() else {
            continue;
        };

        if let Some(entry) = node.child() {
            stack.push_back((entry, format!("{}::{}", tree_path, tag)));
        }

        if let Some((k, v)) = node.tag().zip(node.value()) {
            res.entry(tree_path).or_default().insert(k, v);
        }

        first = false;
    }
}

pub fn get_tree(config: &Config, key: &str) -> Vec<(String, HashMap<String, String>)> {
    let mut res = HashMap::new();
    let tree = config.tree(key);

    let Some(tree) = tree else {
        return vec![];
    };

    modify_result(
        tree,
        &mut res,
        key.rsplit_once("::")
            .map(|x| x.0.to_string())
            .unwrap_or_else(|| key.to_string()),
    );

    res.into_iter().collect::<Vec<_>>()
}

pub struct IndexTargetConfig<'a> {
    deb: Vec<(String, HashMap<String, String>)>,
    deb_src: Vec<(String, HashMap<String, String>)>,
    replacer: AhoCorasick,
    native_arch: &'a str,
}

impl<'a> IndexTargetConfig<'a> {
    pub fn new(config: &Config, native_arch: &'a str) -> Self {
        Self {
            deb: get_index_target_tree(config, "Acquire::IndexTargets::deb"),
            deb_src: get_index_target_tree(config, "Acquire::IndexTargets::deb-src"),
            replacer: AhoCorasick::new([
                "$(ARCHITECTURE)",
                "$(COMPONENT)",
                "$(LANGUAGE)",
                "$(NATIVE_ARCHITECTURE)",
            ])
            .unwrap(),
            native_arch,
        }
    }

    pub fn get_download_list(
        &self,
        checksums: &[ChecksumItem],
        is_source: bool,
        is_flat: bool,
        archs: &mut Vec<&str>,
        components: &[String],
    ) -> Result<Vec<ChecksumDownloadEntry>, RefreshError> {
        let key = if is_flat { "flatMetaKey" } else { "MetaKey" };
        let lang = env::var("LANG").map(Cow::Owned).unwrap_or("C".into());
        let langs = get_matches_language(&lang);

        let mut res_map: AHashMap<String, Vec<ChecksumDownloadEntry>> = AHashMap::new();

        let tree = if is_source { &self.deb_src } else { &self.deb };

        if !archs.contains(&"all") {
            archs.push("all");
        }

        for c in checksums {
            for (template, config) in tree.iter().map(|x| (x.1.get(key), &x.1)) {
                let template =
                    template.ok_or_else(|| RefreshError::WrongConfigEntry(key.to_string()))?;

                for a in &*archs {
                    for comp in components {
                        for l in &langs {
                            if self.template_is_match(template, &c.name, a, comp, l) {
                                let name_without_ext = Path::new(&c.name).with_extension("");
                                let name_without_ext =
                                    name_without_ext.to_string_lossy().to_string();
                                res_map.entry(name_without_ext).or_default().push(
                                    ChecksumDownloadEntry {
                                        item: c.to_owned(),
                                        keep_compress: config
                                            .get("KeepCompressed")
                                            .and_then(|x| x.parse::<bool>().ok())
                                            .unwrap_or(false),
                                        msg: config
                                            .get("ShortDescription")
                                            .map(|x| {
                                                self.replacer.replace_all(
                                                    x,
                                                    &[*a, comp, l, self.native_arch],
                                                )
                                            })
                                            .unwrap_or_else(|| "Other".to_string()),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        let mut res = vec![];

        for (_, v) in &mut res_map {
            v.sort_unstable_by(|a, b| {
                compress_file(&a.item.name).cmp(&compress_file(&b.item.name))
            });
            if v[0].item.size == 0 {
                continue;
            }
            res.push(v.last().unwrap().to_owned());
        }

        Ok(res)
    }

    fn template_is_match(
        &self,
        template: &str,
        target: &str,
        arch: &str,
        component: &str,
        lang: &str,
    ) -> bool {
        let target_without_ext = Path::new(target).with_extension("");
        let target_without_ext = target_without_ext.to_string_lossy();

        target_without_ext
            == self
                .replacer
                .replace_all(template, &[arch, component, lang, self.native_arch])
    }
}

fn get_index_target_tree(config: &Config, key: &str) -> Vec<(String, HashMap<String, String>)> {
    get_tree(config, key)
        .into_iter()
        .filter(|x| {
            x.1.get("DefaultEnabled")
                .and_then(|x| x.parse::<bool>().ok())
                .unwrap_or(true)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
pub struct ChecksumDownloadEntry {
    pub item: ChecksumItem,
    pub keep_compress: bool,
    pub msg: String,
}

fn get_matches_language(env_lang: &str) -> Vec<&str> {
    let mut langs = vec![];
    let env_lang = env_lang.split_once('.').map(|x| x.0).unwrap_or(env_lang);

    let lang = if env_lang == "C" { "en" } else { env_lang };

    langs.push(lang);

    // en_US.UTF-8 => en
    if let Some((a, _)) = lang.split_once('_') {
        langs.push(a);
    }

    langs
}

fn compress_file(name: &str) -> CompressFile {
    CompressFile::from(
        Path::new(name)
            .extension()
            .map(|x| x.to_string_lossy())
            .unwrap_or_default()
            .to_string()
            .as_str(),
    )
}

#[test]
fn test_get_matches_language() {
    assert_eq!(get_matches_language("C"), vec!["en"]);
    assert_eq!(get_matches_language("zh_CN.UTF-8"), vec!["zh_CN", "zh"]);
    assert_eq!(get_matches_language("en_US.UTF-8"), vec!["en_US", "en"]);
}

#[test]
fn test_get_tree() {
    let t = get_tree(&Config::new(), "Acquire::IndexTargets::deb");
    assert!(t.iter().any(|x| x.0.contains("::deb::")));
    assert!(t.iter().all(|x| !x.0.contains("::deb-src::")))
}
