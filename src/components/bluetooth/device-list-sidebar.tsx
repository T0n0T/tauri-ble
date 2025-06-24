"use client";

import { useState, useEffect } from "react";
import { Sidebar } from "@/components/ui/sidebar";
import ScanButton from "./scan-button";
import DeviceCard from "./device-card";
import { BleDevice, startScan, connect, disconnect } from '@mnlphlp/plugin-blec';
interface DeviceListSidebarProps {
  onDeviceConnected: (deviceName: string) => void;
  onDeviceDisconnected: (deviceName: string) => void;
}

export default function DeviceListSidebar({ onDeviceConnected, onDeviceDisconnected }: DeviceListSidebarProps) {
  const [devices, setDevices] = useState<BleDevice[]>([]);
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const [connectedDeviceAddress, setConnectedDeviceAddress] = useState<string | null>(null);

  useEffect(() => {
    if (isScanning) {
      console.log("Starting BLE scan...", isScanning);
      startScan((newDevices) => {
        setDevices((prev) => {
          const updatedDevices = [...prev];
          newDevices.forEach((newDevice) => {
            if (!updatedDevices.some((d) => d.address === newDevice.address)
              && !(newDevice.name && newDevice.name.startsWith("hci0"))
              && !(newDevice.name && newDevice.name.startsWith("Unknown"))
            ) {
              updatedDevices.push(newDevice);
            }
          });
          return updatedDevices;
        });
      }, 0).catch((error) => {
        console.error("Failed to start scan:", error);
      });
    }
  }, [isScanning]);

  const handleScanToggle = async (scanning: boolean) => {
    setIsScanning(scanning);
    if (scanning) {
      setDevices([]);
    }
  };

  const handleDeviceSelect = async (deviceAddress: string) => {
    if (connectedDeviceAddress && connectedDeviceAddress !== deviceAddress) {
      try {
        await disconnect();
        console.log(`Disconnected from previous device: ${connectedDeviceAddress}`);
      } catch (error) {
        console.error(`Failed to disconnect from previous device ${connectedDeviceAddress}:`, error);
      }
    }

    setSelectedDeviceId(deviceAddress);
    const connectedDevice = devices.find(d => d.address === deviceAddress);
    if (connectedDevice) {
      try {
        await connect(deviceAddress, () => {
          console.log(`Disconnected from device: ${deviceAddress}`);
          setConnectedDeviceAddress(null); // Clear connected device on disconnect
          const deviceName = devices.find(d => d.address === deviceAddress)?.name || deviceAddress;
          onDeviceDisconnected(deviceName);
        });
        console.log(`Connected to device: ${deviceAddress}`);
        setConnectedDeviceAddress(deviceAddress);
        const deviceName = connectedDevice.name || connectedDevice.address;
        onDeviceConnected(deviceName);
      } catch (error) {
        console.error(`Failed to connect to device ${deviceAddress}:`, error);
        setConnectedDeviceAddress(null);
      }
    }
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