import { Button } from "@/components/ui/button";
import { FolderIcon } from "lucide-react"; // Assuming lucide-react is installed for icons

export default function OtaPage() {
  return (
    <div className="flex flex-col items-center justify-center h-full p-4 border rounded-lg">
      <h2 className="text-xl font-semibold mb-4">OTA Update</h2>
      <Button variant="outline" size="lg" className="h-24 w-24 flex flex-col items-center justify-center">
        <FolderIcon className="h-12 w-12 mb-2" />
        <span>Select Firmware</span>
      </Button>
    </div>
  );
}