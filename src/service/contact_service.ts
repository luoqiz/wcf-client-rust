import { invoke } from "@tauri-apps/api/core";
import { BaseResponse } from "~/types/base";
import { Contact } from "~/types/contact";

export const get_contacts = async () => {
  let result: BaseResponse<{ [key: string]: any }> = await invoke('get_contacts');
  const result_contact = result.data["contacts"] as Contact[];
  const contact_remark = [];
  for(let item of result_contact){
    if(!item.remark){
      item.remark = item.name;
    }
    item.label = item.remark;
    item.value = item.wxid;
    contact_remark.push(item);
  }
  return contact_remark;
}