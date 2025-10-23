"use client";

import { useForm } from "react-hook-form";
import { useCallback } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Form, FormControl, FormField, FormItem, FormMessage } from "@/components/ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";

const formSchema = z.object({
  model: z.string().min(1, { message: "气压检测装置型号不能为空" }),
  pressure: z.string().regex(/^\d+$/, { message: "气压阈值必须是数字" }).min(1, { message: "气压阈值不能为空" }),
});

export default function AirPressureConfig({ deviceName }: { deviceName: string }) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      model: "",
    },
  });

  const handleReadConfig = useCallback(async () => {
    try {
        await invoke("airpressure_readconfig");
    } catch (error) {
        toast.error("读取配置失败：" + error);
    }
  }, []);

  const handleRefactory = useCallback(async () => {
    try {
        await invoke("airpressure_refactory");
        toast.info("重置配置成功");
    } catch (error) {
        toast.error("重置配置失败：" + error);
    }
  }, []);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    const data = {
      ...values,
    };
    try {
      console.log(`${data}`)
      await invoke<string>("airpressure_configure", { config: data });
      toast.success("配置成功！");
    } catch (error: any) {
      toast.error("配置失败：" + error);
    }
  }

  return (
    <div className="p-4 border rounded-lg">
      <h2 className="text-xl font-semibold mb-4 text-center">气压检测装置配置</h2>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col space-y-4">
          <FormField
            control={form.control}
            name="model"
            render={({ field }) => (
              <FormItem>
                <div className="flex gap-4 items-center">
                  <label htmlFor="model" className="w-20 text-sm text-right flex-shrink-0">型号</label>
                  <FormControl>
                    <Input placeholder="输入装置型号" {...field} />
                  </FormControl>
                </div>
                <FormMessage className="ml-24" />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="pressure"
            render={({ field }) => (
              <FormItem>
                <div className="flex gap-4 items-center">
                  <label htmlFor="tick" className="w-20 text-sm text-right flex-shrink-0">气压阈值</label>
                  <FormControl>
                    <Input placeholder="输入气压阈值(单位：MPa)" {...field} />
                  </FormControl>
                </div>
                <FormMessage className="ml-24" />
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