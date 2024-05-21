class ForwardTask {
    id: string;
    from_wxid_list: string[];
    to_wxid_list: string[];

    constructor(id:string,from_wxid_list:string[],to_wxid_list: string[]){
        this.id = id;
        this.from_wxid_list = from_wxid_list;
        this.to_wxid_list = to_wxid_list;
    }
}
 
