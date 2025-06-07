"use client";

import { useParams } from "next/navigation";
import React from "react";

export const dynamic = 'force-dynamic';

export default function DevicePage() {
  const params = useParams();
  const deviceName = params.deviceName;

  return (
    <div className="flex flex-col items-center justify-center h-full">
      <h1 className="text-2xl font-bold">
        当前连接设备: {deviceName}
      </h1>
      <p className="text-gray-500">
        这里将显示与设备相关的详细信息和控制选项。
      </p>
    </div>
  );
}