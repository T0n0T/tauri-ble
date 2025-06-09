"use client";

import { useState } from "react";
import { SidebarProvider, useSidebar } from "@/components/ui/sidebar";
import DeviceListSidebar from "@/components/bluetooth/device-list-sidebar";
import DeviceDetailsView from "@/components/bluetooth/device-details-view";

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
    // For desktop, if sidebar is collapsible, it might also close here
    // For now, just handle mobile sheet
    
  };

  return (
    <div className="flex h-screen w-screen">
      <DeviceListSidebar onDeviceConnected={handleDeviceConnected} />
      <div className="flex flex-col flex-grow">
        <DeviceDetailsView deviceName={selectedDeviceName} />
      </div>
    </div>
  );
}