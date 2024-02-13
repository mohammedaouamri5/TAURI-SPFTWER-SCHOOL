import { MouseEventHandler } from "react";

 


interface ButtonProps {
  onClick:MouseEventHandler<HTMLButtonElement> ; 
  label:String;
  className:String;
} 




const Button = (props:ButtonProps) => {
  
  return (
    <button onClick={props.onClick} className={  "button " + props.className}  >
      {props.label}
    </button>
  );
};

export default Button;