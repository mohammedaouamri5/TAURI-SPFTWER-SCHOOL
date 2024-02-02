





export class Level {
    public id?:number;
    public name?:string ; 
    public toString() {
        return `\n{
            \n id: ${this.id ? this.id : "undefined"},
            \n name: ${this.name ? this.name : "undefined"},
        \n}`;
      }
    
}