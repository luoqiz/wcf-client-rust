import { invoke } from "@tauri-apps/api/core";
import { ForwardTask } from "~/types/task";

// 保存任务为文件
export const task_save = async (wxid: string, forwardTask: ForwardTask) => {
  await invoke("write_wxid_task", {wxid: wxid,task: forwardTask});
}