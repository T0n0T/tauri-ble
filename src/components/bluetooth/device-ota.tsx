"use client";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { error } from "@tauri-apps/plugin-log";
import Image from "next/image";
import { useEffect } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { useOtaProgress } from "@/context/OtaProgressContext";

export default function DeviceOta() {
  const { otaProgress, setOtaProgress, otaInProgress, setOtaInProgress } =
    useOtaProgress();

  useEffect(() => {
    const unlistenProgress = listen("ota_progress", (event) => {
      setOtaProgress(event.payload as number);
      if (event.payload === 100) {
        toast.error(`OTA Sucess`);
        setOtaInProgress(false);
      }
    });

    const unlistenError = listen("ota_error", (event) => {
      toast.error(`OTA Error: ${event.payload}`);
      setOtaInProgress(false);
      setOtaProgress(0); // Reset progress on error
    });

    return () => {
      unlistenProgress.then((f) => f());
      unlistenError.then((f) => f());
    };
  }, [setOtaInProgress, setOtaProgress]);

  const handleFileSelect = async () => {
    setOtaInProgress(true);
    setOtaProgress(0);
    try {
      await invoke("start_valve_ota");
    } catch (invokeError) {
      error(`Failed to start OTA: ${invokeError}`);
      toast.error(`Failed to start OTA: ${invokeError}`);
      setOtaInProgress(false);
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <Card className="w-full max-w-sm">
        <CardContent className="flex flex-col items-center justify-center">
          <Button
            variant="ghost"
            className="h-auto w-auto p-4"
            onClick={handleFileSelect}
            disabled={otaInProgress}
          >
            <Image
              src="/file.svg"
              alt="选择文件"
              width={50}
              height={50}
              priority
            />
          </Button>
          <p className="text-sm text-muted-foreground mt-2">
            点击图标选择固件文件
          </p>
          {otaInProgress && (
            <div className="mt-4 w-full">
              <Progress value={otaProgress} className="w-full" />
              <p className="text-sm text-gray-500 mt-2 text-center">
                升级进度: {otaProgress}%
              </p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
