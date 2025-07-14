"use client";

import { invoke } from '@tauri-apps/api/core';
import Image from "next/image";
import { SidebarTrigger } from "@/components/ui/sidebar";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Label } from "@/components/ui/label";
import { OtaProgressProvider } from "@/context/OtaProgressContext";
import DeviceOta from "@/components/bluetooth/device-ota";
import ValveInfo from "@/components/device/valve/valve-info";
import ValveConfig from "@/components/device/valve/valve-conig";
import { toast } from 'sonner';
import { useCallback } from 'react';

interface DeviceDetailsViewProps {
  deviceName: string | null;
}

export default function DeviceDetailsView({ deviceName }: DeviceDetailsViewProps) {
  return (
    <>
      <header className="flex items-center justify-between p-4 border-b">
        <SidebarTrigger />
        <h1 className="text-lg font-semibold">{deviceName || "未选择设备"}</h1>
        <div></div>
      </header>
      <main className="p-2 flex-grow flex flex-col items-center justify-center relative">
        {deviceName ?
          (<OtaProgressProvider>
            <Tabs defaultValue="command" className="w-full h-full flex flex-col">
              <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger value="command">配置</TabsTrigger>
                <TabsTrigger value="ota">OTA</TabsTrigger>
                <TabsTrigger value="info">实时数据</TabsTrigger>
              </TabsList>
              <TabsContent value="command" className="flex-grow mt-4">
                <ValveConfig deviceName={deviceName} />
              </TabsContent>
              <TabsContent value="ota" className="flex-grow mt-4">
                <DeviceOta></DeviceOta>
              </TabsContent>
              <TabsContent value="info" className="flex-grow mt-4">
                <ValveInfo></ValveInfo>
              </TabsContent>
            </Tabs>
            <button
              onClick={() => {
                invoke('reboot_valve')
                  .then(() => {
                    toast.success('设备重启命令已发送，请稍候');
                  })
                  .catch((error) => {
                    toast.error(`发送设备重启命令失败，请稍后重试: ${error}`);
                  });
              }}
              className="absolute bottom-6 right-4 p-3 bg-blue-500 text-white rounded-full shadow-lg"
            >
              <Image
                src="/reset.svg"
                alt="重启设备"
                width={24}
                height={24}
                priority
              />
            </button>
          </OtaProgressProvider>) : (
            <div className="flex flex-col items-center justify-center flex-grow px-6">
              <Label className="text-center text-3xl font-bold">
                欢迎来到蓝牙设备管理系统
              </Label>
              <Label className="text-lg text-gray-600 mt-4">
                请从左侧边栏选择一个设备进行管理
              </Label>
            </div>
          )}
      </main>
    </>
  );
}