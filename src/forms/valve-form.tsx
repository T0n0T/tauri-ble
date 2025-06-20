import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { Form, FormControl, FormField, FormItem, FormMessage } from "@/components/ui/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import * as z from "zod";
import { toast } from "sonner";

const formSchema = z.object({
  model: z.string().min(1, { message: "阀门型号不能为空" }),
  count: z.string().regex(/^\d+$/, { message: "圈数阈值必须是数字" }).min(1, { message: "圈数阈值不能为空" }),
  dir: z.boolean(),
});

export default function ValveForm({ deviceName }: { deviceName: string }) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      model: "",
      count: "",
      dir: false,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    const data = {
      ...values,
      count: Number(values.count), // 将 count 转换为数字
    };
    console.log("提交的表单数据:", data);
    try {
      await invoke<string>("submit_valve_form", { formData: data });
      toast.success("配置成功！");
    } catch (error: any) {
      toast.error("配置失败：" + error);
    }
  }

  return (
    <div className="p-4 border rounded-lg">
      <h2 className="text-xl font-semibold mb-4 text-center">阀门锁配置</h2>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col space-y-4">
          <FormField
            control={form.control}
            name="model"
            render={({ field }) => (
              <FormItem>
                <div className="flex gap-4 items-center">
                  <label htmlFor="model" className="w-20 text-sm text-right flex-shrink-0">阀门型号</label>
                  <FormControl>
                    <Input placeholder="输入阀门型号" {...field} />
                  </FormControl>
                </div>
                <FormMessage className="ml-24" />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="count"
            render={({ field }) => (
              <FormItem>
                <div className="flex gap-4 items-center">
                  <label htmlFor="count" className="w-20 text-sm text-right flex-shrink-0">圈数阈值</label>
                  <FormControl>
                    <Input type="text" placeholder="输入界定阀门开闭的转动圈数" {...field} />
                  </FormControl>
                </div>
                <FormMessage className="ml-24" />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="dir"
            render={({ field }) => (
              <FormItem className="flex gap-4 items-center">
                <label htmlFor="dir" className="w-20 text-sm text-right flex-shrink-0">逆时针旋转</label>
                <Switch
                  checked={field.value}
                  onCheckedChange={field.onChange}
                />
              </FormItem>
            )}
          />
          <div className="flex justify-end space-x-2">
            <Button type="submit">提交</Button>
            <Button type="button" variant="outline" onClick={() => form.reset()}>重置</Button>
          </div>
        </form>
      </Form>
    </div>
  );
}
