我现在打算使用tauri+nextjs做一个应用于Bluetooth上位机的应用，需要考虑布局的跨平台性，包括linux，window，Android
# 准备
- 使用我已经添加的shadcn ui组件
- 布局包括侧边栏，主页面
- 主页面包含header和main，主页面显示的内容依据router来渲染
- 布局需要适应跨平台，比如侧边栏对于Android应用来说，需要从左侧展开更多的宽度

# 规划复合组件
## 左侧边栏设备选择复合组件
- 使用sidebar作为基础
- 页面右下方添加扫描开始/暂停button，是一个可切换字样显示的按钮
- 开始扫描后可以获取周围的蓝牙从机设备信息，使用card展示可以连接的设备
- card包含了展示设备名称的label，mac地址的label，信号强度的label
- card包含了展示设备类型的图标
- card表示的设备如果被选择，需要将底色改变为更深的颜色
- card表示的设备连接成功时，自动收起复合组件

## 主页面header
- 放置弹出左侧边栏的按钮
- 显示当前选中的设备名称

## 主页面main
- 使用tabs包装两个功能，command，ota
- command子页面放置表单和提交取消按钮，表单先放置测试条目test1 test2即可
- ota子页面正中央拜访文件夹图标按钮

# 编辑内容
`src`为我的nextjs项目目录，先不用帮我处理tauri方面的事务，如果需要shadcn ui的其他组件，停下来告诉我