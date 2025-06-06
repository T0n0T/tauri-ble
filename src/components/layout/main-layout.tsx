"use client";

import { useState } from "react";
import { SidebarProvider, Sidebar, useSidebar } from "@/components/ui/sidebar";
import DeviceListSidebar from "@/components/bluetooth/device-list-sidebar";
import MainHeader from "@/components/layout/main-header";
import MainContent from "@/components/layout/main-content";

export default function MainLayout() {
  // No need for isSidebarOpen state here anymore, as SidebarProvider handles it.
  // We will use useSidebar hook to get toggleSidebar function.

  return (
    <SidebarProvider>
      <LayoutContent />
    </SidebarProvider> 
  );
}

function LayoutContent() {
  const { toggleSidebar, setOpenMobile } = useSidebar();
  const [selectedDeviceName, setSelectedDeviceName] = useState<string | null>(null);

  const handleDeviceConnected = (deviceName: string) => {
    setSelectedDeviceName(deviceName);
    setOpenMobile(false); // Close mobile sidebar
    // For desktop, if sidebar is collapsible, it might also close here
    // For now, just handle mobile sheet
  };

  return (
    <div className="flex h-screen w-screen">
      <Sidebar variant="sidebar">
        <DeviceListSidebar onDeviceConnected={handleDeviceConnected} />
      </Sidebar>
      <div className="flex flex-col flex-grow">
        <MainHeader selectedDeviceName={selectedDeviceName} />
        <MainContent />
      </div>
    </div>
  );
}