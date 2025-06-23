"use client";

import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

interface ValveInfo {
    total_ticks: number;
    position: number;
    rotation: number;
}

export default function DeviceInfo() {
    const [valveInfo, setValveInfo] = useState<ValveInfo>({
        total_ticks: 0,
        position: 0,
        rotation: 0,
    });

    useEffect(() => {
        let unlisten: (() => void) | undefined;

        const setup = async () => {
            try {
                await invoke('start_valve_info');
                console.log('start_valve_info invoked');

                unlisten = await listen<ValveInfo>('valve_info_update', (event) => {
                    console.log('Received valve_info_update:', event.payload);
                    setValveInfo(event.payload);
                });
            } catch (error) {
                console.error('Error in DeviceInfo useEffect setup:', error);
            }
        };

        setup();

        return () => {
            if (unlisten) {
                unlisten();
                console.log('unlisten called');
            }
            invoke('stop_valve_info')
                .then(() => console.log('stop_valve_info invoked'))
                .catch((error) => console.error('Error invoking stop_valve_info:', error));
        };
    }, []);

    return (
        <div>
            <p>Total Ticks: {valveInfo.total_ticks}</p>
            <p>Position: {valveInfo.position}</p>
            <p>Rotation: {valveInfo.rotation}</p>
        </div>
    )
}