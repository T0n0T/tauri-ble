"use client";

import { SidebarTrigger } from "@/components/ui/sidebar";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import React from "react";
import ValveForm from "@/forms/valve-form";

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
    <main className="flex-grow flex flex-col items-center justify-center">
      {deviceName ?
        (<Tabs defaultValue="command" className="w-full h-full flex flex-col">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="command">配置</TabsTrigger>
            <TabsTrigger value="ota">OTA</TabsTrigger>
          </TabsList>
          <TabsContent value="command" className="flex-grow mt-4">
            <ValveForm deviceName={deviceName} />
          </TabsContent>
          <TabsContent value="ota" className="flex-grow mt-4">
          </TabsContent>
        </Tabs>) : (
          <div className="flex flex-col items-center justify-center flex-grow">
            <h1 className="text-4xl font-bold">欢迎来到蓝牙设备管理系统</h1>
            <p className="text-lg text-gray-600 mt-4">请从左侧边栏选择一个设备进行管理。</p>
          </div>
        )}
      </main>
    </>
  );
}