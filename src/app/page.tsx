"use client";

import { useEffect, useState } from "react";
import { SidebarProvider, useSidebar } from "@/components/ui/sidebar";
import DeviceListSidebar from "@/components/bluetooth/device-list-sidebar";
import DeviceDetailsView from "@/components/bluetooth/device-details-view";
import { Toaster, toast } from "sonner";
import { listen } from "@tauri-apps/api/event";

interface BleStatus {
  name: string;
  address: string;
  isconnected: boolean;
}

export default function Home() {
  return (
    <SidebarProvider>
      <LayoutContent />
    </SidebarProvider>
  );
}

function LayoutContent() {
  const { setOpenMobile } = useSidebar();
  const [connectedDeviceName, setConnectedDeviceName] = useState<string | null>(null);
  useEffect(() => {
    const connectUnlisten = listen("ble_status", (event) => {
      const device = event.payload as BleStatus;
      if (device.isconnected) {
        toast.success(`已连接到设备: ${device.name}(${device.address})`);
        setConnectedDeviceName(device.name.toString());
        setOpenMobile(false); // Close mobile sidebar
      } else {
        toast.warning(`已从设备断开连接: ${device.name}(${device.address})`);
        setConnectedDeviceName(null);
      }
    });

    return () => {
      connectUnlisten.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <>
      <div className="flex h-screen w-screen">
        <DeviceListSidebar />
        <div className="flex flex-col flex-grow">
          <DeviceDetailsView deviceName={connectedDeviceName} />
        </div>
      </div>
      <Toaster position="top-center" />
    </>
  );
}