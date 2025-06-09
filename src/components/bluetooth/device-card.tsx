import { Card, CardContent } from "@/components/ui/card";
import { BluetoothIcon } from "lucide-react"; // Placeholder icon

interface DeviceCardProps {
    deviceName: string;
    macAddress: string;
    rssi: number;
    deviceType: string; // e.g., "BLE", "Classic"
    isSelected: boolean;
    onSelect: () => void;
}

export default function DeviceCard({
    deviceName,
    macAddress,
    rssi,
    deviceType,
    isSelected,
    onSelect,
}: DeviceCardProps) {
    return (
        <Card
            className={`py-0 cursor-pointer ${isSelected ? "bg-accent" : ""}`}
            onClick={onSelect}
        >
            <CardContent className="p-2 flex items-center space-x-1">
                <BluetoothIcon className="h-8 w-8 text-blue-500" /> {/* Device type icon */}
                <div className="flex-grow">
                    <p className="text-lg font-semibold">{deviceName}</p>
                    <p className="text-sm text-gray-500">MAC: {macAddress}</p>
                    <p className="text-sm text-gray-500">RSSI: {rssi} dBm</p>
                </div>
            </CardContent>
        </Card>
    );
}