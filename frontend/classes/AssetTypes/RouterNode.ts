import BaseNode from './BaseNode';

export default class RouterNode extends BaseNode {
    constructor(id: string, position: { x: number, y: number }, label: string) {
        super(id, 'router', position, { label });
    }

    getIcon(): string {
        return '@assets/icons/router.svg';
    }
}
