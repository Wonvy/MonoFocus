// 等待 Tauri API 加载完成
function waitForTauri() {
  return new Promise((resolve) => {
    if (window.__TAURI__) {
      resolve();
    } else {
      // 每 50ms 检查一次
      const checkInterval = setInterval(() => {
        if (window.__TAURI__) {
          clearInterval(checkInterval);
          resolve();
        }
      }, 50);
    }
  });
}

// 等待 DOM 和 Tauri API 加载完成
window.addEventListener("DOMContentLoaded", async () => {
  console.log("DOM 加载完成，等待 Tauri API...");

  // 等待 Tauri API
  await waitForTauri();
  console.log("Tauri API 已就绪！");

  const { invoke } = window.__TAURI__.tauri;
  const { listen } = window.__TAURI__.event;

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
  const animationSelect = document.getElementById("animationSelect");
  const animationValue = document.getElementById("animationValue");
  const languageSelect = document.getElementById("languageSelect");

  // 初始化应用
  async function init() {
    try {
      // 加载配置
      const config = await invoke("get_config");
      console.log("加载的配置:", config);

      // 先应用语言设置（确保翻译可用）
      const lang = config.language || "zh";
      window.i18n.currentLang = lang;
      window.i18n.applyTranslations(lang);
      languageSelect.value = lang;

      // 然后设置其他配置
      opacitySlider.value = config.opacity * 100;
      opacityValue.textContent = `${Math.round(config.opacity * 100)}%`;
      enabledToggle.checked = config.enabled;
      autoStartToggle.checked = config.auto_start;

      // 设置动画选择器的值
      const animDuration = config.animation_duration || 0;
      animationSelect.value = animDuration.toString();
      updateAnimationText(animDuration);
      console.log(
        "动画时长设置为:",
        animDuration,
        "选择器值:",
        animationSelect.value
      );

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
      console.log("开始加载显示器信息...");
      monitors = await invoke("get_monitor_info");
      console.log("显示器信息:", monitors);

      // 使用固定的逻辑尺寸，保持显示器比例不变
      const FIXED_WIDTH = 400;
      const FIXED_HEIGHT = 160;

      uiRects = await invoke("get_monitor_layout", {
        containerWidth: FIXED_WIDTH,
        containerHeight: FIXED_HEIGHT,
      });
      console.log("UI布局:", uiRects);

      currentMonitorId = await invoke("get_current_monitor");
      console.log("当前显示器ID:", currentMonitorId);

      if (monitors.length === 0) {
        monitorStatus.textContent = window.i18n.t("noMonitors");
      } else {
        monitorStatus.textContent = `${window.i18n.t("detected")} ${monitors.length} ${window.i18n.t("monitors")}`;
      }

      drawMonitors();
    } catch (error) {
      console.error("加载显示器信息失败:", error);
      monitorStatus.textContent = `${window.i18n.t("detectFailed")}: ${error}`;
    }
  }

  // 更新动画文本
  function updateAnimationText(duration) {
    const animationTexts = {
      0: window.i18n.t("animNone"),
      200: window.i18n.t("animFast"),
      300: window.i18n.t("animMedium"),
      500: window.i18n.t("animSlow"),
    };
    animationValue.textContent =
      animationTexts[duration] || window.i18n.t("animMedium");
  }

  // 绘制显示器布局
  function drawMonitors() {
    // 获取 Canvas 的实际尺寸
    const canvasWidth = canvas.width;
    const canvasHeight = canvas.height;

    // 清空画布
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    if (uiRects.length === 0) {
      ctx.fillStyle = "#666";
      ctx.font = "14px sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("未检测到显示器", canvasWidth / 2, canvasHeight / 2);
      return;
    }

    // 计算所有显示器的边界框（基于固定逻辑尺寸）
    let minX = Infinity,
      minY = Infinity,
      maxX = -Infinity,
      maxY = -Infinity;
    uiRects.forEach((rect) => {
      minX = Math.min(minX, rect.x);
      minY = Math.min(minY, rect.y);
      maxX = Math.max(maxX, rect.x + rect.width);
      maxY = Math.max(maxY, rect.y + rect.height);
    });

    const contentWidth = maxX - minX;
    const contentHeight = maxY - minY;

    // 计算缩放比例和居中偏移（保持纵横比）
    const scaleX = (canvasWidth - 40) / contentWidth;
    const scaleY = (canvasHeight - 40) / contentHeight;
    const scale = Math.min(scaleX, scaleY);

    const offsetX = (canvasWidth - contentWidth * scale) / 2 - minX * scale;
    const offsetY = (canvasHeight - contentHeight * scale) / 2 - minY * scale;

    // 绘制每个显示器
    uiRects.forEach((rect, index) => {
      const isActive = rect.id === currentMonitorId;

      const x = rect.x * scale + offsetX;
      const y = rect.y * scale + offsetY;
      const w = rect.width * scale;
      const h = rect.height * scale;

      // 绘制矩形（极简黑白风格）
      ctx.fillStyle = isActive ? "#000000" : "#f5f5f5";
      ctx.fillRect(x, y, w, h);

      // 绘制边框
      ctx.strokeStyle = isActive ? "#000000" : "#d0d0d0";
      ctx.lineWidth = isActive ? 2 : 1;
      ctx.strokeRect(x, y, w, h);

      // 计算字体大小
      const numberSize = Math.max(18, Math.min(w, h) * 0.35);
      const resolutionSize = Math.max(9, Math.min(w, h) * 0.12);

      // 绘制显示器编号
      ctx.fillStyle = isActive ? "#ffffff" : "#999999";
      ctx.font = `${numberSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`;
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(
        (index + 1).toString(),
        x + w / 2,
        y + h / 2 - numberSize * 0.3
      );

      // 绘制分辨率（小字）
      ctx.fillStyle = isActive ? "#cccccc" : "#bbbbbb";
      ctx.font = `${resolutionSize}px -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`;
      ctx.fillText(
        `${rect.resolution_width} × ${rect.resolution_height}`,
        x + w / 2,
        y + h / 2 + numberSize * 0.5
      );
    });
  }

  // 绑定事件
  function bindEvents() {
    // 透明度滑条 - 实时更新显示
    opacitySlider.addEventListener("input", (e) => {
      const value = parseInt(e.target.value);
      opacityValue.textContent = `${value}%`;
      console.log("透明度滑动:", value);
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

    // 动画速度选择
    animationSelect.addEventListener("change", async (e) => {
      const duration = parseInt(e.target.value);
      updateAnimationText(duration);
      try {
        await invoke("update_animation_duration", { duration });
      } catch (error) {
        console.error("更新动画时长失败:", error);
      }
    });

    // 语言切换
    languageSelect.addEventListener("change", async (e) => {
      const lang = e.target.value;
      window.i18n.currentLang = lang;
      window.i18n.applyTranslations(lang);
      try {
        await invoke("update_language", { language: lang });
        await loadMonitors(); // 重新加载以更新文本
      } catch (error) {
        console.error("更新语言失败:", error);
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
});
