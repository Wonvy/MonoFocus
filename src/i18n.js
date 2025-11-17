// 多语言翻译
const translations = {
  zh: {
    eyeCareMode: "护眼模式",
    opacity: "遮罩透明度",
    animation: "切换动画",
    autoStart: "开机自启动",
    language: "语言",
    footerInfo: "当鼠标移至某一显示器时，其他显示器将自动显示遮罩",
    detecting: "检测中...",
    detected: "已检测到",
    monitors: "台显示器",
    noMonitors: "未检测到显示器",
    detectFailed: "检测失败",
    animNone: "无",
    animFast: "快",
    animMedium: "中",
    animSlow: "慢",
  },
  en: {
    eyeCareMode: "Eye Care Mode",
    opacity: "Mask Opacity",
    animation: "Animation Speed",
    autoStart: "Auto Start",
    language: "Language",
    footerInfo:
      "When mouse moves to a monitor, other monitors will show overlay",
    detecting: "Detecting...",
    detected: "Detected",
    monitors: "monitors",
    noMonitors: "No monitors detected",
    detectFailed: "Detection failed",
    animNone: "None",
    animFast: "Fast",
    animMedium: "Medium",
    animSlow: "Slow",
  },
  ja: {
    eyeCareMode: "アイケアモード",
    opacity: "マスク透明度",
    animation: "アニメーション速度",
    autoStart: "自動起動",
    language: "言語",
    footerInfo:
      "マウスがモニターに移動すると、他のモニターにマスクが表示されます",
    detecting: "検出中...",
    detected: "検出しました",
    monitors: "台のモニター",
    noMonitors: "モニターが検出されません",
    detectFailed: "検出失敗",
    animNone: "なし",
    animFast: "速い",
    animMedium: "中",
    animSlow: "遅い",
  },
  fr: {
    eyeCareMode: "Mode Protection des Yeux",
    opacity: "Opacité du Masque",
    animation: "Vitesse d'Animation",
    autoStart: "Démarrage Auto",
    language: "Langue",
    footerInfo:
      "Lorsque la souris se déplace vers un moniteur, les autres moniteurs affichent un masque",
    detecting: "Détection...",
    detected: "Détecté",
    monitors: "moniteurs",
    noMonitors: "Aucun moniteur détecté",
    detectFailed: "Échec de la détection",
    animNone: "Aucune",
    animFast: "Rapide",
    animMedium: "Moyen",
    animSlow: "Lent",
  },
  de: {
    eyeCareMode: "Augenschutzmodus",
    opacity: "Maskendeckkraft",
    animation: "Animationsgeschwindigkeit",
    autoStart: "Autostart",
    language: "Sprache",
    footerInfo:
      "Wenn die Maus zu einem Monitor bewegt wird, zeigen andere Monitore eine Maske an",
    detecting: "Erkennung...",
    detected: "Erkannt",
    monitors: "Monitore",
    noMonitors: "Keine Monitore erkannt",
    detectFailed: "Erkennung fehlgeschlagen",
    animNone: "Keine",
    animFast: "Schnell",
    animMedium: "Mittel",
    animSlow: "Langsam",
  },
  es: {
    eyeCareMode: "Modo Cuidado de Ojos",
    opacity: "Opacidad de Máscara",
    animation: "Velocidad de Animación",
    autoStart: "Inicio Automático",
    language: "Idioma",
    footerInfo:
      "Cuando el mouse se mueve a un monitor, otros monitores mostrarán una máscara",
    detecting: "Detectando...",
    detected: "Detectado",
    monitors: "monitores",
    noMonitors: "No se detectaron monitores",
    detectFailed: "Detección fallida",
    animNone: "Ninguna",
    animFast: "Rápida",
    animMedium: "Media",
    animSlow: "Lenta",
  },
};

// 应用翻译
function applyTranslations(lang) {
  const t = translations[lang] || translations["zh"];

  // 更新所有带 data-i18n 属性的元素
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    const key = el.getAttribute("data-i18n");
    if (t[key]) {
      el.textContent = t[key];
    }
  });

  return t;
}

// 导出
window.i18n = {
  translations,
  applyTranslations,
  currentLang: "zh",
  t: function (key) {
    return translations[this.currentLang][key] || key;
  },
};
