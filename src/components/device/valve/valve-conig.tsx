import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { Form, FormControl, FormField, FormItem, FormMessage } from "@/components/ui/form";
import { Hand } from "lucide-react";
import { useState, useEffect, useCallback } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import * as z from "zod";
import { toast } from "sonner";
import { ValveVal } from "@/types/valve";

const formSchema = z.object({
  model: z.string().min(1, { message: "阀门型号不能为空" }),
  tick: z.string().regex(/^\d+$/, { message: "圈数阈值必须是数字" }).min(1, { message: "圈数阈值不能为空" }),
  dir: z.boolean(),
});

export default function ValveConfig({ deviceName }: { deviceName: string }) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      model: "",
      tick: "",
      dir: false,
    },
  });

  const [valveInfo, setValveInfo] = useState<ValveVal>({
    total_ticks: 0,
    current_status: 0,
  });

  const [isTuningDisabled, setIsTuningDisabled] = useState(true);

  const valve_tuning = useCallback(() => {
    setIsTuningDisabled((prev) => {
      if (prev) {
        toast.info("开始标定")
        try {
          invoke("valve_tuning_start")
        } catch (error) {
          toast.error("标定开始失败")
        }
      } else {
        toast.info("valve_tuning_stop")
        try {
          invoke("valve_tuning_stop")
        } catch (error) {
          toast.error("标定停止失败")
        }
      }
      return !prev
    });
  }, []);

  const handleReadConfig = useCallback(async () => {
    try {
      await invoke("valve_readconfig");
    } catch (error) {
      toast.error("读取配置失败：" + error);
    }
  }, []);

  const handleRefactory = useCallback(async () => {
    try {
      await invoke("valve_refactory");
      toast.info("重置配置成功");
    } catch (error) {
      toast.error("重置配置失败：" + error);
    }
  }, []);

  useEffect(() => {
    // 监听事件
    const unlisten = listen('valve_tuning', (event) => {
      const data = event.payload as ValveVal;
      setValveInfo(data);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [isTuningDisabled]);

  useEffect(() => {
    // 监听阀门配置事件
    const unlisten = listen('valve_config', (event) => {
      const config = event.payload as { model: string; tick: number; dir: boolean };
      form.reset({
        model: config.model,
        tick: config.tick.toString(),
        dir: config.dir,
      });
      toast.success("配置读取成功");
    });

    return () => {
      unlisten.then((f) => f());
      console.log(`clear readconfig`)
    };
  }, [form]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    const data = {
      ...values,
      tick: Number(values.tick), // 将 tick 转换为数字
    };
    try {
      console.log(`${data}`)
      await invoke<string>("valve_configure", { config: data });
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
            name="tick"
            render={({ field }) => (
              <FormItem>
                <div className="flex gap-4 items-center">
                  <label htmlFor="tick" className="w-20 text-sm text-right flex-shrink-0">圈数阈值</label>
                  <FormControl>
                    <div className="relative w-full">
                      <Input
                        type="text"
                        placeholder="输入界定阀门开闭的转动圈数"
                        className="pr-10"
                        {...field}
                        disabled={!isTuningDisabled}
                        value={!isTuningDisabled ? valveInfo.total_ticks : field.value}
                      />
                      <Button type="button" variant="ghost" size="icon" className="absolute right-0 top-1/2 -translate-y-1/2" onClick={valve_tuning}>
                        <Hand />
                      </Button>
                    </div>
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
                <label htmlFor="dir" className="w-20 text-sm text-right flex-shrink-0">顺时针开启</label>
                <Switch
                  checked={field.value}
                  onCheckedChange={field.onChange}
                />
              </FormItem>
            )}
          />
          <div className="flex justify-end space-x-2">
            <Button type="button" variant="outline" onClick={handleReadConfig}>读取</Button>
            <Button type="submit">提交</Button>
            <Button type="button" variant="outline" className="bg-red-400" onClick={handleRefactory}>恢复默认</Button>
          </div>
        </form>
      </Form>
    </div>
  );
}
