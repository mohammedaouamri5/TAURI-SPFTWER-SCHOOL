

import { LOG } from "./LOG";
import { Group, All_The_Groups_Table, A_Groups_Table } from "./group";
import { All_User_Table } from "./user";


export enum ActionType {
  none, 
  all_the_group,
  a_group,
  all_the_user,
  all_the_active_user,
  a_user,


}


interface ActionProp {
  groups?: Group[];
  group_id?: number;
  action?: ActionType;
}

export default function Action({ groups: data, action:action, group_id: group_id }: ActionProp) {


    LOG.info('data',  "just get the data" , data)

  switch (action) {
    case ActionType.all_the_group:  if (data && data.length)    { return <All_The_Groups_Table data={data} />; }      break ;
    case ActionType.a_group:         if (group_id )             { return <A_Groups_Table group_id={group_id} />; }    break ; 
    case ActionType.all_the_user:     { return <All_User_Table />;    }    break ; 

    // case ActionType.all_the_active_user:
    //   return <All_The_Groups_Table data={data} />;
    // case ActionType.all_the_active_user:
    //   return <All_The_Groups_Table data={data} />;

 
  }
  return <p>fuch you</p>
}
//   