"use client";

import { useForm } from "react-hook-form";
import { useEffect, useCallback } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Form, FormControl, FormField, FormItem, FormMessage } from "@/components/ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const formSchema = z.object({
    model: z.string().min(1, { message: "通道门型号不能为空" }),
});

export default function ChannelConfig({ deviceName }: { deviceName: string }) {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            model: "",
        },
    });

    const handleReadConfig = useCallback(async () => {
        try {
            await invoke("channel_readconfig");
        } catch (error) {
            toast.error("读取配置失败：" + error);
        }
    }, []);

    const handleRefactory = useCallback(async () => {
        try {
            await invoke("channel_refactory");
            toast.info("重置配置成功");
        } catch (error) {
            toast.error("重置配置失败：" + error);
        }
    }, []);

    useEffect(() => {
        // 监听阀门配置事件
        const unlisten = listen('channel_config', (event) => {
            const config = event.payload as { model: string; };
            form.reset({
                model: config.model,
            });
            toast.success("配置读取成功");
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, [form]);

    async function onSubmit(values: z.infer<typeof formSchema>) {
        const data = {
            ...values,
        };
        try {
            console.log(`${data}`)
            await invoke<string>("channel_configure", { config: data });
            toast.success("配置成功！");
        } catch (error: any) {
            toast.error("配置失败：" + error);
        }
    }

    return (
        <div className="p-4 border rounded-lg">
            <h2 className="text-xl font-semibold mb-4 text-center">通道门锁配置</h2>
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