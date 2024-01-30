import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Sidebar from "./sidebare";
import Action from "./action";
import Button from "./button";
import { Group } from "./group";



function App() {

  const [data, setData] = useState<Group[]>([])
  const [show_groups_nams, set_show_groups_nams] = useState<boolean>(false)


  const group_button_set = () => {
    if(show_groups_nams )
    {
      setData([]); 
    }
    else {
      
      const m_data: Promise<Group[]> = invoke("get_all_the_groups", {})
      
      // console.log(m_data)
      m_data.then((groups: Group[]) => {
        setData(groups)
      });
    }
    set_show_groups_nams(!show_groups_nams);
  };



  return (
    <div className="container">
      <div className="sidebare">
        <Button onClick={group_button_set} label="cleckme" className={"sidebare_button"} />
        {
          show_groups_nams  ? (
            data.map((group) => (
              <Button onClick={group_button_set} label={String(group?.name)} className={"sidebare_button"} />
            ))
          ) : <p>fuck you</p>

        }
        <Sidebar /></div>
      <div className="action"> <Action data={data} /> </div>
    </div>
  );
}

export default App;
