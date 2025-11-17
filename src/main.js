import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

// 全局状态
let monitors = [];
let currentMonitorId = null;
let uiRects = [];

// DOM 元素
const canvas = document.getElementById("monitorCanvas");
const ctx = canvas.getContext("2d");
const monitorStatus = document.getElementById("monitorStatus");
const opacitySlider = document.getElementById("opacitySlider");
const opacityValue = document.getElementById("opacityValue");
const enabledToggle = document.getElementById("enabledToggle");
const autoStartToggle = document.getElementById("autoStartToggle");

// 初始化应用
async function init() {
  try {
    // 加载配置
    const config = await invoke("get_config");
    opacitySlider.value = config.opacity * 100;
    opacityValue.textContent = `${Math.round(config.opacity * 100)}%`;
    enabledToggle.checked = config.enabled;
    autoStartToggle.checked = config.auto_start;

    // 加载显示器信息
    await loadMonitors();

    // 绑定事件
    bindEvents();

    // 监听后端事件
    listenToBackend();

    // 定时刷新显示器布局
    setInterval(loadMonitors, 5000);
  } catch (error) {
    console.error("初始化失败:", error);
    monitorStatus.textContent = "初始化失败";
  }
}

// 加载显示器信息
async function loadMonitors() {
  try {
    monitors = await invoke("get_monitor_info");
    uiRects = await invoke("get_monitor_layout", {
      containerWidth: canvas.width,
      containerHeight: canvas.height,
    });
    currentMonitorId = await invoke("get_current_monitor");

    if (monitors.length === 0) {
      monitorStatus.textContent = "未检测到显示器";
    } else {
      monitorStatus.textContent = `已检测到 ${monitors.length} 台显示器`;
    }

    drawMonitors();
  } catch (error) {
    console.error("加载显示器信息失败:", error);
    monitorStatus.textContent = "检测失败";
  }
}

// 绘制显示器布局
function drawMonitors() {
  // 清空画布
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (uiRects.length === 0) {
    ctx.fillStyle = "#666";
    ctx.font = "14px sans-serif";
    ctx.textAlign = "center";
    ctx.fillText("未检测到显示器", canvas.width / 2, canvas.height / 2);
    return;
  }

  // 绘制每个显示器
  uiRects.forEach((rect) => {
    const isActive = rect.id === currentMonitorId;

    // 绘制矩形
    ctx.fillStyle = isActive ? "#667eea" : "#e0e0e0";
    ctx.fillRect(rect.x, rect.y, rect.width, rect.height);

    // 绘制边框
    ctx.strokeStyle = isActive ? "#764ba2" : "#999";
    ctx.lineWidth = isActive ? 3 : 2;
    ctx.strokeRect(rect.x, rect.y, rect.width, rect.height);

    // 如果是活跃显示器，添加高亮效果
    if (isActive) {
      ctx.shadowColor = "rgba(102, 126, 234, 0.5)";
      ctx.shadowBlur = 10;
      ctx.strokeRect(rect.x, rect.y, rect.width, rect.height);
      ctx.shadowBlur = 0;
    }
  });
}

// 绑定事件
function bindEvents() {
  // 透明度滑条
  opacitySlider.addEventListener("input", (e) => {
    const value = parseInt(e.target.value);
    opacityValue.textContent = `${value}%`;
  });

  opacitySlider.addEventListener("change", async (e) => {
    const value = parseInt(e.target.value) / 100;
    try {
      await invoke("update_opacity", { opacity: value });
    } catch (error) {
      console.error("更新透明度失败:", error);
    }
  });

  // 护眼模式开关
  enabledToggle.addEventListener("change", async (e) => {
    try {
      await invoke("update_enabled", { enabled: e.target.checked });
    } catch (error) {
      console.error("更新启用状态失败:", error);
      e.target.checked = !e.target.checked; // 回滚
    }
  });

  // 开机自启动
  autoStartToggle.addEventListener("change", async (e) => {
    try {
      await invoke("update_auto_start", { autoStart: e.target.checked });
    } catch (error) {
      console.error("更新自启动失败:", error);
      e.target.checked = !e.target.checked; // 回滚
    }
  });
}

// 监听后端事件
function listenToBackend() {
  // 监听显示器变化
  listen("monitor-changed", (event) => {
    currentMonitorId = event.payload;
    drawMonitors();
  });

  // 监听切换护眼模式（从托盘触发）
  listen("toggle-shield", async () => {
    enabledToggle.checked = !enabledToggle.checked;
    try {
      await invoke("update_enabled", { enabled: enabledToggle.checked });
    } catch (error) {
      console.error("切换护眼模式失败:", error);
      enabledToggle.checked = !enabledToggle.checked;
    }
  });
}

// 启动应用
init();

