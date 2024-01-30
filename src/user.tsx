




export class User implements IUser {
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

export interface IUser {
    id: number;
    id_type: number;
    name: string;
    family_name: string;
    birth_day: string;
    notes: string;
}
