"use client";
import { open } from '@tauri-apps/plugin-dialog';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

import Image from "next/image";
import React from "react";

export default function DeviceOta() {
  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: "Firmware",
          directory: false,
          extensions: ["bin", "hex"],
        }],
      });

      if (selected) {
        console.log("Selected file:", selected);
        // 在这里可以添加处理选中文件的逻辑，例如上传或读取
      }
    } catch (error) {
      console.error("Error selecting file:", error);
    }
  };

  return (
    <div className="flex items-center justify-center h-full">
      <Card>
        <CardContent className="flex flex-col items-center justify-center">
          <Button
            variant="ghost"
            className="h-auto w-auto p-4"
            onClick={handleFileSelect}
          >
            <Image
              src="/file.svg"
              alt="选择文件"
              width={50}
              height={50}
              priority
            />
          </Button>
          <p className="text-sm text-muted-foreground mt-2">点击图标选择固件文件</p>
        </CardContent>
      </Card>
    </div>
  );
}