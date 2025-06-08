"use client";

import { useState, useEffect } from "react";
import { Sidebar } from "@/components/ui/sidebar";
import ScanButton from "./scan-button";
import DeviceCard from "./device-card";
import { BleDevice, startScan, stopScan, getScanningUpdates } from '@mnlphlp/plugin-blec';

interface Device extends BleDevice {}

interface DeviceListSidebarProps {
  onDeviceConnected: (deviceName: string) => void;
}

export default function DeviceListSidebar({ onDeviceConnected }: DeviceListSidebarProps) {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(false);

  useEffect(() => {
    if (isScanning) {
      // Start scanning and provide a handler to receive device updates
      startScan((newDevices) => {
        setDevices((prev) => {
          const updatedDevices = [...prev];
          newDevices.forEach((newDevice) => {
            // Check if device already exists by address
            if (!updatedDevices.some((d) => d.address === newDevice.address)) {
              updatedDevices.push(newDevice);
            }
          });
          return updatedDevices;
        });
      }, 0); // Timeout 0 means scan indefinitely until stopScan is called
    } else {
      // Stop scanning when isScanning is false
      stopScan();
    }

    // Cleanup function to stop scan when component unmounts or isScanning changes
    return () => {
      stopScan();
    };
  }, [isScanning]);

  const handleScanToggle = async (scanning: boolean) => {
    setIsScanning(scanning);
    if (scanning) {
      setDevices([]); // Clear devices when starting a new scan
    }
  };

  const handleDeviceSelect = (deviceAddress: string) => {
    setSelectedDeviceId(deviceAddress);
    // Simulate connection success after selection
    setTimeout(() => {
      console.log(`Connecting to device: ${deviceAddress}`);
      // In a real app, this would involve actual Bluetooth connection logic
      // On successful connection, call onDeviceConnected with the device name
      const connectedDevice = devices.find(d => d.address === deviceAddress);
      if (connectedDevice) {
        onDeviceConnected(connectedDevice.name || connectedDevice.address); // Use device.name or fallback to address
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
            key={device.address}
            deviceName={device.name || "Unknown Device"}
            macAddress={device.address}
            rssi={device.rssi}
            deviceType={"BLE"} // Assuming all devices are BLE for now
            isSelected={selectedDeviceId === device.address}
            onSelect={() => handleDeviceSelect(device.address)}
          />
        ))}
      </div>
      <div className="p-4 border-t">
        <ScanButton onScanToggle={handleScanToggle} />
      </div>
    </Sidebar>
  );
}