"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useParams } from "next/navigation";
import React from "react";
import ValveForm from "@/forms/valve-form";

export const dynamic = 'force-dynamic';

export default function DevicePage() {
  const params = useParams();
  const deviceName = String(params.deviceName || "");

  return (
    <main className="flex-grow p-4">
      <Tabs defaultValue="command" className="w-full h-full flex flex-col">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="command">配置</TabsTrigger>
          <TabsTrigger value="ota">OTA</TabsTrigger>
        </TabsList>
        <TabsContent value="command" className="flex-grow mt-4">
          <ValveForm deviceName={deviceName} />
        </TabsContent>
        <TabsContent value="ota" className="flex-grow mt-4">
        </TabsContent>
      </Tabs>
    </main>
  );
}