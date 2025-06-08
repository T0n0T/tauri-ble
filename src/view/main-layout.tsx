"use client";

import { useState } from "react";
import { SidebarProvider, Sidebar, useSidebar } from "@/components/ui/sidebar";
import DeviceListSidebar from "@/components/bluetooth/device-list-sidebar";
import MainHeader from "@/view/main-header";
import { useRouter } from "next/navigation";

export default function MainLayout({ children }: { children: React.ReactNode }) {
  return (
    <SidebarProvider>
      <LayoutContent>{children}</LayoutContent>
    </SidebarProvider>
  );
}

function LayoutContent({ children }: { children: React.ReactNode }) {
  const { toggleSidebar, setOpenMobile } = useSidebar();
  const [selectedDeviceName, setSelectedDeviceName] = useState<string | null>(null);
  const router = useRouter();

  const handleDeviceConnected = (deviceName: string) => {
    setSelectedDeviceName(deviceName);
    setOpenMobile(false); // Close mobile sidebar
    // For desktop, if sidebar is collapsible, it might also close here
    // For now, just handle mobile sheet
    router.push(`/${deviceName}`); // 恢复路由跳转，但路径改为 /${deviceType}/${deviceName}
  };

  return (
    <div className="flex h-screen w-screen">
      <Sidebar variant="sidebar">
        <DeviceListSidebar onDeviceConnected={handleDeviceConnected} />
      </Sidebar>
      <div className="flex flex-col flex-grow">
        <MainHeader selectedDeviceName={selectedDeviceName} />
        {/* 这里是子页面内容的占位符，实际内容由 Next.js 路由加载 */}
        <div className="flex-grow p-4">
          {children} {/* 渲染子路由内容 */}
        </div>
      </div>
    </div>
  );
}