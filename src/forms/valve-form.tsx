import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export default function ValveForm({ deviceName }: { deviceName: string }) {
  return (
    <div className="p-4 border rounded-lg">
      <h2 className="text-xl font-semibold mb-4">阀门锁配置</h2>
      <form className="space-y-4">
        <div>
          <label htmlFor="转动圈数阈值" className="block text-sm font-medium text-gray-700">
            转动圈数阈值
          </label>
          <Input type="text" id="count" placeholder="输入界定阀门开关的转动圈数" />
        </div>
        <div>
          <label htmlFor="型号" className="block text-sm font-medium text-gray-700">
            型号
          </label>
          <Input type="text" id="Model" placeholder="输入阀门型号" />
        </div>
        <div className="flex justify-end space-x-2">
          <Button type="submit">提交</Button>
          <Button type="button" variant="outline">取消</Button>
        </div>
      </form>
    </div>
  );
}