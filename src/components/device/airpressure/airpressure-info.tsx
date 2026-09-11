"use client";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { error, info } from "@tauri-apps/plugin-log";
import { useEffect, useState } from "react";
import { enqueueRealtimeCommand } from "@/lib/realtime-command";
import type { AirPressureVal } from "@/types/airpressure";

export default function AirPressureInfo() {
  const [airPressureInfo, setAirPressureInfo] = useState<AirPressureVal>({
    current_pressure: 0,
  });
  useEffect(() => {
    let active = true;
    const setup = enqueueRealtimeCommand(async () => {
      if (!active) return;

      try {
        await invoke<AirPressureVal>("start_airpressure_info");
        if (active) info("start_airpressure_info invoked");
      } catch (e) {
        error(`Error invoking start_airpressure_info: ${e}`);
      }
    });

    // 监听事件
    const unlisten = listen("airpressure_info", (event) => {
      if (!active) return;
      const data = event.payload as AirPressureVal;
      setAirPressureInfo(data);
    });

    return () => {
      active = false;
      void enqueueRealtimeCommand(async () => {
        try {
          (await unlisten)();
        } catch (e) {
          error(`Error removing airpressure_info listener: ${e}`);
        }

        try {
          await setup;
        } finally {
          try {
            await invoke("stop_airpressure_info");
            info("stop_airpressure_info invoked");
          } catch (e) {
            error(`Error invoking stop_airpressure_info: ${e}`);
          }
        }
      }).catch((e) => error(`Error cleaning up airpressure_info: ${e}`));
    };
  }, []);

  return (
    <div className="flex flex-row items-center justify-around w-full h-full p-4 space-x-1">
      <div className="flex flex-col items-center gap-4">
        <p className="text-3xl font-semibold">压力值</p>
        <div className="text-4xl text-blue-400">
          {((airPressureInfo.current_pressure * 1.6) / 2000).toFixed(4)} MPa
        </div>
      </div>
    </div>
  );
}
