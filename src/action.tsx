

import All_The_Groups_Table, { Group } from "./group";

interface ActionProp {
  data: Group[]
}

export default function Action({ data }: ActionProp) {

  if (data.length) {

    return <All_The_Groups_Table data={data} />;
  } return <div></div>;

}
//   