export default abstract class BaseEdge {
    id: string;
    source: string;
    target: string;
    label: string;
  
    constructor(id: string, source: string, target: string, label: string = '') {
        this.id = id;
        this.source = source;
        this.target = target;
        this.label = label;
    }

    getDetails() {
        return {
            id: this.id,
            source: this.source,
            target: this.target,
            label: this.label,
        };
    }
}
