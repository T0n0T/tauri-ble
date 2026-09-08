"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";

interface ScanButtonProps {
  onScanToggle: (isScanning: number) => void;
}

export default function ScanButton({ onScanToggle }: ScanButtonProps) {
  const [isScanning, setIsScanning] = useState(0);

  const handleClick = () => {
    const newState = isScanning === 1 ? 2 : 1;
    setIsScanning(newState);
    onScanToggle(newState);
  };

  return (
    <Button onClick={handleClick} className="w-full">
      {isScanning === 1 ? "暂停扫描" : "开始扫描"}
    </Button>
  );
}
