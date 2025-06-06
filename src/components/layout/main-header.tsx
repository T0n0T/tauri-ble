import { SidebarTrigger } from "@/components/ui/sidebar";

interface MainHeaderProps {
  selectedDeviceName: string | null;
}

export default function MainHeader({ selectedDeviceName }: MainHeaderProps) {
  return (
    <header className="flex items-center justify-between p-4 border-b">
      <SidebarTrigger />
      <h1 className="text-lg font-semibold">{selectedDeviceName || "未选择设备"}</h1>
      <div>{/* Placeholder for potential right-side elements */}</div>
    </header>
  );
}