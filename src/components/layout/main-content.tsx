import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import CommandForm from "@/components/forms/command-form";
import OtaPage from "@/components/ota/ota-page";

export default function MainContent() {
  return (
    <main className="flex-grow p-4">
      <Tabs defaultValue="command" className="w-full h-full flex flex-col">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="command">Command</TabsTrigger>
          <TabsTrigger value="ota">OTA</TabsTrigger>
        </TabsList>
        <TabsContent value="command" className="flex-grow mt-4">
          <CommandForm />
        </TabsContent>
        <TabsContent value="ota" className="flex-grow mt-4">
          <OtaPage />
        </TabsContent>
      </Tabs>
    </main>
  );
}