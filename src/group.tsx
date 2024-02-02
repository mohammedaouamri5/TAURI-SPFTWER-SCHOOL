
import * as React from 'react';

import { User } from "./user";
import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { MyTable, Column } from "./MyTabule";




export interface GroupsOfUsers {
  id_group: number,
  id_user: number,
  name: string,
}


export interface Group {
  date_end: string;
  date_start: string;
  id: number;
  idfrom: number;
  idteacher: number;
  idto: number;
  name: String
  is_done: boolean;
}


interface All_The_Groups_TableProps {
  data: Group[];
}


interface UserIdGroup {
  [key: string]: User;
}


export function All_The_Groups_Table({ data }: All_The_Groups_TableProps) {
  const [users, setUsers] = useState<UserIdGroup>({});

  const setUserData = async (element: Group) => {
    try {
      const m_user: User = await invoke<User>("get_user_by_id", { id: element.id });

      setUsers((prevUsers) => ({
        ...prevUsers,
        [`${element.id}`]: new User(
          m_user.id,
          m_user.id_type,
          m_user.name,
          m_user.family_name,
          m_user.birth_day,
          m_user.notes
        )
      }));

    } catch (error) {
      console.error("Error fetching user:", error);
    }
  };


  React.useEffect(() => {
    if (data.length > 0 && Object.keys(users).length === 0) {
      data.forEach((element) => {
        setUserData(element);
      });
    }
  }, [data, users]);


  const columns: readonly Column[] = [

    {
      id: 'date_start',
      label: 'started',
      align: 'center',
      minWidth: 90
    },
    {
      id: 'idteacher',
      label: 'by',
      minWidth: 100,
      align: 'center',
      format: (value: number) => {

        // return `${value}`;
        return users[`${value}`]?.name || "bruh";
      },
    },
    {
      id: 'idfrom',
      label: 'from',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    },
    {
      id: 'idto',
      label: 'to',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    },
    {
      id: 'is_done',
      label: 'working',
      minWidth: 70,
      align: 'center',
      format: (value: number) => { if (value == 0) { return "✅" } else { return "❌" } },
    },
    {
      id: 'date_end',
      label: 'until',
      align: 'center',
      minWidth: 120
    },
  ];

  return <MyTable<Group> data={data} columns={columns} name={"All_The_Groups_Table"} />;



}



interface A_Groups_TableProps {
  group_id: number;
}


export function A_Groups_Table({ group_id }: A_Groups_TableProps) {
  const [users, setUsers] = useState<User[]>([]);

  const setUserData = async (group_id: number) => {
    try {
      const respond: Promise<User[]> = invoke<User[]>("get_all_the_users_by_group", { groupId: group_id });
      respond.then(
        (p_users: User[]) => { setUsers(p_users) }
      );
    } catch (error) {
      console.error("Error fetching user:", error);
    }
  };

  React.useEffect(() => {
    setUserData(group_id);
  }, [group_id, users]);



  const columns: readonly Column[] = [
    {
      id: 'name',
      label: 'name',
      minWidth: 100,
      align: 'center',
      format: (value: number) => {

        // return `${value}`;
        return users[`${value}`]?.name || "bruh";
      },
    },
    {
      id: 'family_name',
      label: 'family name',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    },
    {
      id: 'birth_day',
      label: 'birth day',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    }
  ];

  return <MyTable<User> data={users} columns={columns} name={"A_Groups_Table"}/>;

}



