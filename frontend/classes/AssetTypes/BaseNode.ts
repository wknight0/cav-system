export interface NodeData {
    label: string;
    ip_address?: string;
    severity?: number | null;
    isHighlighted?: boolean;
    showHandles?: boolean;
    properties?: {
        operating_system?: string;
        internet_facing?: boolean;
        antivirus_status?: string;
        firewall_status?: string;
        firmware_status?: string;
        tamper_detection_status?: string;
        vpn_support?: boolean;
        vlan_support?: boolean;
        poe_support?: boolean;
        encryption_enabled?: boolean;
        virtualization_enabled?: boolean;
        network_enabled?: boolean;
        health_check_enabled?: boolean;
    };
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
