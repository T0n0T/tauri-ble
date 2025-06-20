"use client";

import { useState } from "react";
import { SidebarProvider, useSidebar } from "@/components/ui/sidebar";
import DeviceListSidebar from "@/components/bluetooth/device-list-sidebar";
import DeviceDetailsView from "@/components/bluetooth/device-details-view";
import { Toaster, toast } from "sonner";

export default function Home() {
  return (
    <SidebarProvider>
      <LayoutContent></LayoutContent>
    </SidebarProvider>
  );
}

function LayoutContent() {
  const { setOpenMobile } = useSidebar();
  const [selectedDeviceName, setSelectedDeviceName] = useState<string | null>(null);

  const handleDeviceConnected = (deviceName: string) => {
    setSelectedDeviceName(deviceName);
    setOpenMobile(false); // Close mobile sidebar
    toast.success(`已连接到设备: ${deviceName}`);
  };

  const handleDeviceDisconnected = (deviceName: string) => {
    toast.warning(`已从设备断开连接: ${deviceName}`);
  };

  return (
    <>
      <div className="flex h-screen w-screen">
        <DeviceListSidebar onDeviceConnected={handleDeviceConnected} onDeviceDisconnected={handleDeviceDisconnected} />
        <div className="flex flex-col flex-grow">
          <DeviceDetailsView deviceName={selectedDeviceName} />
        </div>
      </div>
      <Toaster position="top-center" />
    </>
  );
}