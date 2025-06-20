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
            className={`py-0 cursor-pointer ${isSelected ? "bg-accent outline-blue-300 outline-3 outline-offset-1" : ""}`}
            onClick={onSelect}
        >
            <CardContent className="p-2 flex items-center space-x-2 overflow-hidden">
                <BluetoothIcon className="h-8 w-8 text-blue-500 flex-shrink-0" /> {/* Device type icon */}
                <div className="flex-grow min-w-0">
                    <p className="text-lg font-semibold truncate">{deviceName}</p>
                    <p className="text-sm text-gray-500 truncate">MAC: {macAddress}</p>
                    <p className="text-sm text-gray-500">RSSI: {rssi} dBm</p>
                </div>
            </CardContent>
        </Card>
    );
}