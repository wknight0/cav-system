export interface NodeData {
    label: string;
    [key: string]: any;
}
  
export default abstract class BaseNode {
    id: string;
    type: string;
    position: { x: number; y: number };
    data: NodeData;
  
    constructor(id: string, type: string, position: { x: number; y: number }, data: NodeData) {
        this.id = id;
        this.type = type;
        this.position = position;
        this.data = data;
    }
  
    getDetails() {
        return {
            id: this.id,
            type: this.type,
            position: this.position,
            data: this.data,
        };
    }
}
