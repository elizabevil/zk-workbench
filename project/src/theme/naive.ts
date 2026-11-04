//! Naive UI 主题覆盖：保持现有深色半透明外观。

import type {GlobalThemeOverrides} from "naive-ui";

export const naiveThemeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: "#2563eb",
    primaryColorHover: "#3b82f6",
    primaryColorPressed: "#1d4ed8",
    primaryColorSuppl: "#2563eb",
    bodyColor: "transparent",
    cardColor: "rgb(0 0 0 / 35%)",
    modalColor: "#171c22",
    popoverColor: "#171c22",
    textColorBase: "#e6e6e6",
    textColor1: "#e6e6e6",
    textColor2: "#cbd5e1",
    textColor3: "#9aa4b2",
    borderColor: "#2a323c",
    dividerColor: "rgb(255 255 255 / 8%)",
    borderRadius: "6px",
    fontSize: "13px",
  },
  Input: {
    color: "#101418",
    colorFocus: "#101418",
    colorDisabled: "rgb(16 20 24 / 60%)",
    textColor: "#e6e6e6",
    placeholderColor: "#6b7280",
    border: "1px solid #2a323c",
    borderHover: "1px solid #3b4654",
    borderFocus: "1px solid #2563eb",
    boxShadowFocus: "0 0 0 2px rgb(37 99 235 / 18%)",
    caretColor: "#e6e6e6",
  },
  Button: {
    color: "rgb(255 255 255 / 8%)",
    colorHover: "rgb(255 255 255 / 14%)",
    colorPressed: "rgb(255 255 255 / 18%)",
    textColor: "#e6e6e6",
    border: "1px solid rgb(255 255 255 / 15%)",
    borderHover: "1px solid rgb(255 255 255 / 24%)",
    borderPressed: "1px solid rgb(255 255 255 / 24%)",
  },
  Form: {
    labelTextColor: "#cbd5e1",
  },
  Card: {
    color: "rgb(0 0 0 / 35%)",
    borderColor: "#2a323c",
    borderRadius: "8px",
    paddingSmall: "12px 14px",
  },
  DataTable: {
    tdColor: "transparent",
    thColor: "rgb(255 255 255 / 4%)",
    tdTextColor: "#e6e6e6",
    thTextColor: "#9aa4b2",
    borderColor: "#2a323c",
  },
  Statistic: {
    labelTextColor: "#9aa4b2",
    valueTextColor: "#e6e6e6",
  },
  Checkbox: {
    colorChecked: "#2563eb",
    colorDisabledChecked: "rgb(37 99 235 / 40%)",
    borderChecked: "1px solid #2563eb",
    checkMarkColor: "#fff",
  },
  Slider: {
    fillColor: "#2563eb",
    fillColorHover: "#3b82f6",
    handleColor: "#2563eb",
  },
};
