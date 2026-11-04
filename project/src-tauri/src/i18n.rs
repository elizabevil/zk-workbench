//! 后端菜单文案。与前端语言包对应，但不依赖前端。

use zkui_core::config::Locale;

pub struct MenuLabels {
    pub app: &'static str,
    pub edit: &'static str,
    pub settings: &'static str,
    pub about: &'static str,
    pub close: &'static str,
}

const ZH: MenuLabels = MenuLabels {
    app: "应用",
    edit: "编辑",
    settings: "设置",
    about: "关于 {name}",
    close: "关闭窗口",
};

const EN: MenuLabels = MenuLabels {
    app: "App",
    edit: "Edit",
    settings: "Settings",
    about: "About {name}",
    close: "Close Window",
};

pub fn labels(locale: Locale) -> &'static MenuLabels {
    match locale {
        Locale::ZhCn => &ZH,
        Locale::EnUs => &EN,
    }
}
