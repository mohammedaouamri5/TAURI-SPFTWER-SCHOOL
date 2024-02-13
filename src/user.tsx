import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { Level } from "./Level";
import { Type } from "./Types";
import { GroupsOfUsers } from "./group";
import { LOG  } from "./LOG";
import { Column, MyTable } from "./MyTabule";





export class User {
  constructor(
    public id: number,
    public id_type: number,
    public name: string,
    public family_name: string,
    public birth_day: string,
    public notes: string
  ) {
    this.id = id;
    this.id_type = id_type;
    this.name = name;
    this.family_name = family_name;
    this.birth_day = birth_day;
    this.notes = notes;
  }
}


export class UserInfo {
  public id?: number;
  public type?: string;
  public groups?: string[];
  public level?: string;
  public name?: string;
  public family_name?: string;
  public birth_day?: string;
  public notes?: string;
  public is_active?:boolean;  
  constructor(
    user?: User,
    groups?: string[],
  ) {
    if (user != undefined) {
      this.id = user.id;
      this.name = user.name;
      this.family_name = user.family_name;
      this.birth_day = user.birth_day;
      this.notes = user.notes;
      this.groups = groups?.copyWithin(0, 0)

    }
  }
  public settype(type: string) { this.type = type }
  public setgroups(groups: string[]) { this.groups = groups?.copyWithin(0, 0) }
  public setlevel(level: string) { this.level = level }
  public setname(name: string) { this.name = name }
  public setfamily_name(family_name: string) { this.family_name = family_name }
  public setbirth_day(birth_day: string) { this.birth_day = birth_day }
  public setnotes(notes: string) { this.notes = notes }
  public setis_active(is_active: boolean) { this.is_active = is_active }
  public toString() {
    return `\n{
      \n type: ${this.type ? this.type : "undefined"},
      \n groups: ${this.groups ? this.groups : "undefined"},
      \n level: ${this.level ? this.level : "undefined"},
      \n name: ${this.name ? this.name : "undefined"},
      \n family_name: ${this.family_name ? this.family_name : "undefined"},
      \n birth_day: ${this.birth_day ? this.birth_day : "undefined"},
      \n notes: ${this.notes ? this.notes : "undefined"}
    \n}`;
  }

}
export function All_User_Table() {

  const usersinfo = useState<UserInfo[]>([])[0];

  const state = useState({
    "users": false,
    "levels": false,
    "types": false,
    "groups_of_users": false,
    "is_active_user": false,
  })[0];

  const [users, setUsers] = useState<User[]>([]);
  const [levels, setLevels] = useState<Level[]>([]);
  const [types, setTypes] = useState<Type[]>([]);
  const [groups_of_users, setGroupsOfUsers] = useState<GroupsOfUsers[]>([]);
  const [active_user, set_active_user] = useState<User[]>([]);

  // console.log(invoke<Level[]>("get_groups_of_users", { id: [5, 1] }))


   

  const setUsersData = async () => {
    try {
      const respond: Promise<User[]> = invoke<User[]>("get_all_the_users", {});
      respond.then(
        (p_users: User[]) => { setUsers(p_users) }
      );
    } catch (error) {
      LOG.error("Type", "Error fetching user :", error)
    }
  };

  const setGroupsOfUsersData = async (p_users: User[]) => {
    try {
      const respond: Promise<GroupsOfUsers[]> = invoke<GroupsOfUsers[]>("get_groups_of_users", { id: p_users.map(obj => obj.id) });
      respond.then((p_groups_of_users: GroupsOfUsers[]) => { setGroupsOfUsers(p_groups_of_users) });
    } catch (error) {
      LOG.error("Type", "Error fetching GroupsOfUsers:", error)
    }
  };

  const setisactivness = async ( ) => {
    try {
      const respond: Promise<User[]> = invoke<User[]>("get_all_active_the_users", { });
      
      respond.then((p_users:User[]) => { set_active_user(p_users) });
    } catch (error) {
      LOG.error("Type", "Error fetching GroupsOfUsers:", error)
    }
  };
   

  const setLevelData = async (p_users: User[]) => {
    try {
      const respond: Promise<Level[]> = invoke<Level[]>("get_level_of_users", { id: p_users.map(obj => obj.id) });
      respond.then((p_levels: Level[]) => { setLevels(p_levels) });
    } catch (error) {
      LOG.error("Type", "Error fetching level:", error)
    }
  };

  const setTypeData = async (p_users: User[]) => {
    try {
      const respond: Promise<Type[]> = invoke<Type[]>("get_type_of_users", { id: p_users.map(obj => obj.id) });
      respond.then((p_types: Type[]) => { setTypes(p_types) });
    } catch (error) {
      LOG.error("Type", "Error fetching type:", error)
    }
  };

  const fulfill_usersinfo = () => {
    // console.group("fulfill usersinfo")
    LOG.startInfo("usersinfo", "fulfill start");


    /* adding levels  */ {
      if (levels.length > 0 && !state["levels"]) {
        usersinfo.map((p_userinfo: UserInfo) => {
          p_userinfo.setlevel(levels.find((p_level: Level) => p_level.id === p_userinfo.id)?.name || "bruh")
        })
        LOG.ifInfo(() => usersinfo.findIndex((item: UserInfo) => item.name === "bruh") === -1, 'level', "the levels is add", usersinfo[0].level)
        state["levels"] = true;
      } else if (!state["levels"]) {
        LOG.warn('levels', 'the levels is still did not fetched yet');
      } else {
        LOG.info('levels', "the levels is all readdy added \n ->", usersinfo[0].level)
      }
    }

    /* adding types*/ {
      if (types.length > 0 && !state["types"]) {
        usersinfo.map((p_userinfo: UserInfo) => {
          p_userinfo.settype(types.find((p_type: Type) => p_type.id == users.find((item: User) => p_userinfo.id === item.id)?.id_type)?.name || "bruh")
        })
        LOG.ifInfo(() => usersinfo.findIndex((item: UserInfo) => item.type === "bruh") === -1, 'types', "the types is add", usersinfo[0].type)
        state["types"] = true;
      } else if (!state["types"]) {
        LOG.warn('types', 'the types is still did not fetched yet');
      } else {
        LOG.info('types', "the types is all readdy added \n ->", usersinfo[0].type)
      }
    }
   
    /* adding the active user */ {
      if (active_user.length > 0 && !state["is_active_user"]) {
        usersinfo.map((p_userinfo: UserInfo) => {
          if(typeof(p_userinfo.id) === 'number'){
            p_userinfo.setis_active(
              active_user.findIndex((p_user:User) => p_user.id == p_userinfo.id ) >= 0        
            )}
        })
        LOG.ifInfo(() => usersinfo.findIndex((p_userinfo: UserInfo) => typeof(p_userinfo.id) === "undefined") === -1, 'types', "the is_active is add", usersinfo[0].is_active)
        state["is_active_user"] = true;
      } else if (!state["is_active_user"]) {
        LOG.warn('is_active', 'the is_active is still did not fetched yet');
      } else {
        LOG.info('is_active', "the is_active is all readdy added \n ->", usersinfo[0].is_active)
      }
    }

    /* adding groups of users*/ {
      if (groups_of_users.length > 0 && !state["groups_of_users"]) {
        usersinfo.map((p_userinfo: UserInfo) => {
          p_userinfo.setgroups(groups_of_users.filter(item => item.id_user === p_userinfo.id).map(item => item.name))
        })
        LOG.ifInfo(() => usersinfo.findIndex((item: UserInfo) => item.groups?.length != undefined && item.groups.length > 0) != -1, 'groups of users', "the groups of users is add", usersinfo[0].groups)
        state["groups_of_users"] = true;
      } else if (!state["groups_of_users"]) {
        LOG.warn('groups of users', 'the groups of users is still did not fetched yet');
      } else {
        LOG.info('groups of users', "the groups of users all readdy added \n ->", usersinfo[0].groups)
      }
    }


    LOG.startTrace('users info', "that what you have in users info now");
    LOG.trace("", usersinfo.join(","));
    LOG.end();

    LOG.end();
    // console.groupEnd( )
  }


  useEffect(() => {

    LOG.startTrace('state', "state now is for");
    LOG.trace("", state);
    LOG.end();

  }, [state]);

  useEffect(() => {
    if (users.length == 0) {
      setUsersData();
    }
  }, [users]);
  
  useEffect(() => {
    if (users.length > 0 && levels.length == 0) {
      setLevelData(users);
    }
  }, [levels, users]);
  
  
  useEffect(() => {
    if (users.length > 0 && active_user.length == 0) {
      setisactivness();
    }
  }, [active_user, users]);



  useEffect(() => {
    if (users.length > 0 && types.length == 0) {
      setTypeData(users);
     }
  }, [types, users]);


  useEffect(() => {
    if (users.length > 0 && groups_of_users.length == 0) {
      setGroupsOfUsersData(users);
     }
  }, [groups_of_users, users]);


  useEffect(() => {
    if (usersinfo.length == 0 && users.length != 0) {
      users.map((p_user: User) => { usersinfo.push(new UserInfo(p_user, Array<string>(0))) })
    }
    else {
      if (users.length > 0) {
        fulfill_usersinfo();
      }
    }
  }, [users, levels, types, groups_of_users, usersinfo , active_user ,state])

 

  const columns: readonly Column[] = [
    {
      id: "name",
      label: "name",
      minWidth: 12,
      align: 'center'
    },
    {
      id: "family_name",
      label: "family name",
      minWidth: 12,
      align: 'center'
    },
    {
      id: "birth_day",
      label: "birth day",
      minWidth: 12,
      align: 'center'
    },
      {
        id: "level",
        label: "level",
        minWidth: 12,
        align: 'center',
        format: (value: number) => value.toLocaleString('en-US'),
      },
      {
        id: "groups",
        label: "groups",
        minWidth: 12,
        align: 'center',
        format: (value: string[]) => value.join(' | '),
      } 
      ,
      {
        id: "is_active",
        label: "is active nowe",
        minWidth: 12,
        align: 'center',
  
        format: (value: boolean) => {
          if (value == true) {
            return "✅";
          } else {
            return "❌";
          } 
        }   , 
      } 
  ]







  return <MyTable<UserInfo> data={usersinfo} columns={columns} name={`active users`}  />
  return (<div>
    <p>{`usersinfo  : ${usersinfo.length}  ${usersinfo[0]}`} </p>
    <p>{`users      : ${users.length}  ${users[0]?.name}`} </p>
    <p>{`groups_of_users      : ${groups_of_users.length}  ${groups_of_users[0]?.name}`} </p>
    <p>{`types      : ${types.length}  ${types[0]?.name}`} </p>
    <p>{`levels     : ${levels.length}  ${levels[0]?.name}`} </p></div>
  )   

}

