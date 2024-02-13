





export class Level {
    public id?:number;
    public setid(id:number)  {this.id = id} 

    public name?:string ; 
    public setname(name:string)  {this.name = name} 

    public toString() {
        return `\n{
            \n id: ${this.id ? this.id : "undefined"},
            \n name: ${this.name ? this.name : "undefined"},
        \n}`;
      }
    
}