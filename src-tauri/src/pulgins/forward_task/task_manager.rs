use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::pulgins::forward_task::task_file::Task;

use super::task_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct WxidMapping {
    from_wxid: String,
    taskid_list: Vec<String>,
    to_wxid_list: Vec<String>,
}

#[derive(Debug)]
pub struct TaskManager {
    // pub root_dir: String,
    pub wxid: String,
    pub tasks: Vec<Task>,
    pub mappings: HashMap<String, WxidMapping>,
}

impl TaskManager {
    //, root_dir:String
    pub fn new(wxid: Option<String>) -> Self {
        match wxid {
            Some(value) => {
                let tasks = task_file::read_from_json_file(&value);
                let mappings = Self::aggregate_tasks(&tasks);
                TaskManager {
                    // root_dir,
                    wxid: value,
                    tasks: tasks,
                    mappings: mappings,
                }
            }
            None => {
                TaskManager {
                    // root_dir,
                    wxid: "".to_string(),
                    tasks: vec![],
                    mappings: HashMap::new(),
                }
            }
        }
    }

    pub fn update_wxid(&mut self, wxid: String) {
        let tasks = task_file::read_from_json_file(&wxid.clone());
        let mappings = Self::aggregate_tasks(&tasks);
        self.wxid = wxid;
        self.tasks = tasks;
        self.mappings = mappings;
    }

    pub fn aggregate_tasks(tasks: &Vec<Task>) -> HashMap<String, WxidMapping> {
        let mut map: HashMap<String, WxidMapping> = HashMap::new();

        for task in tasks {
            if task.enabled {
                for from_wxid in &task.from_wxid_list {
                    let entry = map.entry(from_wxid.clone()).or_insert(WxidMapping {
                        from_wxid: from_wxid.clone(),
                        taskid_list: Vec::new(),
                        to_wxid_list: Vec::new(),
                    });

                    entry.taskid_list.push(task.id.clone());
                    for to_wxid in &task.to_wxid_list {
                        if !entry.to_wxid_list.contains(to_wxid) {
                            entry.to_wxid_list.push(to_wxid.clone());
                        }
                    }
                }
            }
        }

        map
    }

    fn update_mappings(&mut self) {
        self.mappings = Self::aggregate_tasks(&self.tasks);
    }

    pub fn update_task(&mut self, task: Task) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks[pos] = task;
            self.update_mappings();
        }
    }

    pub fn add_or_remove_task(
        &mut self,
        wxid: &str,
        task: Option<Task>,
        remove_id: Option<String>,
    ) {
        if let Some(task) = task {
            // 定义文件路径
            // let file_path = ".\\".to_string() + wxid + "\\task\\"+ &task.id +".json";
            let _ = task_file::write_to_json_file(wxid, &task);
            self.tasks.push(task);
        }
        if let Some(remove_id) = remove_id {
            self.tasks.retain(|t| t.id != remove_id);
        }
        self.update_mappings();
    }

    pub fn get_to_wxids_by_wxid(&mut self, wxid: String) -> Vec<String> {
        let mapping = self.mappings.get(&wxid);
        if let Some(maps) = mapping {
            maps.to_wxid_list.clone()
        } else {
            vec![]
        }
    }
}

impl fmt::Display for TaskManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tasks_json = serde_json::to_string_pretty(&self.tasks).unwrap();
        let mappings_json = serde_json::to_string_pretty(&self.mappings).unwrap();
        write!(f, "Tasks:\n{}\nMappings:\n{}", tasks_json, mappings_json)
    }
}
