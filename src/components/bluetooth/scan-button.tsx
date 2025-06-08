"use client";

import { BleDevice, startScan, stopScan, getScanningUpdates } from '@mnlphlp/plugin-blec'
import { Button } from "@/components/ui/button";
import { useState } from "react";

interface ScanButtonProps {
  onScanToggle: (isScanning: boolean) => void;
}

export default function ScanButton({ onScanToggle }: ScanButtonProps) {
  const [isScanning, setIsScanning] = useState(false);

  const handleClick = () => {
    const newState = !isScanning;
    setIsScanning(newState);
    onScanToggle(newState);
    // startScan((dev: BleDevice[]) => devices = dev, 10000)
  };

  return (
    <Button onClick={handleClick} className="w-full">
      {isScanning ? "暂停扫描" : "开始扫描"}
    </Button>
  );
}