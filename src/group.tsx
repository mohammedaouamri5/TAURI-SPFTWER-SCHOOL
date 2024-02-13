import * as React from "react";

import { User } from "./user";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { MyTable, Column } from "./MyTabule";
// import { Type } from "./Types";
import { LOG } from "./LOG";
// import { Level } from "./Level";

export interface GroupsOfUsers {
  id_group: number;
  id_user: number;
  name: string;
}

export interface Group {
  date_end: string;
  date_start: string;
  id: number;
  idfrom: number;
  idteacher: number;
  idto: number;
  name: String;
  is_done: boolean;
}

export class GroupInfo {
  public date_end?: string;
  public setdate_end(date_end: string) {
    this.date_end = date_end;
  }

  public date_start?: string;
  public setdate_start(date_start: string) {
    this.date_start = date_start;
  }

  public id: number;

  public from?: string;
  public setfrom(from: string) {
    this.from = from;
  }

  public teacher?: string;
  public setteacher(teacher: string) {
    this.teacher = teacher;
  }

  public to?: string;
  public setto(to: string) {
    this.to = to;
  }

  public name?: String;
  public setname(name: String) {
    this.name = name;
  }

  public is_done?: boolean;
   constructor(p_group:Group) {
    this.date_end = p_group.date_end 
    this.date_start = p_group.date_start
    this.is_done =  p_group.is_done
    this.id = p_group.id
    this.name = p_group.name
  }
}

interface All_The_Groups_TableProps {
  data: Group[];
}

export function All_The_Groups_Table({ data }: All_The_Groups_TableProps) {
  const [users, setUsers] = useState<User[]>([]);
  // const [levels, setLevel] = useState<Level[]>([]);
  const groupsinfo = useState<GroupInfo[]>([])[0];
  // const state = useState({
  //   users: false,
  //   levels: false,
  //   types: false,
  //   groups_of_users: false,
  // })[0];

  const fetch_users_by_ids = async (p_groups: Group[]) => {
    try {
      const respond: Promise<User[]> = invoke<User[]>("get_users_by_ids", {
        id: Array.from(new Set(p_groups.map((obj) => obj.idteacher))),
      });
      respond.then((p_users: User[]) => {
        setUsers(p_users);
      });
    } catch (error) {
      LOG.error("Type", "Error fetching GroupsOfUsers:", error);
    }
  };

  const fulfill_groups = () => {
    if (data.length > 0 && groupsinfo.length == 0) {
      data.map((p_group:Group)=> {
        groupsinfo.push(new GroupInfo(p_group))
      })
     }
  };

  useEffect(() => {
    if (data.length > 0 && users.length < 0) {
      fetch_users_by_ids(data);
    }
  }, [users]);

  useEffect(() => {
    fulfill_groups();
  }, [data, users]);


  const columns: readonly Column[] = [
    {
      id: "date_start",
      label: "started",
      align: "center",
      minWidth: 90,
    },
    {
      id: "idteacher",
      label: "by",
      minWidth: 100,
      align: "center",
      format: (value: number) => {
        // return `${value}`;
        return users[`${value}`]?.name || "bruh";
      },
    },
    {
      id: "idfrom",
      label: "from",
      minWidth: 100,
      align: "center",
      format: (value: number) => value.toLocaleString("en-US"),
    },
    {
      id: "idto",
      label: "to",
      minWidth: 100,
      align: "center",
      format: (value: number) => value.toLocaleString("en-US"),
    },
    {
      id: "is_done",
      label: "working",
      minWidth: 70,
      align: "center",
      format: (value: number) => {
        if (value == 0) {
          return "✅";
        } else {
          return "❌";
        }
      },
    },
    {
      id: "date_end",
      label: "until",
      align: "center",
      minWidth: 120,
    },
  ];

  return <MyTable<Group> data={data} columns={columns} name={"groups"} />;
}

interface A_Groups_TableProps {
  group_id: number;
}

export function A_Groups_Table({ group_id }: A_Groups_TableProps) {
  const [users, setUsers] = useState<User[]>([]);

  const setUserData = async (group_id: number) => {
    try {
      const respond: Promise<User[]> = invoke<User[]>(
        "get_all_the_users_by_group",
        { groupId: group_id }
      );
      respond.then((p_users: User[]) => {
        setUsers(p_users);
      });
    } catch (error) {
      console.error("Error fetching user:", error);
    }
  };

  React.useEffect(() => {
    setUserData(group_id);
  }, [group_id, users]);

  const columns: readonly Column[] = [
    {
      id: "name",
      label: "name",
      minWidth: 100,
      align: "center",
      format: (value: number) => {
        // return `${value}`;
        return users[`${value}`]?.name || "bruh";
      },
    },
    {
      id: "family_name",
      label: "family name",
      minWidth: 100,
      align: "center",
      format: (value: number) => value.toLocaleString("en-US"),
    },
    {
      id: "birth_day",
      label: "birth day",
      minWidth: 100,
      align: "center",
      format: (value: number) => value.toLocaleString("en-US"),
    },
  ];

  return (
    <MyTable<User> data={users} columns={columns} name={"A_Groups_Table"} />
  );
}
