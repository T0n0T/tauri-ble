"use client";

import { useState, useEffect } from "react";
import { Sidebar } from "@/components/ui/sidebar";
import ScanButton from "./scan-button";
import DeviceCard from "./device-card";

interface Device {
  id: string;
  deviceName: string;
  macAddress: string;
  rssi: number;
  deviceType: string;
}

interface DeviceListSidebarProps {
  onDeviceConnected: (deviceName: string) => void;
}

export default function DeviceListSidebar({ onDeviceConnected }: DeviceListSidebarProps) {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(false);

  // Simulate scanning for devices
  useEffect(() => {
    if (isScanning) {
      const interval = setInterval(() => {
        const newDevice: Device = {
          id: `device-${Math.random()}`,
          deviceName: `Device ${Math.floor(Math.random() * 100)}`,
          macAddress: `XX:XX:XX:XX:${Math.floor(Math.random() * 100).toString(16).padStart(2, '0')}:${Math.floor(Math.random() * 100).toString(16).padStart(2, '0')}`,
          rssi: -Math.floor(Math.random() * 100),
          deviceType: "BLE",
        };
        setDevices((prev) => {
          // Avoid adding duplicate devices for simplicity in simulation
          if (!prev.some(d => d.macAddress === newDevice.macAddress)) {
            return [...prev, newDevice];
          }
          return prev;
        });
      }, 2000); // Add a new device every 2 seconds
      return () => clearInterval(interval);
    }
  }, [isScanning]);

  const handleScanToggle = (scanning: boolean) => {
    setIsScanning(scanning);
    if (scanning) { // Only clear devices when starting a new scan
      setDevices([]);
    }
  };

  const handleDeviceSelect = (deviceId: string) => {
    setSelectedDeviceId(deviceId);
    // Simulate connection success after selection
    setTimeout(() => {
      console.log(`Connecting to device: ${deviceId}`);
      // In a real app, this would involve actual Bluetooth connection logic
      // On successful connection, call onDeviceConnected with the device name
      const connectedDevice = devices.find(d => d.id === deviceId);
      if (connectedDevice) {
        onDeviceConnected(connectedDevice.deviceName);
      }
    }, 1000);
  };

  return (
    <Sidebar className="flex flex-col h-full">
      <div className="flex-grow overflow-y-auto p-4 space-y-2">
        <h2 className="text-lg font-semibold mb-4">Available Devices</h2>
        {devices.length === 0 && !isScanning && (
          <p className="text-gray-500">点击“开始扫描”查找设备。</p>
        )}
        {isScanning && devices.length === 0 && (
          <p className="text-gray-500">正在扫描设备...</p>
        )}
        {devices.map((device) => (
          <DeviceCard
            key={device.id}
            deviceName={device.deviceName}
            macAddress={device.macAddress}
            rssi={device.rssi}
            deviceType={device.deviceType}
            isSelected={selectedDeviceId === device.id}
            onSelect={() => handleDeviceSelect(device.id)}
          />
        ))}
      </div>
      <div className="p-4 border-t">
        <ScanButton onScanToggle={handleScanToggle} />
      </div>
    </Sidebar>
  );
}