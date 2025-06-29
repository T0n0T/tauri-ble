"use client";

import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { info, error } from '@tauri-apps/plugin-log';
import { useEffect, useState } from "react";
import { Separator } from "@/components/ui/separator";

interface ValveInfo {
    total_ticks: number;
    position: number;
}

export default function DeviceInfo() {
    const [valveInfo, setValveInfo] = useState<ValveInfo>({
        total_ticks: 0,
        position: 0,
    });
    const setup = async () => {
        try {
            await invoke<ValveInfo>('start_valve_info');
            info('start_valve_info invoked');
        } catch (e) {
            error(`Error invoking get_valve_info: ${e}`);
        }
    };
    // 类似 Vue 的 mounted + updated（依赖 count）
    useEffect(() => {
        // 监听事件
        const unlisten = listen('valve_info', (event) => {
            const data = event.payload as ValveInfo;
            setValveInfo(data);
        });

        setup();

        return () => {
            unlisten.then((f) => f());
            invoke('stop_valve_info')
                .then(() => info('stop_valve_info invoked'))
                .catch((e) => error(`Error invoking stop_valve_info: ${e}`));
        };
    }, []); // 依赖项为 count

    return (
        <div className="flex flex-row items-center justify-around w-full h-full p-4 space-x-1">
            <div className="flex flex-col items-center gap-4">
                <p className="text-3xl font-semibold">角度</p>
                <div className="text-4xl text-blue-400">{valveInfo.total_ticks * 60}</div>
            </div>
            <Separator orientation="vertical" />
            <div className="flex flex-col items-center gap-5">
                <p className="text-5xl font-semibold">圈数</p>
                <div className="text-6xl text-blue-400">{Math.floor(valveInfo.total_ticks / 6)}</div>
            </div>
            <Separator orientation="vertical" />
            <div className="flex flex-col items-center gap-4">
                <p className="text-3xl font-semibold">位置</p>
                <div className="text-4xl text-blue-400">{valveInfo.position * 60}</div>
            </div>
        </div>
    )
}