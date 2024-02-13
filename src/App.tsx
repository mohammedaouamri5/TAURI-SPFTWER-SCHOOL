import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Sidebar from "./sidebare";
import Action, { ActionType } from "./action";
import Button from "./button";
import { Group } from "./group";
import { LOG } from "./LOG"
import Slider from 'react-slider';
import ReactSlider from "react-slider";

function App() {


 


 
  const [groups, set_groups] = useState<Group[]>([])
  const [show_groups_nams, set_show_groups_nams] = useState<boolean>(false)
  const [action, set_action] = useState<ActionType | undefined>(0)
  const [group_id, set_group_id] = useState<number | undefined>(0)


  const group_button_set = () => {
    if (show_groups_nams) {
      set_action(ActionType.none);
      LOG.info('groups', "groups are already detected", groups);
    }
    else {
      if (groups.length == 0) {
        const m_data: Promise<Group[]> = invoke("get_all_the_groups", {})
        console.log(m_data)
        m_data.then((p_groups: Group[]) => {
          set_groups(p_groups)
        });
      }
      set_action(ActionType.all_the_group);
    }
    set_show_groups_nams(!show_groups_nams);
  };
 
  return (

    <div className="container">   
      <div className="sidebare">
        <Button onClick={group_button_set} label="all the groups" className={"sidebare_button"} />
        {
          show_groups_nams ? groups.map((group) => (
            <Button onClick={() => { set_action(ActionType.a_group); set_group_id(group.id) }} label={String(group?.name)} className={"sidebare_button"} />
          )) : null
        }

        <Button onClick={() => set_action(ActionType.all_the_user)} label="all the users" className={"sidebare_button"} />
        <Sidebar />



      </div>
      <div className="action">  
        <Action group_id={group_id} groups={groups} action={action} />
      </div>

    </div>


  );
}

export default App;
