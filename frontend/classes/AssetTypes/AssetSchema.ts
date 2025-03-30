export enum AssetType {
    Router = "Router",
    Switch = "Switch",
    Server = "Server",
    Desktop = "Desktop",
    Laptop = "Laptop",
    MobileDevice = "MobileDevice",
    IoTDevice = "IoTDevice",
}

export interface Properties {
    relations: string[];
    firewall_status: string;
    internet_facing: string;
}

export interface Asset {
    asset_type: AssetType;
    ip_address: string;
    properties: Properties;
}
