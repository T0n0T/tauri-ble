"use client";

import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator";
import Image from "next/image";
interface ValveInfo {
    total_ticks: number;
    position: number;
    rotation: number;
}

interface DeviceInfoProps {
    activeTab: string;
}

export default function DeviceInfo({ activeTab }: DeviceInfoProps) {
    const [valveInfo, setValveInfo] = useState<ValveInfo>({
        total_ticks: 0,
        position: 0,
        rotation: 0,
    });

    // 类似 Vue 的 mounted + updated（依赖 count）
    useEffect(() => {
        console.log('组件挂载');
        // 监听事件
        const unlisten = listen('valve_info', (event) => {
            const data = event.payload as ValveInfo;
            console.log('接收到事件:', data);
            setValveInfo(data);
        });

        invoke('start_valve_info')
            .catch((error) => console.error('Error invoking start_valve_info:', error));
        
        return () => {
            console.log('组件销毁前清理');
            unlisten.then((f) => f());
            invoke('stop_valve_info')
                .catch((error) => console.error('Error invoking stop_valve_info:', error));
        };
    }, []); // 依赖项为 count

    return (
        <div className="flex flex-row items-center justify-around w-full h-full p-4 space-x-1">
            <div className="flex flex-col items-center gap-4">
                <p className="text-3xl font-semibold">角度</p>
                <div className="text-4xl text-blue-400">{valveInfo.total_ticks}</div>
            </div>
            <Separator orientation="vertical" />
            <div className="flex flex-col items-center gap-5">
                <p className="text-5xl font-semibold">圈数</p>
                <div className="text-6xl text-blue-400">{valveInfo.rotation}</div>
            </div>
            <Separator orientation="vertical" />
            <div className="flex flex-col items-center gap-4">
                <p className="text-3xl font-semibold">位置</p>
                <div className="text-4xl text-blue-400">{valveInfo.position}</div>
            </div>
        </div>
    )
}