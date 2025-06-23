"use client";

import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

export default function DeviceInfo() {
    return (
        <div className="flex flex-col items-center justify-center flex-grow px-6">
            <h2 className="text-2xl font-bold mb-4">设备信息</h2>
            <p className="text-gray-600">此处将显示设备的实时数据和状态信息。</p>
            {/* 这里可以添加更多的设备信息展示逻辑 */}
        </div>
    )
}