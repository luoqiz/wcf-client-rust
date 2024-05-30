import { invoke } from "@tauri-apps/api/core";
import { ForwardTask } from "~/types/task";
import { useServiceStore } from "~/stores/service";

// 保存任务为文件
export const task_save = async (wxid: string, forwardTask: ForwardTask) => {
  await invoke("write_wxid_task", {wxid: wxid,task: forwardTask});
}

export const task_list_all = async() => {
  const serviceStore = useServiceStore();
  const result = await invoke("read_wxid_task", {wxid: serviceStore.userInfo?.wxid});
  return result as ForwardTask[]
}