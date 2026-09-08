"use client";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { error, info } from "@tauri-apps/plugin-log";
import { useEffect, useState } from "react";
import { Separator } from "@/components/ui/separator";
import { enqueueRealtimeCommand } from "@/lib/realtime-command";
import type { ValveVal } from "@/types/valve";

export default function ValveInfo() {
  const [valveInfo, setValveInfo] = useState<ValveVal>({
    total_ticks: 0,
    current_status: 0,
  });
  useEffect(() => {
    let active = true;
    const setup = enqueueRealtimeCommand(async () => {
      if (!active) return;

      try {
        await invoke<ValveVal>("start_valve_info");
        if (active) info("start_valve_info invoked");
      } catch (e) {
        error(`Error invoking get_valve_info: ${e}`);
      }
    });

    // 监听事件
    const unlisten = listen("valve_info", (event) => {
      if (!active) return;
      const data = event.payload as ValveVal;
      setValveInfo(data);
    });

    return () => {
      active = false;
      void enqueueRealtimeCommand(async () => {
        try {
          (await unlisten)();
        } catch (e) {
          error(`Error removing valve_info listener: ${e}`);
        }

        await setup;
        try {
          await invoke("stop_valve_info");
          info("stop_valve_info invoked");
        } catch (e) {
          error(`Error invoking stop_valve_info: ${e}`);
        }
      }).catch((e) => error(`Error cleaning up valve_info: ${e}`));
    };
  }, []);

  return (
    <div className="flex flex-row items-center justify-around w-full h-full p-4 space-x-1">
      <div className="flex flex-col items-center gap-4">
        <p className="text-3xl font-semibold">角度</p>
        <div className="text-4xl text-blue-400">
          {valveInfo.total_ticks * 12}
        </div>
      </div>
      <Separator orientation="vertical" />
      <div className="flex flex-col items-center gap-4">
        <p className="text-5xl font-semibold">状态</p>
        <div className="text-6xl text-blue-400">
          {valveInfo.current_status === 0 && <p className="text-red-400">关</p>}
          {valveInfo.current_status === 1 && (
            <p className="text-green-400">开</p>
          )}
        </div>
      </div>
      <Separator orientation="vertical" />
      <div className="flex flex-col items-center gap-5">
        <p className="text-3xl font-semibold">圈数</p>
        <div className="text-4xl text-blue-400">
          {Math.floor(valveInfo.total_ticks / 30)}
        </div>
      </div>
    </div>
  );
}
